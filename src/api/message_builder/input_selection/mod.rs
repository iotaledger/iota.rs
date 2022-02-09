// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

use crate::{
    api::{address::search_address, types::OutputWrapper, ClientMessageBuilder, ADDRESS_GAP_RANGE},
    node_api::indexer_api::query_parameters::QueryParameter,
    signing::types::InputSigningData,
    Client, Error, Result,
};

use bee_message::{
    address::Address,
    input::{Input, UtxoInput, INPUT_COUNT_MAX},
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        AliasId, BasicOutputBuilder, NativeToken, Output, TokenId,
    },
    payload::transaction::TransactionId,
};
use bee_rest_api::types::dtos::OutputDto;
use packable::PackableExt;

use primitive_types::U256;

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    str::FromStr,
};

mod native_token_helpers;
mod output_data;
mod types;
use native_token_helpers::{get_remainder_native_tokens, missing_native_tokens};
use output_data::{get_accumulated_output_data, get_remainder};
use types::SelectedTransactionData;

/// Select inputs from provided inputs(OutputResponse), validate amounts and create remainder output if necessary
pub async fn try_select_inputs(
    client: &Client,
    mut inputs: Vec<InputSigningData>,
    mut outputs: Vec<Output>,
    force_use_all_inputs: bool,
) -> Result<SelectedTransactionData> {
    let mut remainder_output = None;

    // Validate and only create a remainder if necessary
    if force_use_all_inputs {
        let input_outputs = inputs
            .iter()
            .map(|i| Ok(Output::try_from(&i.output_response.output)?))
            .collect::<Result<Vec<Output>>>()?;
        remainder_output = get_remainder(client, &input_outputs, &outputs, None).await?;
        return Ok(SelectedTransactionData {
            inputs,
            outputs,
            remainder_output,
        });
    }

    // only use inputs that are necessary for the required outputs
    let required = get_accumulated_output_data(client, &outputs).await?;

    let mut input_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    let mut total_already_spent = 0;
    let mut selected_inputs = Vec::new();
    let bech32_hrp = client.get_bech32_hrp().await?;

    // 1. get alias or nft inputs (because amount and native tokens of these outputs could be used)
    for (unlock_address, utxo_chain_input) in required.utxo_chains {
        let output = Output::try_from(&utxo_chain_input.output)?;
        total_already_spent += output.amount();
        if let Some(output_native_tokens) = output.native_tokens() {
            for native_token in output_native_tokens {
                match input_native_tokens.entry(*native_token.token_id()) {
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
    // governance transactions are separated because we have a different unlock address for them
    // not anymore?

    // 2. get native tokens (because amount of these outputs will also be used)
    // 3. get amount (additional to possible amount from before)

    // todo first try to select inputs with an exact matching amount
    // Order input outputs descending, so that as few inputs as necessary are used,
    // might not be true with native tokens
    inputs.sort_by(|l, r| {
        let output_1 = Output::try_from(&l.output_response.output).unwrap();
        let output_2 = Output::try_from(&r.output_response.output).unwrap();
        output_1.amount().cmp(&output_2.amount())
    });

    // check for amount and native tokens
    if total_already_spent >= required.amount {
        // check for native tokens/ break
    }
    for (_offset, input_signing_data) in inputs
        .iter()
        // Max inputs is 128
        .take(INPUT_COUNT_MAX.into())
        .enumerate()
    {
        // todo: check for amount and native tokens
        let output = Output::try_from(&input_signing_data.output_response.output)?;
        total_already_spent += output.amount();
        if let Some(output_native_tokens) = output.native_tokens() {
            for native_token in output_native_tokens {
                match input_native_tokens.entry(*native_token.token_id()) {
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

    // todo validate amounts, generate remainder
    // if remainder{
    //     outputs.push(remainder);
    //     remainder_output.replace(remainder);
    // }
    Ok(SelectedTransactionData {
        inputs: vec![],
        outputs,
        remainder_output,
    })
}

// Searches inputs for an amount which a user wants to spend, also checks that it doesn't create dust
pub(crate) async fn get_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    required_amount: u64,
    required_native_tokens: HashMap<TokenId, U256>,
    // todo: check if it's possible to automatically select alias/foundry/nft inputs if we have them on the outputs (by
    // their alias/nft id)
) -> Result<(Vec<Input>, Vec<Output>, Vec<InputSigningData>)> {
    let mut input_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    let mut outputs = Vec::new();
    let mut outputs_for_essence = Vec::new();
    let mut input_signing_data_entrys = Vec::new();
    let mut total_already_spent = 0;
    let account_index = message_builder.account_index.unwrap_or(0);
    let mut gap_index = message_builder.initial_address_index.unwrap_or(0);
    let mut empty_address_count: u64 = 0;
    'input_selection: loop {
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
            }

            // We store output responses locally in outputs and after each output we sort
            // them and try to get enough inputs for the transaction, so we don't request more
            // outputs than we need
            for output_response in address_outputs.into_iter() {
                if !output_response.is_spent {
                    let output = Output::try_from(&output_response.output)?;
                    let amount = output.amount();

                    let output_wrapper = OutputWrapper {
                        output: output_response,
                        address_index,
                        internal: *internal,
                        amount,
                        bech32_address: str_address.clone(),
                    };
                    match output_wrapper.output.output {
                        OutputDto::Basic(_) => outputs.push(output_wrapper),
                        OutputDto::Treasury(_) => {}
                        // todo: add other outputs
                        _ => unimplemented!(),
                    };

                    // Order outputs descending, so that as few inputs as necessary are used
                    outputs.sort_by(|l, r| r.amount.cmp(&l.amount));

                    for (_offset, output_wrapper) in outputs
                        .iter()
                        // Max inputs is 128
                        .take(INPUT_COUNT_MAX.into())
                        .enumerate()
                    {
                        total_already_spent += output_wrapper.amount;
                        let output = Output::try_from(&output_wrapper.output.output)?;
                        if let Some(output_native_tokens) = output.native_tokens() {
                            for native_token in output_native_tokens {
                                match input_native_tokens.entry(*native_token.token_id()) {
                                    Entry::Vacant(e) => {
                                        e.insert(*native_token.amount());
                                    }
                                    Entry::Occupied(mut e) => {
                                        *e.get_mut() += *native_token.amount();
                                    }
                                }
                            }
                        }

                        let address_index_record = ClientMessageBuilder::create_input_signing_data(
                            account_index,
                            output_wrapper.address_index,
                            output_wrapper.internal,
                            &output_wrapper.output,
                            str_address.to_owned(),
                        )?;
                        input_signing_data_entrys.push(address_index_record);
                        // println!("input_signing_data_entrys : {:?}", input_signing_data_entrys);
                        // println!("input_native_tokens: {:?}", input_native_tokens);
                        // println!("required_native_tokens: {:?}", required_native_tokens);
                        // println!("amount reached: {:?}", native_tokens_amount_reached(&input_native_tokens,
                        // &required_native_tokens)); Break if we have enough funds and don't
                        // create dust for the remainder
                        if total_already_spent >= required_amount
                            && missing_native_tokens(&input_native_tokens, &required_native_tokens).is_none()
                        {
                            let remaining_balance = total_already_spent - required_amount;
                            // Output possible remaining tokens back to the original address
                            if remaining_balance != 0 {
                                let mut remainder_output_builder = BasicOutputBuilder::new(remaining_balance)?
                                    .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                                        Address::try_from_bech32(&output_wrapper.bech32_address)?,
                                    )));
                                if let Some(remainder_native_tokens) =
                                    get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
                                {
                                    for (token_id, amount) in remainder_native_tokens {
                                        remainder_output_builder = remainder_output_builder
                                            .add_native_token(NativeToken::new(token_id, amount)?);
                                    }
                                }
                                outputs_for_essence.push(Output::Basic(remainder_output_builder.finish()?));
                            }
                            // don't break if we have remaining native tokens, but no remaining balance for the dust
                            // deposit
                            if get_remainder_native_tokens(&input_native_tokens, &required_native_tokens).is_some() {
                                if remaining_balance > 0 {
                                    break 'input_selection;
                                }
                            } else {
                                break 'input_selection;
                            }
                        }
                    }
                    // We need to cleare all gathered records if we haven't reached the total amount we need in this
                    // iteration.
                    input_native_tokens.clear();
                    outputs_for_essence.clear();
                    input_signing_data_entrys.clear();
                    total_already_spent = 0;
                }
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
            let inputs_balance = outputs.iter().fold(0, |acc, output| acc + output.amount);
            let inputs_amount = outputs.len();
            if inputs_balance >= required_amount && inputs_amount > INPUT_COUNT_MAX.into() {
                return Err(Error::ConsolidationRequired(inputs_amount));
            } else if inputs_balance > required_amount {
                // todo check dust protection
                // return Err(Error::DustError(format!(
                //     "Transaction would create a dust output with {}i",
                //     inputs_balance - required_amount
                // )));
            } else {
                if let Some(missing_native_tokens) =
                    missing_native_tokens(&input_native_tokens, &required_native_tokens)
                {
                    return Err(Error::NotEnoughNativeTokens(missing_native_tokens));
                }
                return Err(Error::NotEnoughBalance(inputs_balance, required_amount));
            }
        }
    }
    // sort inputs so ed25519 address unlocks will be first, safe to unwrap since we encoded it before
    input_signing_data_entrys
        .sort_unstable_by_key(|a| Address::try_from_bech32(&a.bech32_address).unwrap().pack_to_vec());
    let mut inputs_for_essence = Vec::new();
    for recoder in &input_signing_data_entrys {
        let input = Input::Utxo(UtxoInput::new(
            TransactionId::from_str(&recoder.output_response.transaction_id)?,
            recoder.output_response.output_index,
        )?);
        inputs_for_essence.push(input);
    }
    // println!("inputs_for_essence {:?}", inputs_for_essence);
    Ok((inputs_for_essence, outputs_for_essence, input_signing_data_entrys))
}

// If custom inputs are provided we check if they are unspent, get the balance and search the address for it
pub(crate) async fn get_custom_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    inputs: &[UtxoInput],
    required_amount: u64,
    required_native_tokens: HashMap<TokenId, U256>,
    governance_transition: Option<HashSet<AliasId>>,
) -> Result<(Vec<Input>, Vec<Output>, Vec<InputSigningData>)> {
    let mut input_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    let mut inputs_for_essence = Vec::new();
    let mut outputs_for_essence = Vec::new();
    let mut input_signing_data_entrys = Vec::new();
    let mut remainder_address_balance: (Option<Address>, u64) = (None, 0);
    let mut total_already_spent = 0;
    let account_index = message_builder.account_index.unwrap_or(0);
    for input in inputs {
        let output_response = message_builder.client.get_output(input.output_id()).await?;
        // Only add unspent outputs
        if !output_response.is_spent {
            let (output_amount, output_address) = ClientMessageBuilder::get_output_amount_and_address(
                &output_response.output,
                governance_transition.clone(),
            )?;
            let output = Output::try_from(&output_response.output)?;

            total_already_spent += output_amount;
            if let Some(output_native_tokens) = output.native_tokens() {
                for native_token in output_native_tokens {
                    match input_native_tokens.entry(*native_token.token_id()) {
                        Entry::Vacant(e) => {
                            e.insert(*native_token.amount());
                        }
                        Entry::Occupied(mut e) => {
                            *e.get_mut() += *native_token.amount();
                        }
                    }
                }
            }

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
            // Output the remaining tokens back to the original address
            if total_already_spent > required_amount {
                let remaining_balance = total_already_spent - required_amount;
                // Keep track of remaining balance, we don't add an output here, because we could have
                // multiple inputs from the same address, which would create multiple outputs with the
                // same address, which is not allowed
                remainder_address_balance = (Some(output_address), remaining_balance);
            }
        }
    }
    // Add output from remaining balance of custom inputs if necessary
    if let Some(address) = remainder_address_balance.0 {
        let mut remainder_output_builder = BasicOutputBuilder::new(remainder_address_balance.1)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)));
        if let Some(remainder_native_tokens) =
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        {
            for (token_id, amount) in remainder_native_tokens {
                remainder_output_builder =
                    remainder_output_builder.add_native_token(NativeToken::new(token_id, amount)?);
            }
        }
        outputs_for_essence.push(Output::Basic(remainder_output_builder.finish()?));
    } else if get_remainder_native_tokens(&input_native_tokens, &required_native_tokens).is_some() {
        return Err(Error::NotEnoughBalanceForNativeTokenRemainder);
    }

    if total_already_spent < required_amount {
        return Err(Error::NotEnoughBalance(total_already_spent, required_amount));
    }
    if let Some(missing_native_tokens) = missing_native_tokens(&input_native_tokens, &required_native_tokens) {
        return Err(Error::NotEnoughNativeTokens(missing_native_tokens));
    }

    // sort inputs by address type (ed25519 addresses will be first because of the type byte), so ed25519 signature
    // unlock blocks will be first and can be referenced by alias and nft unlock blocks
    input_signing_data_entrys
        .sort_unstable_by_key(|a| Address::try_from_bech32(&a.bech32_address).unwrap().pack_to_vec());

    for recoder in &input_signing_data_entrys {
        // println!("input address: {}", recoder.bech32_address);
        let input = Input::Utxo(UtxoInput::new(
            TransactionId::from_str(&recoder.output_response.transaction_id)?,
            recoder.output_response.output_index,
        )?);
        inputs_for_essence.push(input);
    }

    Ok((inputs_for_essence, outputs_for_essence, input_signing_data_entrys))
}
