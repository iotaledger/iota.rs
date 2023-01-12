// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use super::{InputSelection, OutputInfo, Requirement};
use crate::{
    block::output::{
        unlock_condition::UnlockCondition, AliasOutputBuilder, FoundryOutputBuilder, NftOutputBuilder, Output, Rent,
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

// TODO checked operations ?

pub(crate) fn base_token_sums(selected_inputs: &[InputSigningData], outputs: &[OutputInfo]) -> (u64, u64) {
    let inputs_sum = selected_inputs.iter().map(|input| input.output.amount()).sum::<u64>();
    let outputs_sum = outputs.iter().map(|output| output.output.amount()).sum::<u64>();

    (inputs_sum, outputs_sum)
}

impl InputSelection {
    pub(crate) fn fulfill_base_token_requirement(&mut self) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
        let (mut inputs_sum, mut outputs_sum) = base_token_sums(&self.selected_inputs, &self.outputs);
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
            // 1. Basic with ED25519 address and nothing else
            {
                let inputs = self.available_inputs.iter().filter(|input| {
                    if let Output::Basic(output) = &input.output {
                        if let [UnlockCondition::Address(address)] = output.unlock_conditions().as_ref() {
                            address.address().is_ed25519()
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

            // 2. Basic with ED25519 address and other things ????
            {}

            // 3. Moving funds of already transitioned other outputs
            {}

            // 4. Basic with other kind of address
            {}

            // 5. Other kinds of outputs
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
                        _ => panic!("TODO"),
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
