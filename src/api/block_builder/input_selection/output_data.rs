// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_block::output::{NativeTokensBuilder, Output};

use crate::{
    api::input_selection::{get_minted_and_melted_native_tokens, types::AccumulatedOutputAmounts},
    Result,
};

// Calculate required accumulated amounts from the outputs, considers also minted and melted native tokens
pub(crate) fn get_accumulated_output_amounts<'a>(
    inputs: &(impl Iterator<Item = &'a Output> + Clone),
    outputs: impl Iterator<Item = &'a Output> + Clone,
) -> Result<AccumulatedOutputAmounts> {
    // Calculate the total tokens to spend
    let mut required_amount: u64 = 0;
    let mut required_native_tokens = NativeTokensBuilder::new();

    for output in outputs.clone() {
        required_amount += output.amount();

        if let Some(output_native_tokens) = output.native_tokens() {
            required_native_tokens.add_native_tokens(output_native_tokens.clone())?;
        }
    }

    // check if a foundry mints or melts native tokens
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(inputs, outputs)?;
    // add melted native tokens as outputs, because we need to have this amount in the inputs
    required_native_tokens.merge(melted_native_tokens)?;

    Ok(AccumulatedOutputAmounts {
        minted_native_tokens,
        amount: required_amount,
        native_tokens: required_native_tokens,
    })
}
