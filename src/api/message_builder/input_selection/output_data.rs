// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    api::{
        input_selection::{
            get_minted_and_burned_native_tokens, get_remainder_native_tokens, types::AccumulatedOutputAmounts,
        },
        message_builder::ClientMessageBuilder,
        search_address,
    },
    constants::HD_WALLET_TYPE,
    signing::types::InputSigningData,
    Error, Result,
};

use bee_message::{
    address::Address,
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, NativeToken, Output, TokenId,
    },
};
use bee_rest_api::types::{
    dtos::{OutputDto, UnlockConditionDto},
    responses::OutputResponse,
};
use crypto::keys::slip10::Chain;

use primitive_types::U256;

use std::collections::{hash_map::Entry, HashMap};

// Get a remainder with amount and native tokens if necessary, if no remainder_address is provided it will be selected
// from the inputs, also validates the amounts
pub(crate) async fn get_remainder(
    inputs: &[Output],
    outputs: &[Output],
    remainder_address: Option<Address>,
) -> Result<Option<Output>> {
    let mut remainder_output = None;
    let input_data = get_accumulated_output_amounts(inputs).await?;
    let output_data = get_accumulated_output_amounts(outputs).await?;
    // Get minted native tokens
    let (minted_native_tokens, burned_native_tokens) = get_minted_and_burned_native_tokens(inputs, outputs)?;

    // check amount first
    if input_data.amount < output_data.amount {
        return Err(Error::NotEnoughBalance(input_data.amount, output_data.amount));
    }
    let remainder_amount = input_data.amount - output_data.amount;

    // add minted tokens
    let mut input_native_tokens = input_data.native_tokens;
    for (token_id, minted_native_token_amount) in minted_native_tokens {
        match input_native_tokens.entry(token_id) {
            Entry::Vacant(e) => {
                e.insert(minted_native_token_amount);
            }
            Entry::Occupied(mut e) => {
                *e.get_mut() += minted_native_token_amount;
            }
        }
    }

    // add burned tokens
    let mut output_native_tokens: HashMap<TokenId, U256> = output_data.native_tokens;
    // add burned native tokens as outputs, because we need to have this amount in the inputs
    for (tokend_id, burned_amount) in burned_native_tokens {
        match output_native_tokens.entry(tokend_id) {
            Entry::Vacant(e) => {
                e.insert(burned_amount);
            }
            Entry::Occupied(mut e) => {
                *e.get_mut() += burned_amount;
            }
        }
    }

    let native_token_remainder = get_remainder_native_tokens(&input_native_tokens, &output_native_tokens);
    // Output possible remaining tokens back to the original address
    if remainder_amount > 0 {
        let remainder_addr = match remainder_address {
            Some(address) => address,
            // get address from an input, by default we only allow ed25519 addresses as remainder, because then we're
            // sure that the sender can access it
            None => {
                let mut address = None;
                for input in inputs {
                    if let Output::Basic(basic_output) = input {
                        for unlock_condition in basic_output.unlock_conditions() {
                            if let UnlockCondition::Address(address_unlock_condition) = unlock_condition {
                                address.replace(address_unlock_condition.address());
                                break;
                            }
                        }
                    }
                }
                match address {
                    Some(addr) => *addr,
                    None => return Err(Error::MissingInputWithEd25519UnlockCondition),
                }
            }
        };

        let mut remainder_output_builder = BasicOutputBuilder::new(remainder_amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_addr)));
        if let Some(remainder_native_tokens) = native_token_remainder {
            for (token_id, amount) in remainder_native_tokens {
                remainder_output_builder =
                    remainder_output_builder.add_native_token(NativeToken::new(token_id, amount)?);
            }
        }
        remainder_output.replace(Output::Basic(remainder_output_builder.finish()?));
    } else {
        // if we have remaining native tokens, but no amount left, then we can't create this transaction
        if native_token_remainder.is_some() {
            return Err(Error::NotEnoughBalanceForNativeTokenRemainder);
        }
    }

    Ok(remainder_output)
}

// Calculate total accumulated amounts from the outputs
pub(crate) async fn get_accumulated_output_amounts(outputs: &[Output]) -> Result<AccumulatedOutputAmounts> {
    // Calculate the total tokens to spend
    let mut required_amount: u64 = 0;
    let mut required_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    for output in outputs {
        required_amount += output.amount();
        if let Some(output_native_tokens) = output.native_tokens() {
            for native_token in output_native_tokens {
                match required_native_tokens.entry(*native_token.token_id()) {
                    Entry::Vacant(e) => {
                        e.insert(*native_token.amount());
                    }
                    Entry::Occupied(mut e) => {
                        *e.get_mut() += *native_token.amount();
                    }
                }
            }
        }
    }
    Ok(AccumulatedOutputAmounts {
        amount: required_amount,
        native_tokens: required_native_tokens,
    })
}

