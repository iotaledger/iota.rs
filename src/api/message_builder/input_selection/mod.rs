// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

use crate::{
    api::{address::search_address, ClientMessageBuilder, ADDRESS_GAP_RANGE},
    node_api::indexer_api::query_parameters::QueryParameter,
    signing::types::InputSigningData,
    Error, Result,
};

use bee_message::{
    address::Address,
    input::INPUT_COUNT_MAX,
    output::{AliasId, Output, TokenId},
};
use packable::PackableExt;

use primitive_types::U256;

use std::collections::{hash_map::Entry, HashMap, HashSet};

mod native_token_helpers;
mod output_data;
pub mod types;
use native_token_helpers::{get_minted_native_tokens, get_remainder_native_tokens, missing_native_tokens};
use output_data::{get_accumulated_output_amounts, get_remainder, get_utxo_chains_inputs};
use types::SelectedTransactionData;

/// Select inputs from provided inputs([InputSigningData]) for provided [Output]s, validate amounts and create remainder
/// output if necessary. Also checks for alias, foundry and nft that they exist in the inputs if required.
pub async fn try_select_inputs(
    mut inputs: Vec<InputSigningData>,
    mut outputs: Vec<Output>,
    force_use_all_inputs: bool,
    remainder_address: Option<Address>,
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
        let remainder_output = get_remainder(&input_outputs, &outputs, remainder_address).await?;
        return Ok(SelectedTransactionData {
            inputs,
            outputs,
            remainder_output,
        });
    }
    // else: only select inputs that are necessary for the provided outputs

    let required = get_accumulated_output_amounts(&outputs).await?;
    // println!("required: {:?}", required);

    // check if a foundry minted native tokens
    let mut selected_input_native_tokens: HashMap<TokenId, U256> = get_minted_native_tokens(&input_outputs, &outputs)?;

    let mut selected_input_amount = 0;
    let mut selected_inputs = Vec::new();

    // 1. get alias, foundry or nft inputs (because amount and native tokens of these outputs could be used)
    for input_signing_data in &inputs {
        let output = Output::try_from(&input_signing_data.output_response.output)?;
        match output {
            Output::Alias(_) | Output::Foundry(_) | Output::Nft(_) => {
                selected_input_amount += output.amount();
                if let Some(output_native_tokens) = output.native_tokens() {
                    for native_token in output_native_tokens {
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
    if !required.native_tokens.is_empty() {
        for input_signing_data in &inputs {
            // only add outputs that aren't already in the inputs
            if !selected_inputs.iter().any(|e| {
                e.output_response.transaction_id == input_signing_data.output_response.transaction_id
                    && e.output_response.output_index == input_signing_data.output_response.output_index
            }) {
                let output = Output::try_from(&input_signing_data.output_response.output)?;
                if let Some(output_native_tokens) = output.native_tokens() {
                    for native_token in output_native_tokens {
                        // only check required tokens
                        if let Some(required_native_token_amount) = required.native_tokens.get(native_token.token_id())
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
    if let Some(native_token) = missing_native_tokens(&selected_input_native_tokens, &required.native_tokens) {
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
                    for native_token in output_native_tokens {
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
    // println!("selected_input_outputs {:?}", selected_input_outputs);
    let remainder_output = get_remainder(&selected_input_outputs, &outputs, remainder_address).await?;
    if let Some(remainder_output) = &remainder_output {
        outputs.push(remainder_output.clone());
    }

    // sort inputs so ed25519 address unlocks will be first, safe to unwrap since we encoded it before
    selected_inputs.sort_unstable_by_key(|a| Address::try_from_bech32(&a.bech32_address).unwrap().pack_to_vec());
    Ok(SelectedTransactionData {
        inputs: selected_inputs,
        outputs,
        remainder_output,
    })
}

/// Searches inputs for provided outputs, by requesting the outputs from the account addresses
/// Forwards to [try_select_inputs()]
pub(crate) async fn get_inputs(message_builder: &ClientMessageBuilder<'_>) -> Result<SelectedTransactionData> {
    let account_index = message_builder.account_index;
    let mut gap_index = message_builder.initial_address_index;
    let mut empty_address_count: u64 = 0;
    let mut cached_error = None;

    // first get inputs for utxo chains
    let mut available_inputs = get_utxo_chains_inputs(message_builder, &message_builder.outputs).await?;

    // then select inputs with outputs from addresses
    let selected_transaction_data = 'input_selection: loop {
        // Get the addresses in the BIP path/index ~ path/index+20
        let addresses = message_builder
            .client
            .get_addresses(
                message_builder
                    .signer
                    .as_ref()
                    .ok_or(crate::Error::MissingParameter("signer"))?,
            )
            .with_account_index(account_index)
            .with_range(gap_index..gap_index + ADDRESS_GAP_RANGE)
            .get_all()
            .await?;
        // Have public and internal addresses with the index ascending ordered
        let mut public_and_internal_addresses = Vec::new();
        for index in 0..addresses.public.len() {
            public_and_internal_addresses.push((addresses.public[index].clone(), false));
            public_and_internal_addresses.push((addresses.internal[index].clone(), true));
        }

        // For each address, get the address outputs
        let mut address_index = gap_index;
        for (index, (str_address, internal)) in public_and_internal_addresses.iter().enumerate() {
            let output_ids = crate::node_api::indexer_api::routes::output_ids(
                message_builder.client,
                vec![QueryParameter::Address(str_address.to_string())],
            )
            .await?;

            let address_outputs = crate::node_api::core_api::get_outputs(message_builder.client, output_ids).await?;

            // If there are more than 20 (ADDRESS_GAP_RANGE) consecutive empty addresses, then we stop
            // looking up the addresses belonging to the seed. Note that we don't
            // really count the exact 20 consecutive empty addresses, which is
            // unnecessary. We just need to check the address range,
            // (index * ADDRESS_GAP_RANGE, index * ADDRESS_GAP_RANGE + ADDRESS_GAP_RANGE), where index is
            // natural number, and to see if the outputs are all empty.
            if address_outputs.is_empty() {
                // Accumulate the empty_address_count for each run of output address searching
                empty_address_count += 1;
            } else {
                // Reset counter if there is an output
                empty_address_count = 0;

                for output_response in address_outputs {
                    available_inputs.push(ClientMessageBuilder::create_input_signing_data(
                        message_builder.coin_type,
                        account_index,
                        address_index,
                        *internal,
                        &output_response,
                        str_address.to_owned(),
                    )?);
                }
                let selected_transaction_data = match try_select_inputs(
                    available_inputs.clone(),
                    message_builder.outputs.clone(),
                    false,
                    // todo allow custom remainder address
                    None,
                )
                .await
                {
                    Ok(r) => r,
                    // for these errors just try again in the next round with more addresses
                    Err(crate::Error::NotEnoughBalance(a, b)) => {
                        cached_error.replace(crate::Error::NotEnoughBalance(a, b));
                        continue;
                    }
                    Err(crate::Error::NotEnoughNativeTokens(a)) => {
                        cached_error.replace(crate::Error::NotEnoughNativeTokens(a));
                        continue;
                    }
                    Err(crate::Error::NotEnoughBalanceForNativeTokenRemainder) => {
                        cached_error.replace(crate::Error::NotEnoughBalanceForNativeTokenRemainder);
                        continue;
                    }
                    Err(crate::Error::ConsolidationRequired(v)) => {
                        cached_error.replace(crate::Error::ConsolidationRequired(v));
                        continue;
                    }
                    Err(e) => return Err(e),
                };

                break 'input_selection selected_transaction_data;
            }

            // if we just processed an even index, increase the address index
            // (because the list has public and internal addresses)
            if index % 2 == 1 {
                address_index += 1;
            }
        }
        gap_index += ADDRESS_GAP_RANGE;
        // The gap limit is 20 and use reference 40 here because there's public and internal addresses
        if empty_address_count >= (ADDRESS_GAP_RANGE * 2) as u64 {
            // returned last cached error
            return Err(cached_error.unwrap_or(Error::NoInputs));
        }
    };

    Ok(selected_transaction_data)
}

/// If custom inputs are provided we check if they are unspent, get the balance and search the Ed25519 addresses for
/// them with the provided input_range so we can later sign them.
/// Forwards to [try_select_inputs()] with `force_use_all_inputs` set to true, so all inputs will be included in the
/// transaction, even if not required for the provided outputs.
pub(crate) async fn get_custom_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    governance_transition: Option<HashSet<AliasId>>,
) -> Result<SelectedTransactionData> {
    let mut input_signing_data_entrys = Vec::new();

    if let Some(inputs) = &message_builder.inputs {
        for input in inputs {
            let output_response = message_builder.client.get_output(input.output_id()).await?;

            if !output_response.is_spent {
                let (_output_amount, output_address) = ClientMessageBuilder::get_output_amount_and_address(
                    &output_response.output,
                    governance_transition.clone(),
                )?;

                let bech32_hrp = message_builder.client.get_bech32_hrp().await?;
                let (address_index, internal) = match message_builder.signer {
                    Some(signer) => {
                        match output_address {
                            Address::Ed25519(_) => {
                                search_address(
                                    signer,
                                    &bech32_hrp,
                                    message_builder.coin_type,
                                    message_builder.account_index,
                                    message_builder.input_range.clone(),
                                    &output_address,
                                )
                                .await?
                            }
                            // Alias and NFT addresses can't be generated from a private key
                            _ => (0, false),
                        }
                    }
                    None => (0, false),
                };
                let input_signing_data = ClientMessageBuilder::create_input_signing_data(
                    message_builder.coin_type,
                    message_builder.account_index,
                    address_index,
                    internal,
                    &output_response,
                    output_address.to_bech32(&bech32_hrp),
                )?;
                input_signing_data_entrys.push(input_signing_data);
            }
        }
    }
    let selected_transaction_data = try_select_inputs(
        input_signing_data_entrys,
        message_builder.outputs.clone(),
        true,
        // todo allow custom remainder address
        None,
    )
    .await?;
    Ok(selected_transaction_data)
}
