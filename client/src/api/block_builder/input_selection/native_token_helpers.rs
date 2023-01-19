// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

use iota_types::block::output::{NativeToken, NativeTokensBuilder, Output, TokenScheme};
use primitive_types::U256;

use crate::Result;

// Get minted and melted tokens from foundry outputs
// minted first, melted second
pub(crate) fn get_minted_and_melted_native_tokens<'a>(
    inputs: &(impl Iterator<Item = &'a Output> + Clone),
    outputs: impl Iterator<Item = &'a Output> + Clone,
) -> Result<(NativeTokensBuilder, NativeTokensBuilder)> {
    let mut minted_native_tokens = NativeTokensBuilder::new();
    let mut melted_native_tokens = NativeTokensBuilder::new();

    for output in outputs {
        if let Output::Foundry(output_foundry) = output {
            let TokenScheme::Simple(output_foundry_simple_ts) = output_foundry.token_scheme();
            let mut initial_creation = true;

            for input in inputs.clone() {
                if let Output::Foundry(input_foundry) = input {
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