/// Get inputs for utxo chains
pub(crate) async fn get_utxo_chains_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    let client = message_builder.client;
    let bech32_hrp = client.get_bech32_hrp().await?;
    let mut utxo_chains: Vec<(Address, OutputResponse)> = Vec::new();
    for output in outputs {
        match output {
            Output::Alias(alias_output) => {
                // if the state_index is [0u8; 20] then there can't be a previous output and it can also not be a
                // governance transition
                if alias_output.alias_id().as_ref() != [0u8; 20] {
                    // Check if the transaction is a governance_transition, by checking if the new index is the same
                    // as the previous index
                    let output_ids = client.alias_output_ids(*alias_output.alias_id()).await?;
                    let outputs = client.get_outputs(output_ids).await?;
                    for output_response in outputs {
                        if let OutputDto::Alias(output) = &output_response.output {
                            if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                                for unlock_condition in &alias_output_dto.unlock_conditions {
                                    // A governance transition is identified by an unchanged State Index in next
                                    // state.
                                    if alias_output.state_index() == output.state_index {
                                        if let UnlockConditionDto::GovernorAddress(governor_unlock_condition_dto) =
                                            unlock_condition
                                        {
                                            let address = Address::try_from(&governor_unlock_condition_dto.address)?;
                                            utxo_chains.push((address, output_response.clone()));
                                        }
                                    } else if let UnlockConditionDto::StateControllerAddress(
                                        state_controller_unlock_condition_dto,
                                    ) = unlock_condition
                                    {
                                        let address =
                                            Address::try_from(&state_controller_unlock_condition_dto.address)?;
                                        utxo_chains.push((address, output_response.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Output::Nft(nft_output) => {
                // If the id is [0u8; 20] then this output creates it and we can't have a previous output
                if nft_output.nft_id().as_ref() != [0u8; 20] {
                    let output_ids = client.nft_output_ids(*nft_output.nft_id()).await?;
                    let outputs = client.get_outputs(output_ids).await?;
                    for output_response in outputs {
                        if let OutputDto::Nft(nft_output_dto) = &output_response.output {
                            for unlock_condition in &nft_output_dto.unlock_conditions {
                                if let UnlockConditionDto::Address(address_unlock_condition_dto) = unlock_condition {
                                    let address = Address::try_from(&address_unlock_condition_dto.address)?;
                                    utxo_chains.push((address, output_response.clone()));
                                }
                            }
                        }
                    }
                }
            }
            Output::Foundry(foundry_output) => {
                // if it's the first foundry output, then we can't have it as input
                if let Ok(output_ids) = client.foundry_output_ids(foundry_output.id()).await {
                    let outputs = client.get_outputs(output_ids).await?;
                    for output_response in outputs {
                        if let OutputDto::Foundry(foundry_output) = &output_response.output {
                            for unlock_condition in &foundry_output.unlock_conditions {
                                if let UnlockConditionDto::ImmutableAliasAddress(
                                    immutable_alias_address_unlock_condition_dto,
                                ) = unlock_condition
                                {
                                    let address =
                                        Address::try_from(&immutable_alias_address_unlock_condition_dto.address)?;
                                    utxo_chains.push((address, output_response.clone()));
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let mut utxo_chain_inputs = Vec::new();
    for (unlock_address, output_response) in utxo_chains {
        let (address_index, internal) = match message_builder.signer {
            Some(signer) => {
                match unlock_address {
                    Address::Ed25519(_) => {
                        search_address(
                            signer,
                            &bech32_hrp,
                            message_builder.coin_type,
                            message_builder.account_index,
                            message_builder.input_range.clone(),
                            &unlock_address,
                        )
                        .await?
                    }
                    // Alias and NFT addresses can't be generated from a private key
                    _ => (0, false),
                }
            }
            None => (0, false),
        };

        utxo_chain_inputs.push(InputSigningData {
            output_response: output_response.clone(),
            chain: Some(Chain::from_u32_hardened(vec![
                HD_WALLET_TYPE,
                message_builder.coin_type,
                message_builder.account_index,
                internal as u32,
                address_index,
            ])),
            bech32_address: unlock_address.to_bech32(&bech32_hrp),
        });
    }

    Ok(utxo_chain_inputs)
}
