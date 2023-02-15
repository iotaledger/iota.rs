// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use iota_types::block::output::OutputId;

use super::{InputSelection, Requirement};
use crate::{
    block::{
        address::Address,
        output::{
            unlock_condition::{StorageDepositReturnUnlockCondition, UnlockCondition, UnlockConditions},
            AliasOutputBuilder, AliasTransition, FoundryOutputBuilder, NftOutputBuilder, Output, Rent,
        },
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

// TODO checked operations ?

/// Get the `StorageDepositReturnUnlockCondition`, if not expired.
pub(crate) fn sdruc_not_expired(output: &Output, current_time: u32) -> Option<&StorageDepositReturnUnlockCondition> {
    // PANIC: safe to unwrap as outputs without unlock conditions have been filtered out already.
    let unlock_conditions = output.unlock_conditions().unwrap();

    unlock_conditions.storage_deposit_return().and_then(|sdr| {
        let expired = unlock_conditions
            .expiration()
            .map_or(false, |expiration| current_time >= expiration.timestamp());

        // We only have to send the storage deposit return back if the output is not expired
        if !expired {
            Some(sdr)
        } else {
            None
        }
    })
}

pub(crate) fn amount_sums(
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
    timestamp: u32,
) -> (u64, u64, HashMap<Address, u64>, HashMap<Address, u64>) {
    let mut inputs_sum = 0;
    let mut outputs_sum = 0;
    let mut inputs_sdr = HashMap::new();
    let mut outputs_sdr = HashMap::new();

    for selected_input in selected_inputs {
        inputs_sum += selected_input.output.amount();

        if let Some(sdruc) = sdruc_not_expired(&selected_input.output, timestamp) {
            *inputs_sdr.entry(*sdruc.return_address()).or_default() += sdruc.amount();
        }
    }

    for output in outputs {
        outputs_sum += output.amount();

        if let Output::Basic(output) = output {
            if let Some(address) = output.simple_deposit_address() {
                *outputs_sdr.entry(*address).or_default() += output.amount();
            }
        }
    }

    // TODO explanation about that
    for (sdr_address, input_sdr_amount) in &inputs_sdr {
        let output_sdr_amount = outputs_sdr.get(sdr_address).unwrap_or(&0);

        if input_sdr_amount > output_sdr_amount {
            outputs_sum += input_sdr_amount - output_sdr_amount;
        }
    }

    (inputs_sum, outputs_sum, inputs_sdr, outputs_sdr)
}

struct AmountSelection {
    newly_selected_ids: HashSet<OutputId>,
    newly_selected_inputs: Vec<(InputSigningData, Option<AliasTransition>)>,
    inputs_sum: u64,
    outputs_sum: u64,
    inputs_sdr: HashMap<Address, u64>,
    outputs_sdr: HashMap<Address, u64>,
    remainder_amount: u64,
    native_tokens_remainder: bool,
}

impl AmountSelection {
    fn new(input_selection: &InputSelection) -> Result<Self> {
        let (inputs_sum, outputs_sum, inputs_sdr, outputs_sdr) = amount_sums(
            &input_selection.selected_inputs,
            &input_selection.outputs,
            input_selection.timestamp,
        );
        let (remainder_amount, native_tokens_remainder) = input_selection.remainder_amount()?;

        Ok(Self {
            newly_selected_ids: HashSet::new(),
            newly_selected_inputs: Vec::new(),
            inputs_sum,
            outputs_sum,
            inputs_sdr,
            outputs_sdr,
            remainder_amount,
            native_tokens_remainder,
        })
    }

    fn missing_amount(&self) -> u64 {
        // If there is already a remainder, make sure it's enough to cover the storage deposit.
        if self.inputs_sum > self.outputs_sum {
            let diff = self.inputs_sum - self.outputs_sum;

            if self.remainder_amount > diff {
                self.remainder_amount - diff
            } else {
                0
            }
        } else if self.inputs_sum < self.outputs_sum {
            self.outputs_sum - self.inputs_sum
        } else if self.native_tokens_remainder {
            self.remainder_amount
        } else {
            0
        }
    }

    fn plop<'a>(&mut self, inputs: impl Iterator<Item = &'a InputSigningData>) -> bool {
        for input in inputs {
            if self.newly_selected_ids.contains(input.output_id()) {
                continue;
            }

            self.inputs_sum += input.output.amount();
            self.newly_selected_inputs.push((input.clone(), None));
            self.newly_selected_ids.insert(*input.output_id());

            if self.missing_amount() == 0 {
                return true;
            }
        }

        false
    }
}

impl InputSelection {
    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn fulfill_amount_requirement(&mut self) -> Result<Vec<(InputSigningData, Option<AliasTransition>)>> {
        let mut amount_selection = AmountSelection::new(self)?;

        if amount_selection.missing_amount() == 0 {
            log::debug!("Amount requirement already fulfilled");
            return Ok(amount_selection.newly_selected_inputs);
        } else {
            log::debug!(
                "Fulfilling amount requirement with input {}, output {}, input sdrs {:?} and output sdrs {:?}",
                amount_selection.inputs_sum,
                amount_selection.outputs_sum,
                amount_selection.inputs_sdr,
                amount_selection.outputs_sdr
            );
        }

        // TODO if consolidate strategy: sum all the lowest amount until diff is covered.
        // TODO this would be lowest amount of input strategy.
        self.available_inputs
            .sort_by(|left, right| left.output.amount().cmp(&right.output.amount()));

        'overall: {
            // 0. Basic with ED25519 address without SDRUC or expired SDRUC, without NTs, without unexpired expiration
            {
                log::debug!(
                    "Trying basic outputs with ed25519 address and no or expired SDRUC, without NTs, without unexpired expiration"
                );

                let inputs = self.available_inputs.iter().filter(|input| {
                    if let Output::Basic(output) = &input.output {
                        output
                            .unlock_conditions()
                            .locked_address(output.address(), self.timestamp)
                            .is_ed25519()
                            && sdruc_not_expired(&input.output, self.timestamp).is_none()
                            && output.native_tokens().is_empty()
                    } else {
                        false
                    }
                });

                if amount_selection.plop(inputs) {
                    break 'overall;
                }
            }

            // 1. Basic with ED25519 address without SDRUC or expired SDRUC
            {
                log::debug!(
                    "Trying basic outputs with ed25519 address and no or expired SDRUC, with NTs, without unexpired expiration"
                );

                let inputs = self.available_inputs.iter().filter(|input| {
                    if let Output::Basic(output) = &input.output {
                        output
                            .unlock_conditions()
                            .locked_address(output.address(), self.timestamp)
                            .is_ed25519()
                            && sdruc_not_expired(&input.output, self.timestamp).is_none()
                            && !output.native_tokens().is_empty()
                    } else {
                        false
                    }
                });

                for input in inputs {
                    if amount_selection.newly_selected_ids.contains(input.output_id()) {
                        continue;
                    }

                    log::debug!("1. adding {}", input.output_id());
                    amount_selection.inputs_sum += input.output.amount();
                    amount_selection.newly_selected_inputs.push((input.clone(), None));
                    amount_selection.newly_selected_ids.insert(*input.output_id());

                    if amount_selection.missing_amount() == 0 {
                        break 'overall;
                    }
                }
            }

            // 2. Basic with ED25519 address and unexpired SDRUC
            {
                log::debug!("Trying basic outputs with ed25519 address and unexpired SDRUC");

                let inputs = self.available_inputs.iter().filter(|input| {
                    if let Output::Basic(output) = &input.output {
                        if output.address().is_ed25519() {
                            // Filter out outputs that have to send back their full amount as they contribute to
                            // nothing.
                            output
                                .unlock_conditions()
                                .storage_deposit_return()
                                .map_or(false, |sdr| sdr.amount() != input.output.amount())
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

                for input in inputs {
                    if amount_selection.newly_selected_ids.contains(input.output_id()) {
                        continue;
                    }

                    log::debug!("2. adding {}", input.output_id());
                    let sdruc = input
                        .output
                        .unlock_conditions()
                        .and_then(UnlockConditions::storage_deposit_return)
                        // PANIC: safe to unwrap as the filter guarantees outputs with SDRUC only.
                        .unwrap();

                    // Skip if no additional amount is made available
                    if input.output.amount() == sdruc.amount() {
                        continue;
                    }

                    amount_selection.inputs_sum += input.output.amount();
                    amount_selection.newly_selected_inputs.push((input.clone(), None));
                    amount_selection.newly_selected_ids.insert(*input.output_id());

                    let input_sdr =
                        amount_selection.inputs_sdr.get(sdruc.return_address()).unwrap_or(&0) + sdruc.amount();
                    let output_sdr = *amount_selection.outputs_sdr.get(sdruc.return_address()).unwrap_or(&0);

                    if input_sdr > output_sdr {
                        let diff = input_sdr - output_sdr;
                        amount_selection.outputs_sum += diff;
                        *amount_selection.outputs_sdr.entry(*sdruc.return_address()).or_default() += sdruc.amount();
                    }

                    *amount_selection.inputs_sdr.entry(*sdruc.return_address()).or_default() += sdruc.amount();

                    if amount_selection.missing_amount() == 0 {
                        break 'overall;
                    }
                }
            }

            // 3. Basic with other kind of address
            {
                log::debug!("Trying basic outputs with other types of address");

                let inputs = self.available_inputs.iter().filter(|input| {
                    if let Output::Basic(output) = &input.output {
                        if let [UnlockCondition::Address(address)] = output.unlock_conditions().as_ref() {
                            !address.address().is_ed25519()
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

                for input in inputs {
                    if amount_selection.newly_selected_ids.contains(input.output_id()) {
                        continue;
                    }

                    log::debug!("3. adding {}", input.output_id());
                    amount_selection.inputs_sum += input.output.amount();
                    amount_selection.newly_selected_inputs.push((input.clone(), None));
                    amount_selection.newly_selected_ids.insert(*input.output_id());

                    if amount_selection.inputs_sum >= amount_selection.outputs_sum + amount_selection.remainder_amount {
                        break 'overall;
                    }
                }
            }

            // 4. Basic with other kind of address and multiple unlock conditions
            {
                log::debug!("Trying basic outputs with other types of address and multiple unlock conditions");

                let inputs = self.available_inputs.iter().filter(|input| {
                    if let Output::Basic(output) = &input.output {
                        !output
                            .unlock_conditions()
                            .locked_address(output.address(), self.timestamp)
                            .is_ed25519()
                            && output.unlock_conditions().len() > 1
                    } else {
                        false
                    }
                });

                for input in inputs {
                    if amount_selection.newly_selected_ids.contains(input.output_id()) {
                        continue;
                    }

                    log::debug!("4. adding {}", input.output_id());
                    match input
                        .output
                        .unlock_conditions()
                        .and_then(UnlockConditions::storage_deposit_return)
                    {
                        Some(sdruc) => {
                            // Skip if no additional amount is made available
                            if input.output.amount() == sdruc.amount() {
                                continue;
                            }

                            amount_selection.inputs_sum += input.output.amount();
                            amount_selection.newly_selected_inputs.push((input.clone(), None));
                            amount_selection.newly_selected_ids.insert(*input.output_id());

                            let input_sdr =
                                amount_selection.inputs_sdr.get(sdruc.return_address()).unwrap_or(&0) + sdruc.amount();
                            let output_sdr = *amount_selection.outputs_sdr.get(sdruc.return_address()).unwrap_or(&0);

                            if input_sdr > output_sdr {
                                let diff = input_sdr - output_sdr;
                                amount_selection.outputs_sum += diff;
                                *amount_selection.outputs_sdr.entry(*sdruc.return_address()).or_default() +=
                                    sdruc.amount();
                            }

                            *amount_selection.inputs_sdr.entry(*sdruc.return_address()).or_default() += sdruc.amount();

                            if amount_selection.missing_amount() == 0 {
                                break 'overall;
                            }
                        }
                        None => {
                            amount_selection.inputs_sum += input.output.amount();
                            amount_selection.newly_selected_inputs.push((input.clone(), None));
                            amount_selection.newly_selected_ids.insert(*input.output_id());

                            if amount_selection.inputs_sum
                                >= amount_selection.outputs_sum + amount_selection.remainder_amount
                            {
                                break 'overall;
                            }
                        }
                    }
                }
            }

            // 5. Other kinds of outputs
            {
                log::debug!("Trying other types of outputs");

                let mut inputs = self
                    .available_inputs
                    .iter()
                    .filter(|input| !input.output.is_basic())
                    .peekable();

                if inputs.peek().is_some() {
                    for input in inputs {
                        if amount_selection.newly_selected_ids.contains(input.output_id()) {
                            continue;
                        }

                        log::debug!("5. adding {}", input.output_id());
                        amount_selection.inputs_sum += input.output.amount();
                        amount_selection.newly_selected_inputs.push((input.clone(), None));
                        amount_selection.newly_selected_ids.insert(*input.output_id());

                        if amount_selection.inputs_sum
                            >= amount_selection.outputs_sum + amount_selection.remainder_amount
                        {
                            break;
                        }
                    }

                    log::debug!(
                        "Outputs {:?} selected to fulfill the amount requirement",
                        amount_selection.newly_selected_ids
                    );
                    log::debug!("Triggering another amount round as non-basic outputs need to be transitioned first");

                    self.available_inputs
                        .retain(|input| !amount_selection.newly_selected_ids.contains(input.output_id()));

                    // TODO explanation of Amount
                    self.requirements.push(Requirement::Amount);

                    return Ok(amount_selection.newly_selected_inputs);
                }
            }
        }

        'ici: {
            if amount_selection.missing_amount() != 0 {
                // Moving funds of already transitioned other outputs ?
                let outputs = self.outputs.iter_mut().filter(|output| {
                    output
                        .chain_id()
                        .as_ref()
                        .map(|chain_id| self.automatically_transitioned.contains(chain_id))
                        .unwrap_or(false)
                });

                for output in outputs {
                    let diff = amount_selection.missing_amount();
                    let amount = output.amount();
                    let rent = output.rent_cost(self.protocol_parameters.rent_structure());

                    let new_amount = if amount >= diff + rent { amount - diff } else { rent };

                    // TODO check that new_amount is enough for the rent

                    // PANIC: unwrap is fine as non-chain outputs have been filtered out already.
                    log::debug!(
                        "Reducing amount of {} to {} to fulfill amount requirement",
                        output.chain_id().unwrap(),
                        new_amount
                    );

                    let new_output = match output {
                        Output::Alias(output) => AliasOutputBuilder::from(&*output)
                            .with_amount(new_amount)?
                            .finish_output(self.protocol_parameters.token_supply())?,
                        Output::Nft(output) => NftOutputBuilder::from(&*output)
                            .with_amount(new_amount)?
                            .finish_output(self.protocol_parameters.token_supply())?,
                        Output::Foundry(output) => FoundryOutputBuilder::from(&*output)
                            .with_amount(new_amount)?
                            .finish_output(self.protocol_parameters.token_supply())?,
                        _ => panic!("only alias, nft and foundry can be automatically created"),
                    };

                    amount_selection.outputs_sum -= amount - new_amount;
                    *output = new_output;

                    if amount_selection.missing_amount() == 0 {
                        break 'ici;
                    }
                }

                return Err(Error::InsufficientAmount {
                    found: amount_selection.inputs_sum,
                    required: amount_selection.inputs_sum + amount_selection.missing_amount(),
                });
            }
        }

        log::debug!(
            "Outputs {:?} selected to fulfill the amount requirement",
            amount_selection.newly_selected_ids
        );

        self.available_inputs
            .retain(|input| !amount_selection.newly_selected_ids.contains(input.output_id()));

        Ok(amount_selection.newly_selected_inputs)
    }
}
