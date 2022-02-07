// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    api::{
        address::search_address,
        types::{AddressIndexRecorder, OutputWrapper},
        ClientMessageBuilder, ADDRESS_GAP_RANGE,
    },
    node_api::indexer_api::query_parameters::QueryParameter,
    Error, Result,
};

use bee_message::{
    address::Address,
    input::{Input, UtxoInput, INPUT_COUNT_MAX},
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        AliasId, BasicOutputBuilder, NativeToken, Output, TokenId,
    },
};
use bee_rest_api::types::dtos::OutputDto;
use packable::PackableExt;

use primitive_types::U256;

use std::collections::{hash_map::Entry, HashMap, HashSet};

// Searches inputs for an amount which a user wants to spend, also checks that it doesn't create dust
pub(crate) async fn get_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    required_amount: u64,
    required_native_tokens: HashMap<TokenId, U256>,
    // todo: check if it's possible to automatically select alias/foundry/nft inputs if we have them on the outputs (by
    // their alias/nft id)
) -> Result<(Vec<Input>, Vec<Output>, Vec<AddressIndexRecorder>)> {
    let mut input_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    let mut outputs = Vec::new();
    let mut outputs_for_essence = Vec::new();
    let mut address_index_recorders = Vec::new();
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
                        address: str_address.clone(),
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

                        let address_index_record = ClientMessageBuilder::create_address_index_recorder(
                            account_index,
                            output_wrapper.address_index,
                            output_wrapper.internal,
                            &output_wrapper.output,
                            str_address.to_owned(),
                        )?;
                        address_index_recorders.push(address_index_record);
                        // println!("address_index_recorders : {:?}", address_index_recorders);
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
                                        Address::try_from_bech32(&output_wrapper.address)?,
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
                    address_index_recorders.clear();
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
    address_index_recorders
        .sort_unstable_by_key(|a| Address::try_from_bech32(&a.bech32_address).unwrap().pack_to_vec());
    let mut inputs_for_essence = Vec::new();
    for recoder in &address_index_recorders {
        inputs_for_essence.push(recoder.input.clone());
    }
    // println!("inputs_for_essence {:?}", inputs_for_essence);
    Ok((inputs_for_essence, outputs_for_essence, address_index_recorders))
}

// If custom inputs are provided we check if they are unspent, get the balance and search the address for it
pub(crate) async fn get_custom_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    inputs: &[UtxoInput],
    required_amount: u64,
    required_native_tokens: HashMap<TokenId, U256>,
    governance_transition: Option<HashSet<AliasId>>,
) -> Result<(Vec<Input>, Vec<Output>, Vec<AddressIndexRecorder>)> {
    let mut input_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    let mut inputs_for_essence = Vec::new();
    let mut outputs_for_essence = Vec::new();
    let mut address_index_recorders = Vec::new();
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

            let address_index_record = ClientMessageBuilder::create_address_index_recorder(
                account_index,
                address_index,
                internal,
                &output_response,
                output_address.to_bech32(&bech32_hrp),
            )?;
            address_index_recorders.push(address_index_record);
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
    address_index_recorders
        .sort_unstable_by_key(|a| Address::try_from_bech32(&a.bech32_address).unwrap().pack_to_vec());

    for recoder in &address_index_recorders {
        // println!("input address: {}", recoder.bech32_address);
        inputs_for_essence.push(recoder.input.clone());
    }

    Ok((inputs_for_essence, outputs_for_essence, address_index_recorders))
}

fn missing_native_tokens(
    inputs: &HashMap<TokenId, U256>,
    required: &HashMap<TokenId, U256>,
) -> Option<HashMap<TokenId, U256>> {
    let mut missing_native_tokens = HashMap::new();
    for (tokend_id, native_token_amount) in required {
        match inputs.get(tokend_id) {
            None => {
                missing_native_tokens.insert(*tokend_id, *native_token_amount);
            }
            Some(amount) => {
                if amount < native_token_amount {
                    missing_native_tokens.insert(*tokend_id, native_token_amount - amount);
                }
            }
        }
    }
    if missing_native_tokens.is_empty() {
        None
    } else {
        Some(missing_native_tokens)
    }
}

fn get_remainder_native_tokens(
    inputs: &HashMap<TokenId, U256>,
    required: &HashMap<TokenId, U256>,
) -> Option<HashMap<TokenId, U256>> {
    // inputs and required are switched
    missing_native_tokens(required, inputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bee_message::output::TokenId;

    #[test]
    fn nativ_token() {
        let token_id_bytes: [u8; 38] =
            hex::decode("08e68f7616cd4948efebc6a77c4f93aed770ac53860100000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap();
        let token_id = TokenId::from(token_id_bytes);

        // inputs == required
        let mut input_native_tokens = HashMap::new();
        input_native_tokens.insert(token_id, U256::from(50));
        let mut required_native_tokens = HashMap::new();
        required_native_tokens.insert(token_id, U256::from(50));

        assert_eq!(
            None,
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        );

        // no inputs
        let input_native_tokens = HashMap::new();
        let mut required_native_tokens = HashMap::new();
        required_native_tokens.insert(token_id, U256::from(50));

        assert_eq!(
            Some(required_native_tokens.clone()),
            missing_native_tokens(&input_native_tokens, &required_native_tokens)
        );

        // no inputs used
        let mut input_native_tokens = HashMap::new();
        input_native_tokens.insert(token_id, U256::from(50));
        let required_native_tokens = HashMap::new();

        assert_eq!(
            Some(input_native_tokens.clone()),
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        );

        // only a part of the inputs is used
        let mut input_native_tokens = HashMap::new();
        input_native_tokens.insert(token_id, U256::from(50));
        let mut required_native_tokens = HashMap::new();
        required_native_tokens.insert(token_id, U256::from(20));
        let mut remainder_native_tokens = HashMap::new();
        remainder_native_tokens.insert(token_id, U256::from(30));

        assert_eq!(
            Some(remainder_native_tokens),
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        );

        // more amount than required
        let mut input_native_tokens = HashMap::new();
        input_native_tokens.insert(token_id, U256::from(20));
        let mut required_native_tokens = HashMap::new();
        required_native_tokens.insert(token_id, U256::from(50));
        let mut remainder_native_tokens = HashMap::new();
        remainder_native_tokens.insert(token_id, U256::from(30));

        assert_eq!(
            Some(remainder_native_tokens),
            missing_native_tokens(&input_native_tokens, &required_native_tokens)
        );
        assert_eq!(
            None,
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        );
    }
}
