// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

use crate::{
    api::{address::search_address, ClientMessageBuilder, ADDRESS_GAP_RANGE},
    node_api::indexer_api::query_parameters::QueryParameter,
    signing::types::InputSigningData,
    Client, Error, Result,
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
use native_token_helpers::{get_remainder_native_tokens, missing_native_tokens};
use output_data::{get_accumulated_output_data, get_remainder};
use types::SelectedTransactionData;

/// Select inputs from provided inputs(OutputResponse), validate amounts and create remainder output if necessary
pub async fn try_select_inputs(
    client: &Client,
    mut inputs: Vec<InputSigningData>,
    mut outputs: Vec<Output>,
    force_use_all_inputs: bool,
    remainder_address: Option<Address>,
    get_utxo_chain_inputs: bool,
) -> Result<SelectedTransactionData> {
    let input_outputs = inputs
        .iter()
        .map(|i| Ok(Output::try_from(&i.output_response.output)?))
        .collect::<Result<Vec<Output>>>()?;

    // Validate and only create a remainder if necessary
    if force_use_all_inputs {
        if inputs.len() as u16 > INPUT_COUNT_MAX {
            return Err(Error::ConsolidationRequired(inputs.len().into()));
        }
        let remainder_output = get_remainder(client, &input_outputs, &outputs, remainder_address).await?;
        return Ok(SelectedTransactionData {
            inputs,
            outputs,
            remainder_output,
        });
    }
    // else only use inputs that are necessary for the required outputs

    // gets inputs for the utxo chains

    let required = get_accumulated_output_data(client, &outputs, get_utxo_chain_inputs).await?;
    let mut selected_input_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    let mut selected_input_amount = 0;
    let mut selected_inputs = Vec::new();
    let bech32_hrp = client.get_bech32_hrp().await?;

    // 1. get alias or nft inputs (because amount and native tokens of these outputs could be used)
    for (unlock_address, utxo_chain_input) in required.utxo_chains {
        // only add outputs that aren't already in the inputs
        if !inputs.iter().any(|e| {
            e.output_response.transaction_id == utxo_chain_input.transaction_id
                && e.output_response.output_index == utxo_chain_input.output_index
        }) {
            let output = Output::try_from(&utxo_chain_input.output)?;
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

            selected_inputs.push(InputSigningData {
                output_response: utxo_chain_input,
                chain: None,
                bech32_address: unlock_address.to_bech32(&bech32_hrp),
            });
        }
    }

    // 2. get native tokens (because amount of these outputs will also be used)
    if !required.native_tokens.is_empty() {
        for input_signing_data in &inputs {
            let output = Output::try_from(&input_signing_data.output_response.output)?;
            if let Some(output_native_tokens) = output.native_tokens() {
                for native_token in output_native_tokens {
                    // only check required tokens
                    if let Some(required_native_token_amount) = required.native_tokens.get(native_token.token_id()) {
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
    // check if we got all native tokens
    let selected_input_outputs = selected_inputs
        .iter()
        .map(|i| Ok(Output::try_from(&i.output_response.output)?))
        .collect::<Result<Vec<Output>>>()?;
    let input_native_tokens = get_accumulated_output_data(client, &selected_input_outputs, false)
        .await?
        .native_tokens;
    if let Some(native_token) = missing_native_tokens(&input_native_tokens, &required.native_tokens) {
        return Err(Error::NotEnoughNativeTokens(native_token));
    }
    let current_selected_input_len = selected_inputs.len() as u16;
    if current_selected_input_len > INPUT_COUNT_MAX {
        return Err(Error::ConsolidationRequired(current_selected_input_len.into()));
    }

    // 3. get amount (additional to possible amount from before)

    // todo first try to select inputs with an exact matching amount
    // Order input outputs descending, so that as few inputs as necessary are used
    inputs.sort_by(|l, r| {
        let output_1 = Output::try_from(&l.output_response.output).unwrap();
        let output_2 = Output::try_from(&r.output_response.output).unwrap();
        output_1.amount().cmp(&output_2.amount())
    });

    for input_signing_data in inputs
        .iter()
        // Max inputs is 128
        .take((INPUT_COUNT_MAX - current_selected_input_len).into())
    {
        if selected_input_amount < required.amount {
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

    // create remainder output if necessary
    let selected_input_outputs = selected_inputs
        .iter()
        .map(|i| Ok(Output::try_from(&i.output_response.output)?))
        .collect::<Result<Vec<Output>>>()?;
    // get_remainder also checks for amounts and returns an error if we don't have enough
    let remainder_output = get_remainder(client, &selected_input_outputs, &outputs, remainder_address).await?;
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

// Searches inputs for an amount which a user wants to spend, also checks that it doesn't create dust
pub(crate) async fn get_inputs(message_builder: &ClientMessageBuilder<'_>) -> Result<SelectedTransactionData> {
    let account_index = message_builder.account_index.unwrap_or(0);
    let mut gap_index = message_builder.initial_address_index.unwrap_or(0);
    let mut empty_address_count: u64 = 0;
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

                // todo handle error and continue searching
                let mut address_inputs = Vec::new();
                for output_response in address_outputs {
                    address_inputs.push(ClientMessageBuilder::create_input_signing_data(
                        account_index,
                        address_index,
                        *internal,
                        &output_response,
                        str_address.to_owned(),
                    )?);
                }
                let selected_transaction_data = match try_select_inputs(
                    message_builder.client,
                    address_inputs,
                    message_builder.outputs.clone(),
                    false,
                    // todo allow custom remainder address
                    None,
                    true,
                )
                .await
                {
                    Ok(r) => r,
                    // for these errors just try again in the next round with more addresses
                    Err(crate::Error::NotEnoughBalance(_, _)) => continue,
                    Err(crate::Error::NotEnoughNativeTokens(_)) => continue,
                    Err(crate::Error::NotEnoughBalanceForNativeTokenRemainder) => continue,
                    Err(crate::Error::ConsolidationRequired(_)) => continue,
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
            // todo get correct balance or return from try_select_inputs()
            return Err(Error::NotEnoughBalance(0, 0));
        }
    };

    Ok(selected_transaction_data)
}

// If custom inputs are provided we check if they are unspent, get the balance and search the address for it
pub(crate) async fn get_custom_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    governance_transition: Option<HashSet<AliasId>>,
) -> Result<SelectedTransactionData> {
    let mut input_signing_data_entrys = Vec::new();
    let account_index = message_builder.account_index.unwrap_or(0);

    if let Some(inputs) = &message_builder.inputs {
        for input in inputs {
            let output_response = message_builder.client.get_output(input.output_id()).await?;

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
                                account_index,
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
            let address_index_record = ClientMessageBuilder::create_input_signing_data(
                account_index,
                address_index,
                internal,
                &output_response,
                output_address.to_bech32(&bech32_hrp),
            )?;
            input_signing_data_entrys.push(address_index_record);
        }
    }

    let selected_transaction_data = try_select_inputs(
        message_builder.client,
        input_signing_data_entrys,
        message_builder.outputs.clone(),
        false,
        // todo allow custom remainder address
        None,
        // we only want to use provided inputs
        false,
    )
    .await?;
    Ok(selected_transaction_data)
}
