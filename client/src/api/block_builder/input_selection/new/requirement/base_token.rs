// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    block::output::Output,
    error::{Error, Result},
    secret::types::InputSigningData,
};

// TODO very dumb first draft.
pub(crate) fn fulfill_base_coin_requirement(
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    let input_sum = selected_inputs.iter().map(|input| input.output.amount()).sum::<u64>();
    let output_sum = outputs.iter().map(|output| output.amount()).sum::<u64>();

    if input_sum >= output_sum {
        // Enough amount in the inputs to cover the outputs amount.
        Ok(Vec::new())
    } else {
        let diff = output_sum - input_sum;
        let mut newly_covered = 0;
        let mut newly_selected_inputs = Vec::new();

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

        println!("{diff}");

        Ok(newly_selected_inputs)
    }
}
