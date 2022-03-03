// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

use crate::{signing::types::InputSigningData, Error, Result};

use bee_message::{
    address::Address,
    input::INPUT_COUNT_MAX,
    output::{ByteCostConfig, Output, TokenId},
};
use packable::PackableExt;

use primitive_types::U256;

use std::collections::{hash_map::Entry, HashMap};

mod automatic;
pub(crate) use automatic::get_inputs;
mod manual;
pub(crate) use manual::get_custom_inputs;
mod native_token_helpers;
mod output_data;
pub mod types;
use native_token_helpers::{get_minted_and_burned_native_tokens, get_remainder_native_tokens, missing_native_tokens};
use output_data::{get_accumulated_output_amounts, get_remainder};
use types::SelectedTransactionData;

/// Select inputs from provided inputs([InputSigningData]) for provided [Output]s, validate amounts and create remainder
/// output if necessary. Also checks for alias, foundry and nft that they exist in the inputs if required.
pub async fn try_select_inputs(
    mut inputs: Vec<InputSigningData>,
    mut outputs: Vec<Output>,
    force_use_all_inputs: bool,
    remainder_address: Option<Address>,
    byte_cost_config: &ByteCostConfig,
) -> Result<SelectedTransactionData> {
    inputs.dedup();
    if inputs.len() as u16 > INPUT_COUNT_MAX {
        return Err(Error::ConsolidationRequired(inputs.len()));
    }

    let input_outputs = inputs
        .iter()
        .map(|i| Ok(Output::try_from(&i.output_response.output)?))
        .collect::<Result<Vec<Output>>>()?;

    // Validate and only create a remainder if necessary
    if force_use_all_inputs {
        let remainder_output = get_remainder(&input_outputs, &outputs, remainder_address, byte_cost_config).await?;
        if let Some(remainder_output) = &remainder_output {
            outputs.push(remainder_output.clone());
        }
        return Ok(SelectedTransactionData {
            inputs,
            outputs,
            remainder_output,
        });
    }
    // else: only select inputs that are necessary for the provided outputs

    let required = get_accumulated_output_amounts(&outputs).await?;

    // check if a foundry minted native tokens
    let (minted_native_tokens, burned_native_tokens) = get_minted_and_burned_native_tokens(&input_outputs, &outputs)?;
    let mut selected_input_native_tokens: HashMap<TokenId, U256> = minted_native_tokens;
    let mut required_native_tokens: HashMap<TokenId, U256> = required.native_tokens;
    // add burned native tokens as outputs, because we need to have this amount in the inputs
    for (tokend_id, burned_amount) in burned_native_tokens {
        match required_native_tokens.entry(tokend_id) {
            Entry::Vacant(e) => {
                e.insert(burned_amount);
            }
            Entry::Occupied(mut e) => {
                *e.get_mut() += burned_amount;
            }
        }
    }

    let mut selected_input_amount = 0;
    let mut selected_inputs = Vec::new();

    // 1. get alias, foundry or nft inputs (because amount and native tokens of these outputs could be used)
    for input_signing_data in &inputs {
        let output = Output::try_from(&input_signing_data.output_response.output)?;
        match output {
            Output::Alias(_) | Output::Foundry(_) | Output::Nft(_) => {
                selected_input_amount += output.amount();
                if let Some(output_native_tokens) = output.native_tokens() {
                    for native_token in output_native_tokens.iter() {
                        match selected_input_native_tokens.entry(*native_token.token_id()) {
                            Entry::Vacant(e) => {
                                e.insert(*native_token.amount());
                            }
                            Entry::Occupied(mut e) => {
                                *e.get_mut() += *native_token.amount();
                            }
                        }
                    }
                }
                selected_inputs.push(input_signing_data.clone());
            }
            _ => {}
        }
    }

    // 2. get native tokens (because amount of these outputs will also be used)
    if !required_native_tokens.is_empty() {
        for input_signing_data in &inputs {
            // only add outputs that aren't already in the inputs
            if !selected_inputs.iter().any(|e| {
                e.output_response.transaction_id == input_signing_data.output_response.transaction_id
                    && e.output_response.output_index == input_signing_data.output_response.output_index
            }) {
                let output = Output::try_from(&input_signing_data.output_response.output)?;
                if let Some(output_native_tokens) = output.native_tokens() {
                    for native_token in output_native_tokens.iter() {
                        // only check required tokens
                        if let Some(required_native_token_amount) = required_native_tokens.get(native_token.token_id())
                        {
                            match selected_input_native_tokens.entry(*native_token.token_id()) {
                                Entry::Vacant(e) => {
                                    e.insert(*native_token.amount());
                                    selected_input_amount += output.amount();
                                    selected_inputs.push(input_signing_data.clone());
                                }
                                Entry::Occupied(mut e) => {
                                    // only add if we haven't already reached the required amount
                                    let mut amount = *e.get_mut();
                                    if amount < *required_native_token_amount {
                                        amount += *native_token.amount();
                                        selected_input_amount += output.amount();
                                        selected_inputs.push(input_signing_data.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // check if we got all required native tokens
    // println!("selected_input_native_tokens: {:?}", selected_input_native_tokens);
    if let Some(native_token) = missing_native_tokens(&selected_input_native_tokens, &required_native_tokens) {
        return Err(Error::NotEnoughNativeTokens(native_token));
    }
    // check if we have too many inputs
    let current_selected_input_len = selected_inputs.len() as u16;
    if current_selected_input_len > INPUT_COUNT_MAX {
        return Err(Error::ConsolidationRequired(current_selected_input_len.into()));
    }

    // todo first try to select inputs with an exact matching amount
    // 3. try to select outputs without native tokens
    for input_signing_data in inputs
        .iter()
        // Max inputs is 128
        .take((INPUT_COUNT_MAX - current_selected_input_len).into())
    {
        // todo: check if this is necessary or if we can just check by output types (foundry, alias, nft should be
        // selected before because of chains)
        if selected_input_amount < required.amount {
            // only add outputs that aren't already in the inputs
            if !selected_inputs.iter().any(|e| {
                e.output_response.transaction_id == input_signing_data.output_response.transaction_id
                    && e.output_response.output_index == input_signing_data.output_response.output_index
            }) {
                let output = Output::try_from(&input_signing_data.output_response.output)?;
                if let Some(output_native_tokens) = output.native_tokens() {
                    if output_native_tokens.is_empty() {
                        selected_input_amount += output.amount();
                        selected_inputs.push(input_signing_data.clone());
                    }
                }
            }
        }
    }
    // check if we have too many inputs
    let current_selected_input_len = selected_inputs.len() as u16;
    if current_selected_input_len > INPUT_COUNT_MAX {
        return Err(Error::ConsolidationRequired(current_selected_input_len.into()));
    }

    // Order input outputs descending, so that as few inputs as necessary are used
    inputs.sort_by(|l, r| {
        let output_1 = Output::try_from(&l.output_response.output).unwrap();
        let output_2 = Output::try_from(&r.output_response.output).unwrap();
        output_1.amount().cmp(&output_2.amount())
    });

    // 4. try to select outputs with native tokens
    // todo: handle remainder amount for native tokens
    for input_signing_data in inputs
        .iter()
        // Max inputs is 128
        .take((INPUT_COUNT_MAX - current_selected_input_len).into())
    {
        // todo: check if this is necessary or if we can just check by output types (foundry, alias, nft should be
        // selected before because of chains)
        if selected_input_amount < required.amount {
            // only add outputs that aren't already in the inputs
            if !selected_inputs.iter().any(|e| {
                e.output_response.transaction_id == input_signing_data.output_response.transaction_id
                    && e.output_response.output_index == input_signing_data.output_response.output_index
            }) {
                let output = Output::try_from(&input_signing_data.output_response.output)?;
                selected_input_amount += output.amount();
                if let Some(output_native_tokens) = output.native_tokens() {
                    for native_token in output_native_tokens.iter() {
                        match selected_input_native_tokens.entry(*native_token.token_id()) {
                            Entry::Vacant(e) => {
                                e.insert(*native_token.amount());
                            }
                            Entry::Occupied(mut e) => {
                                *e.get_mut() += *native_token.amount();
                            }
                        }
                    }
                }
                selected_inputs.push(input_signing_data.clone());
            }
        }
    }

    // create remainder output if necessary
    let selected_input_outputs = selected_inputs
        .iter()
        .map(|i| Ok(Output::try_from(&i.output_response.output)?))
        .collect::<Result<Vec<Output>>>()?;
    // get_remainder also checks for amounts and returns an error if we don't have enough
    let remainder_output =
        get_remainder(&selected_input_outputs, &outputs, remainder_address, byte_cost_config).await?;
    if let Some(remainder_output) = &remainder_output {
        outputs.push(remainder_output.clone());
    }

    // sort inputs so ed25519 address unlocks will be first, safe to unwrap since we encoded it before
    selected_inputs.sort_unstable_by_key(|a| Address::try_from_bech32(&a.bech32_address).unwrap().1.pack_to_vec());
    Ok(SelectedTransactionData {
        inputs: selected_inputs,
        outputs,
        remainder_output,
    })
}
