// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use crate::{
    block::output::Output,
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
    let (inputs_sum, outputs_sum) = base_token_sums(selected_inputs, outputs);
    let mut newly_selected_inputs = Vec::new();

    if inputs_sum < outputs_sum {
        let diff = outputs_sum - inputs_sum;
        let mut newly_covered = 0;

        // // 1. Basic with ED25519 address and nothing else
        // // 2. Basic with ED25519 address and other things
        // // 3. Moving funds of already transitioned other outputs
        // // 4. Basic with other kind of address
        // // 5. Other kinds of outputs

        // // // TODO a first pass with even more basic restrictions?
        // let basic_inputs = available_inputs.iter().filter(|input| input.output.is_basic());

        // // evaluate(basic_inputs, diff, &mut newly_covered);

        // let current_selection = HashSet::new();
        // while let Some(input) = basic_inputs {
        //     newly_covered += input.output.amount();
        //     current_selection.insert(input.output_id());
        //     if diff <= newly_covered {
        //         break;
        //     }
        // }

        // TODO if consolidate strategy: sum all the lowest amount until diff is covered.
        available_inputs.sort_by(|left, right| left.output.amount().cmp(&right.output.amount()));

        // TODO this would be lowest amount of input strategy.
        while diff > newly_covered && !available_inputs.is_empty() {
            // TODO avoid remove because it shifts the order.
            let input = available_inputs.remove(0);
            newly_covered += input.output.amount();
            newly_selected_inputs.push(input);
        }

        if diff > newly_covered {
            return Err(Error::NotEnoughBalance {
                found: newly_covered,
                required: diff,
            });
        }

        // println!("{diff}");
    }

    Ok(newly_selected_inputs)
}
