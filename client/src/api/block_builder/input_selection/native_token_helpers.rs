// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

use iota_types::block::output::{NativeToken, NativeTokens, NativeTokensBuilder, Output, TokenScheme};
use primitive_types::U256;

use crate::Result;

pub(crate) fn missing_native_tokens(
    inputs: &NativeTokensBuilder,
    required: &NativeTokensBuilder,
) -> Result<Option<NativeTokens>> {
    let mut missing_native_tokens = NativeTokensBuilder::new();

    for (token_id, required_native_token_amount) in required.iter() {
        match inputs.get(token_id) {
            None => {
                missing_native_tokens.insert(*token_id, *required_native_token_amount);
            }
            Some(amount) => {
                if amount < required_native_token_amount {
                    missing_native_tokens.insert(*token_id, required_native_token_amount - amount);
                }
            }
        }
    }

    if missing_native_tokens.is_empty() {
        Ok(None)
    } else {
        Ok(Some(missing_native_tokens.finish()?))
    }
}

pub(crate) fn get_remainder_native_tokens(
    inputs: &NativeTokensBuilder,
    required: &NativeTokensBuilder,
) -> Result<Option<NativeTokens>> {
    // inputs and required are switched
    missing_native_tokens(required, inputs)
}

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

#[cfg(test)]
mod tests {
    use iota_types::block::output::TokenId;

    use super::*;

    #[test]
    fn native_token() {
        let token_id_bytes: [u8; 38] =
            prefix_hex::decode("0x08e68f7616cd4948efebc6a77c4f935eaed770ac53869cba56d104f2b472a8836d0100000000")
                .unwrap();
        let token_id = TokenId::from(token_id_bytes);

        // inputs == required
        let mut input_native_tokens = NativeTokensBuilder::new();
        input_native_tokens.insert(token_id, U256::from(50u32));
        let mut required_native_tokens = NativeTokensBuilder::new();
        required_native_tokens.insert(token_id, U256::from(50u32));

        assert_eq!(
            None,
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens).unwrap()
        );

        // no inputs
        let input_native_tokens = NativeTokensBuilder::new();
        let mut required_native_tokens = NativeTokensBuilder::new();
        required_native_tokens.insert(token_id, U256::from(50u32));

        assert_eq!(
            Some(required_native_tokens.clone().finish().unwrap()),
            missing_native_tokens(&input_native_tokens, &required_native_tokens).unwrap()
        );

        // no inputs used
        let mut input_native_tokens = NativeTokensBuilder::new();
        input_native_tokens.insert(token_id, U256::from(50u32));
        let required_native_tokens = NativeTokensBuilder::new();

        assert_eq!(
            Some(input_native_tokens.clone().finish().unwrap()),
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens).unwrap()
        );

        // only a part of the inputs is used
        let mut input_native_tokens = NativeTokensBuilder::new();
        input_native_tokens.insert(token_id, U256::from(50u32));
        let mut required_native_tokens = NativeTokensBuilder::new();
        required_native_tokens.insert(token_id, U256::from(20u32));
        let mut remainder_native_tokens = NativeTokensBuilder::new();
        remainder_native_tokens.insert(token_id, U256::from(30u32));

        assert_eq!(
            Some(remainder_native_tokens.finish().unwrap()),
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens).unwrap()
        );

        // more amount than required
        let mut input_native_tokens = NativeTokensBuilder::new();
        input_native_tokens.insert(token_id, U256::from(20u32));
        let mut required_native_tokens = NativeTokensBuilder::new();
        required_native_tokens.insert(token_id, U256::from(50u32));
        let mut remainder_native_tokens = NativeTokensBuilder::new();
        remainder_native_tokens.insert(token_id, U256::from(30u32));

        assert_eq!(
            Some(remainder_native_tokens.finish().unwrap()),
            missing_native_tokens(&input_native_tokens, &required_native_tokens).unwrap()
        );
        assert_eq!(
            None,
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens).unwrap()
        );
    }
}
