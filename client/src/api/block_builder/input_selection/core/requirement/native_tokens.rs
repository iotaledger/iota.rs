// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{cmp::Ordering, collections::HashSet};

use primitive_types::U256;

use super::{Error, InputSelection};
use crate::{
    block::output::{AliasTransition, NativeToken, NativeTokens, NativeTokensBuilder, Output, TokenScheme},
    secret::types::InputSigningData,
};

pub(crate) fn get_native_tokens<'a>(outputs: impl Iterator<Item = &'a Output>) -> Result<NativeTokensBuilder, Error> {
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
    outputs: &[Output],
) -> Result<(NativeTokensBuilder, NativeTokensBuilder), Error> {
    let mut minted_native_tokens = NativeTokensBuilder::new();
    let mut melted_native_tokens = NativeTokensBuilder::new();

    for output in outputs {
        if let Output::Foundry(output_foundry) = output {
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
) -> Result<Option<NativeTokens>, Error> {
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
        &mut self,
    ) -> Result<Vec<(InputSigningData, Option<AliasTransition>)>, Error> {
        let mut input_native_tokens = get_native_tokens(self.selected_inputs.iter().map(|input| &input.output))?;
        let mut output_native_tokens = get_native_tokens(self.outputs.iter())?;
        let (minted_native_tokens, melted_native_tokens) =
            get_minted_and_melted_native_tokens(&self.selected_inputs, &self.outputs)?;

        input_native_tokens.merge(minted_native_tokens)?;
        output_native_tokens.merge(melted_native_tokens)?;

        if let Some(burn) = self.burn.as_ref() {
            output_native_tokens.merge(NativeTokensBuilder::from(burn.native_tokens.clone()))?;
        }

        // TODO weird that it happens in this direction?
        if let Some(diffs) = get_native_tokens_diff(&output_native_tokens, &input_native_tokens)? {
            log::debug!(
                "Fulfilling native tokens requirement with input {input_native_tokens:?} and output {output_native_tokens:?}"
            );

            let mut newly_selected_inputs = Vec::new();
            let mut newly_selected_ids = HashSet::new();

            for diff in diffs.iter() {
                let mut amount = U256::zero();
                // TODO sort ?
                let inputs = self.available_inputs.iter().filter(|input| {
                    input
                        .output
                        .native_tokens()
                        .map_or(false, |native_tokens| native_tokens.contains(diff.token_id()))
                });

                for input in inputs {
                    amount += input
                        .output
                        .native_tokens()
                        .and_then(|native_tokens| native_tokens.get(diff.token_id()))
                        // PANIC: safe to unwrap as the filter guarantees inputs containing this native token.
                        .unwrap()
                        .amount();

                    if newly_selected_ids.insert(*input.output_id()) {
                        newly_selected_inputs.push((input.clone(), None));
                    }

                    if amount >= diff.amount() {
                        break;
                    }
                }

                if amount < diff.amount() {
                    return Err(Error::InsufficientNativeTokenAmount {
                        token_id: *diff.token_id(),
                        found: amount,
                        required: diff.amount(),
                    });
                }
            }

            log::debug!("Outputs {newly_selected_ids:?} selected to fulfill the native tokens requirement");

            self.available_inputs
                .retain(|input| !newly_selected_ids.contains(input.output_id()));

            Ok(newly_selected_inputs)
        } else {
            log::debug!("Native tokens requirement already fulfilled");

            Ok(Vec::new())
        }
    }
}
