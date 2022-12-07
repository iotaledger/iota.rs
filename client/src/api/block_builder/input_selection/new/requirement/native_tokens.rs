// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{OutputInfo, Requirement};
use crate::{error::Result, secret::types::InputSigningData};

pub(crate) fn fulfill_native_tokens_requirement(
    available_inputs: &mut [InputSigningData],
    selected_inputs: &[InputSigningData],
    outputs: &[OutputInfo],
) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
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

// let input_native_tokens = gather_nts(selected_inputs);
//     let output_native_tokens = gather_nts(outputs);
//     let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_nts(selected_inputs, outputs)?;
//     let native_tokens_diffs = (input_native_tokens + minted) - (output + melted + burn);

//     // TODO: add required amount for storage deposit return unlock conditions of inputs to the require output
// amount     let base_coin_diff = get_base_coin_diff(selected_inputs, output);

//     // if !native_tokens_diffs.is_empty() || base_coin_diff != 0
//     let remainder = OutputBuilder::new(base_coin_diff).with_native_tokens(native_tokens_diffs);

//     let min_storage_deposit = remainder.min_storage_deposit();
//     let base_coin_diff = base_coin_diff-min_storage_deposit;

//     let mut new_inputs = Vec::new();
//     if !native_tokens_diffs.is_empty() || base_coin_diff > 0{
//     // TODO: select inputs for base coin diff until we have enough for the remainder if necesary
//     }
//     new_inputs
