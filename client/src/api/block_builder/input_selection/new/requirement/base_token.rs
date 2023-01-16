// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use super::{InputSelection, OutputInfo, Requirement};
use crate::{
    block::{
        address::Address,
        output::{
            unlock_condition::UnlockCondition, AliasOutputBuilder, FoundryOutputBuilder, NftOutputBuilder, Output, Rent,
        },
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

// TODO checked operations ?

pub(crate) fn base_token_sums(
    selected_inputs: &[InputSigningData],
    outputs: &[OutputInfo],
) -> (u64, u64, HashMap<Address, u64>, HashMap<Address, u64>) {
    let mut inputs_sum = 0;
    let mut outputs_sum = 0;
    let mut inputs_sdr = HashMap::new();
    let mut outputs_sdr = HashMap::new();

    for selected_input in selected_inputs {
        inputs_sum += selected_input.output.amount();

        if let Some(sdruc) = selected_input
            .output
            .unlock_conditions()
            .and_then(|unlock_condition| unlock_condition.storage_deposit_return())
        {
            *inputs_sdr.entry(*sdruc.return_address()).or_default() += sdruc.amount();
        }
    }

    for output in outputs {
        outputs_sum += output.output.amount();

        if let Output::Basic(output) = &output.output {
            if let Some(address) = output.simple_deposit_address() {
                *outputs_sdr.entry(*address).or_default() += output.amount();
            }
        }
    }

    (inputs_sum, outputs_sum, inputs_sdr, outputs_sdr)
}

impl InputSelection {
    pub(crate) fn fulfill_base_token_requirement(&mut self) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
        let (mut inputs_sum, mut outputs_sum, mut inputs_sdr, mut outputs_sdr) =
            base_token_sums(&self.selected_inputs, &self.outputs);
        let mut newly_selected_inputs = Vec::new();
        let mut newly_selected_ids = HashSet::new();

        if inputs_sum >= outputs_sum {
            return Ok((newly_selected_inputs, None));
        }

        // TODO don't pick burned things

        // TODO if consolidate strategy: sum all the lowest amount until diff is covered.
        // TODO this would be lowest amount of input strategy.
        self.available_inputs
            .sort_by(|left, right| left.output.amount().cmp(&right.output.amount()));

        'overall: {
            // 1. Basic with ED25519 address without SDRUC
            {
                let inputs = self.available_inputs.iter().filter(|input| {
                    if let Output::Basic(output) = &input.output {
                        output.address().is_ed25519() && output.unlock_conditions().storage_deposit_return().is_none()
                    } else {
                        false
                    }
                });

                for input in inputs {
                    inputs_sum += input.output.amount();
                    newly_selected_inputs.push(input.clone());
                    newly_selected_ids.insert(*input.output_id());

                    if inputs_sum >= outputs_sum {
                        break 'overall;
                    }
                }
            }

            // 2. Basic with ED25519 address and SDRUC
            {
                let inputs = self.available_inputs.iter().filter(|input| {
                    if let Output::Basic(output) = &input.output {
                        if output.address().is_ed25519() {
                            if let Some(sdr) = output.unlock_conditions().storage_deposit_return() {
                                // Filter out outputs that have to send back their full amount as they contribute to nothing.
                                sdr.amount() != input.output.amount()
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

                for input in inputs {
                    let sdruc = input
                        .output
                        .unlock_conditions()
                        .and_then(|unlock_conditions| unlock_conditions.storage_deposit_return())
                        .unwrap();

                    inputs_sum += input.output.amount();
                    newly_selected_inputs.push(input.clone());
                    newly_selected_ids.insert(*input.output_id());

                    let input_sdr = inputs_sdr.get(sdruc.return_address()).unwrap_or(&0) + sdruc.amount();
                    let output_sdr = *outputs_sdr.get(sdruc.return_address()).unwrap_or(&0);

                    if input_sdr > output_sdr {
                        let diff = input_sdr - output_sdr;
                        outputs_sum += diff;
                        *outputs_sdr.entry(*sdruc.return_address()).or_default() += sdruc.amount();
                    }

                    *inputs_sdr.entry(*sdruc.return_address()).or_default() += sdruc.amount();

                    if inputs_sum >= outputs_sum {
                        break 'overall;
                    }
                }
            }

            // 3. Basic with other kind of address
            {
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
                    inputs_sum += input.output.amount();
                    newly_selected_inputs.push(input.clone());
                    newly_selected_ids.insert(*input.output_id());

                    if inputs_sum >= outputs_sum {
                        break 'overall;
                    }
                }
            }

            // 4. Other kinds of outputs
            {
                let mut inputs = self
                    .available_inputs
                    .iter()
                    .filter(|input| !input.output.is_basic())
                    .peekable();

                if inputs.peek().is_some() {
                    for input in inputs {
                        inputs_sum += input.output.amount();
                        newly_selected_inputs.push(input.clone());
                        newly_selected_ids.insert(*input.output_id());

                        if inputs_sum >= outputs_sum {
                            break;
                        }
                    }

                    self.available_inputs
                        .retain(|input| !newly_selected_ids.contains(input.output_id()));

                    // TODO explanation of BaseToken
                    return Ok((newly_selected_inputs, Some(Requirement::BaseToken)));
                }
            }
        }

        'ici: {
            if inputs_sum < outputs_sum {
                // Moving funds of already transitioned other outputs ?
                let outputs = self
                    .outputs
                    .iter_mut()
                    .filter(|output| !output.output.is_basic() && !output.provided);

                for output in outputs {
                    let diff = outputs_sum - inputs_sum;
                    let amount = output.output.amount();
                    let rent = output.output.rent_cost(self.protocol_parameters.rent_structure());

                    let new_amount = if amount >= diff + rent { amount - diff } else { rent };

                    // TODO check that new_amount is enough for the rent

                    let new_output = match &output.output {
                        Output::Alias(output) => AliasOutputBuilder::from(output)
                            .with_amount(new_amount)?
                            .finish_output(self.protocol_parameters.token_supply())?,
                        Output::Nft(output) => NftOutputBuilder::from(output)
                            .with_amount(new_amount)?
                            .finish_output(self.protocol_parameters.token_supply())?,
                        Output::Foundry(output) => FoundryOutputBuilder::from(output)
                            .with_amount(new_amount)?
                            .finish_output(self.protocol_parameters.token_supply())?,
                        _ => panic!("only alias, nft and foundry can be automatically created"),
                    };

                    outputs_sum -= amount - new_amount;
                    output.output = new_output;

                    if inputs_sum >= outputs_sum {
                        break 'ici;
                    }
                }

                return Err(Error::InsufficientBaseTokenAmount {
                    found: inputs_sum,
                    required: outputs_sum,
                });
            }
        }

        self.available_inputs
            .retain(|input| !newly_selected_ids.contains(input.output_id()));

        Ok((newly_selected_inputs, None))
    }
}
