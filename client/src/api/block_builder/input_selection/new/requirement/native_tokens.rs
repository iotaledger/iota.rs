// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{block::output::Output, error::Result, secret::types::InputSigningData};

pub(crate) fn fulfill_native_tokens_requirement(
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    // let input_native_tokens = gather_nts(selected_inputs);
    // let output_native_tokens = gather_nts(outputs);

    // let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_nts(selected_inputs, outputs)?;

    // let diffs = (input_native_tokens + minted) - (output + melted + burn);

    // let mut new_inputs = Vec::new();

    // for (token_id, mut diff) in diffs {
    //     // subtract the possible added native token amount from new selected inputs
    //     let new_input_native_tokens = gather_nts(new_inputs);
    //     let amount = new_input_native_tokens.get(token_id);
    //     let diff = diff-amount;

    //     if diff < 0 {
    //         let native_token_inputs = available_inputs.filter(|input|input.has(token_id));
    //         let inputs = native_token_inputs.smart_sum(diff);
    //     }
    // }

    // new_inputs
    todo!()
}
