// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

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

// fn evaluate(inputs: impl Iterator<Item = InputSigningData>, diff: u64, newly_covered: &mut u64) {
//     let current_selection = HashSet::new();

//     while let Some(input) = inputs {
//         newly_covered += input.output.amount();
//         current_selection.insert(input.output_id());
//         if diff <= newly_covered {
//             break
//         }
//     }
// }

// TODO very dumb first draft.
pub(crate) fn fulfill_base_token_requirement(
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    let (mut inputs_sum, outputs_sum) = base_token_sums(selected_inputs, outputs);
    let mut newly_selected_inputs = Vec::new();
    let mut newly_selected_ids = HashSet::new();

    let diff = outputs_sum - inputs_sum;

    if inputs_sum >= outputs_sum {
        return Ok(newly_selected_inputs);
    }

    println!("{available_inputs:?}\n{selected_inputs:?}\n{outputs:?}");

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

                if inputs_sum >= outputs_sum {
                    break 'overall;
                }
            }

            // while inputs_sum < outputs_sum {
            //     let diff = outputs_sum - inputs_sum;

            //     // if available_inputs.is_empty() {
            //     //     return Err(Error::NotEnoughBalance {
            //     //         found: inputs_sum,
            //     //         required: outputs_sum,
            //     //     });
            //     // }
            // }
        }

        // 2. Basic with ED25519 address and other things ????
        {}

        // 3. Moving funds of already transitioned other outputs
        {}

        // 4. Basic with other kind of address
        {}

        // 5. Other kinds of outputs
        {
            let inputs = available_inputs.iter().filter(|input| !input.output.is_basic());

            for input in inputs {
                inputs_sum += input.output.amount();
                newly_selected_inputs.push(input.clone());
                newly_selected_ids.insert(*input.output_id());

                if inputs_sum >= outputs_sum {
                    break 'overall;
                }
            }
        }
    }

    if inputs_sum < outputs_sum {
        // Moving funds of already transitioned other outputs ?
        return Err(Error::NotEnoughBalance {
            found: inputs_sum,
            required: outputs_sum,
        });
    }

    available_inputs.retain(|input| !newly_selected_ids.contains(input.output_id()));

    Ok(newly_selected_inputs)
}
