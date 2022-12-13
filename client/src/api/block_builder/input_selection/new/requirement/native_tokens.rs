// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

use primitive_types::U256;

use super::{InputSelection, OutputInfo, Requirement};
use crate::{
    block::output::{NativeToken, NativeTokens, NativeTokensBuilder, Output, TokenScheme},
    error::Result,
    secret::types::InputSigningData,
};

pub(crate) fn get_native_tokens<'a>(outputs: impl Iterator<Item = &'a Output>) -> Result<NativeTokensBuilder> {
    let mut required_native_tokens = NativeTokensBuilder::new();

    for output in outputs {
        if let Some(output_native_tokens) = output.native_tokens() {
            required_native_tokens.add_native_tokens(output_native_tokens.clone())?;
        }
    }

    Ok(required_native_tokens)
}

pub(crate) fn get_minted_and_melted_native_tokens(
    inputs: &[InputSigningData],
    outputs: &[OutputInfo],
) -> Result<(NativeTokensBuilder, NativeTokensBuilder)> {
    let mut minted_native_tokens = NativeTokensBuilder::new();
    let mut melted_native_tokens = NativeTokensBuilder::new();

    for output in outputs {
        if let Output::Foundry(output_foundry) = &output.output {
            let TokenScheme::Simple(output_foundry_simple_ts) = output_foundry.token_scheme();
            let mut initial_creation = true;

            for input in inputs {
                if let Output::Foundry(input_foundry) = &input.output {
                    let token_id = output_foundry.token_id();

                    if output_foundry.id() == input_foundry.id() {
                        initial_creation = false;
                        let TokenScheme::Simple(input_foundry_simple_ts) = input_foundry.token_scheme();

                        match output_foundry_simple_ts
                            .circulating_supply()
                            .cmp(&input_foundry_simple_ts.circulating_supply())
                        {
                            Ordering::Greater => {
                                let minted_native_token_amount = output_foundry_simple_ts.circulating_supply()
                                    - input_foundry_simple_ts.circulating_supply();

                                minted_native_tokens
                                    .add_native_token(NativeToken::new(token_id, minted_native_token_amount)?)?;
                            }
                            Ordering::Less => {
                                let melted_native_token_amount = input_foundry_simple_ts.circulating_supply()
                                    - output_foundry_simple_ts.circulating_supply();

                                melted_native_tokens
                                    .add_native_token(NativeToken::new(token_id, melted_native_token_amount)?)?;
                            }
                            Ordering::Equal => {}
                        }
                    }
                }
            }

            // If we created the foundry with this transaction, then we need to add the circulating supply as minted
            // tokens
            if initial_creation {
                let circulating_supply = output_foundry_simple_ts.circulating_supply();

                if circulating_supply != U256::from(0) {
                    minted_native_tokens
                        .add_native_token(NativeToken::new(output_foundry.token_id(), circulating_supply)?)?;
                }
            }
        }
    }

    Ok((minted_native_tokens, melted_native_tokens))
}

// TODO checked ops
// TODO only handles one side
pub(crate) fn get_native_tokens_diff(
    inputs: &NativeTokensBuilder,
    outputs: &NativeTokensBuilder,
) -> Result<Option<NativeTokens>> {
    let mut native_tokens_diff = NativeTokensBuilder::new();

    for (token_id, input_amount) in inputs.iter() {
        match outputs.get(token_id) {
            None => {
                native_tokens_diff.insert(*token_id, *input_amount);
            }
            Some(output_amount) => {
                if input_amount > output_amount {
                    native_tokens_diff.insert(*token_id, input_amount - output_amount);
                }
            }
        }
    }

    if native_tokens_diff.is_empty() {
        Ok(None)
    } else {
        Ok(Some(native_tokens_diff.finish()?))
    }
}

impl InputSelection {
    pub(crate) fn fulfill_native_tokens_requirement(
        &self,
        selected_inputs: &[InputSigningData],
    ) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
        println!("NATIVE TOKENS");

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
        Ok((Vec::new(), None))
    }
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
