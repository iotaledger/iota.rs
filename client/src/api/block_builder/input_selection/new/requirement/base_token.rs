// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use super::Requirement;
use crate::{
    block::output::{unlock_condition::UnlockCondition, Output},
    error::{Error, Result},
    secret::types::InputSigningData,
};

pub(crate) fn base_token_sums(selected_inputs: &[InputSigningData], outputs: &[Output]) -> (u64, u64) {
    let inputs_sum = selected_inputs.iter().map(|input| input.output.amount()).sum::<u64>();
    let outputs_sum = outputs.iter().map(|output| output.amount()).sum::<u64>();

    (inputs_sum, outputs_sum)
}

pub(crate) fn fulfill_base_token_requirement(
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
    let (mut inputs_sum, outputs_sum) = base_token_sums(selected_inputs, outputs);
    let mut newly_selected_inputs = Vec::new();
    let mut newly_selected_ids = HashSet::new();

    if inputs_sum >= outputs_sum {
        return Ok((newly_selected_inputs, None));
    }

    println!("BASE TOKEN {available_inputs:?}\n{selected_inputs:?}\n{outputs:?}");

    // TODO don't pick burned things

    // TODO if consolidate strategy: sum all the lowest amount until diff is covered.
    // TODO this would be lowest amount of input strategy.
    available_inputs.sort_by(|left, right| left.output.amount().cmp(&right.output.amount()));

    'overall: {
        // 1. Basic with ED25519 address and nothing else
        {
            let inputs = available_inputs.iter().filter(|input| {
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

                println!("SELECTED {:?}", input.output);

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
            let mut inputs = available_inputs
                .iter()
                .filter(|input| !input.output.is_basic())
                .peekable();

            if inputs.peek().is_some() {
                for input in inputs {
                    inputs_sum += input.output.amount();
                    newly_selected_inputs.push(input.clone());
                    newly_selected_ids.insert(*input.output_id());

                    println!("SELECTED {:?}", input.output);

                    if inputs_sum >= outputs_sum {
                        break;
                    }
                }

                available_inputs.retain(|input| !newly_selected_ids.contains(input.output_id()));

                return Ok((newly_selected_inputs, Some(Requirement::BaseToken)));
            }
        }
    }

    if inputs_sum < outputs_sum {
        // Moving funds of already transitioned other outputs ?
        println!("NOT ENOUGH, OUTPUTS: {:?}", outputs);
        return Err(Error::NotEnoughBalance {
            found: inputs_sum,
            required: outputs_sum,
        });
    }

    available_inputs.retain(|input| !newly_selected_ids.contains(input.output_id()));

    Ok((newly_selected_inputs, None))
}
