// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_api_types::responses::OutputResponse;
use bee_block::{
    address::Address,
    output::{dto::OutputDto, unlock_condition::dto::UnlockConditionDto, NativeTokensBuilder, NftOutput, Output},
};
use crypto::keys::slip10::Chain;

use crate::{
    api::{
        block_builder::ClientBlockBuilder,
        input_selection::{get_minted_and_melted_native_tokens, types::AccumulatedOutputAmounts},
        search_address,
    },
    constants::HD_WALLET_TYPE,
    secret::types::{InputSigningData, OutputMetadata},
    Result,
};

// Calculate required accumulated amounts from the outputs, considers also minted and melted native tokens
pub(crate) fn get_accumulated_output_amounts<'a>(
    inputs: &(impl Iterator<Item = &'a Output> + Clone),
    outputs: impl Iterator<Item = &'a Output> + Clone,
) -> Result<AccumulatedOutputAmounts> {
    // Calculate the total tokens to spend
    let mut required_amount: u64 = 0;
    let mut required_native_tokens = NativeTokensBuilder::new();

    for output in outputs.clone() {
        required_amount += output.amount();

        if let Some(output_native_tokens) = output.native_tokens() {
            required_native_tokens.add_native_tokens(output_native_tokens.clone())?;
        }
    }

    // check if a foundry mints or melts native tokens
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(inputs, outputs)?;
    // add melted native tokens as outputs, because we need to have this amount in the inputs
    required_native_tokens.merge(melted_native_tokens)?;

    Ok(AccumulatedOutputAmounts {
        minted_native_tokens,
        amount: required_amount,
        native_tokens: required_native_tokens,
    })
}

/// Get inputs for utxo chains
pub(crate) async fn get_utxo_chains_inputs(
    block_builder: &ClientBlockBuilder<'_>,
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    log::debug!("[get_utxo_chains_inputs]");
    let client = block_builder.client;
    let bech32_hrp = client.get_bech32_hrp().await?;
    let mut utxo_chains: Vec<(Address, OutputResponse)> = Vec::new();
    for output in outputs {
        match output {
            Output::Alias(alias_output) => {
                // if the state_index is null then there can't be a previous output and it can also not be a
                // governance transition
                if !alias_output.alias_id().is_null() {
                    // Check if the transaction is a governance_transition, by checking if the new index is the same
                    // as the previous index
                    let output_id = client.alias_output_id(*alias_output.alias_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Alias(output) = &output_response.output {
                        for unlock_condition in &output.unlock_conditions {
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
                                let address = Address::try_from(&state_controller_unlock_condition_dto.address)?;
                                utxo_chains.push((address, output_response.clone()));
                            }
                        }
                    }
                }
            }
            Output::Nft(nft_output) => {
                // If the id is null then this output creates it and we can't have a previous output
                if !nft_output.nft_id().is_null() {
                    let output_id = client.nft_output_id(*nft_output.nft_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Nft(nft_output) = &output_response.output {
                        let nft_output = NftOutput::try_from(nft_output)?;
                        let output_address = nft_output
                            .unlock_conditions()
                            .address()
                            .expect("Nft output needs to have an address unlock condition")
                            .address();

                        let local_time = block_builder.client.get_time_checked().await?;

                        let unlock_address = nft_output
                            .unlock_conditions()
                            .locked_address(output_address, local_time);

                        utxo_chains.push((*unlock_address, output_response.clone()));
                    }
                }
            }
            Output::Foundry(foundry_output) => {
                // if it's the first foundry output, then we can't have it as input
                if let Ok(output_id) = client.foundry_output_id(foundry_output.id()).await {
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Foundry(foundry_output) = &output_response.output {
                        for unlock_condition in &foundry_output.unlock_conditions {
                            if let UnlockConditionDto::ImmutableAliasAddress(
                                immutable_alias_address_unlock_condition_dto,
                            ) = unlock_condition
                            {
                                let address = Address::try_from(&immutable_alias_address_unlock_condition_dto.address)?;
                                utxo_chains.push((address, output_response.clone()));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Get recursive owned alias or nft outputs
    let mut unprocessed_alias_nft_addresses = std::collections::HashSet::new();

    for (unlock_address, _output_response) in utxo_chains.clone() {
        unprocessed_alias_nft_addresses.insert(unlock_address);
    }

    while !unprocessed_alias_nft_addresses.is_empty() {
        for (unlock_address, _output_response) in utxo_chains.clone() {
            if !unprocessed_alias_nft_addresses.contains(&unlock_address) {
                continue;
            }
            match unlock_address {
                Address::Alias(address) => {
                    let output_id = client.alias_output_id(*address.alias_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                        for unlock_condition in &alias_output_dto.unlock_conditions {
                            // State transition if we add them to inputs
                            if let UnlockConditionDto::StateControllerAddress(state_controller_unlock_condition_dto) =
                                unlock_condition
                            {
                                let address = Address::try_from(&state_controller_unlock_condition_dto.address)?;
                                // Add address to unprocessed_alias_nft_addresses so we get the required output there
                                // also
                                match address {
                                    Address::Alias(_) | Address::Nft(_) => {
                                        unprocessed_alias_nft_addresses.insert(address);
                                    }
                                    _ => {}
                                }
                                utxo_chains.push((address, output_response.clone()));
                            }
                        }
                    }
                }
                Address::Nft(address) => {
                    let output_id = client.nft_output_id(*address.nft_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Nft(nft_output) = &output_response.output {
                        let nft_output = NftOutput::try_from(nft_output)?;
                        let output_address = nft_output
                            .unlock_conditions()
                            .address()
                            .expect("Nft output needs to have an address unlock condition")
                            .address();

                        let local_time = block_builder.client.get_time_checked().await?;

                        let unlock_address = nft_output
                            .unlock_conditions()
                            .locked_address(output_address, local_time);
                        // Add address to unprocessed_alias_nft_addresses so we get the required output there also
                        match unlock_address {
                            Address::Alias(_) | Address::Nft(_) => {
                                unprocessed_alias_nft_addresses.insert(*unlock_address);
                            }
                            _ => {}
                        }
                        utxo_chains.push((*unlock_address, output_response.clone()));
                    }
                }
                _ => {}
            }
            // Remove processed addresses
            unprocessed_alias_nft_addresses.remove(&unlock_address);
        }
    }

    let mut utxo_chain_inputs = Vec::new();
    for (unlock_address, output_response) in utxo_chains {
        let (address_index, internal) = match block_builder.secret_manager {
            Some(secret_manager) => {
                match unlock_address {
                    Address::Ed25519(_) => {
                        search_address(
                            secret_manager,
                            &bech32_hrp,
                            block_builder.coin_type,
                            block_builder.account_index,
                            block_builder.input_range.clone(),
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
            output: Output::try_from(&output_response.output)?,
            output_metadata: OutputMetadata::try_from(&output_response.metadata)?,
            chain: Some(Chain::from_u32_hardened(vec![
                HD_WALLET_TYPE,
                block_builder.coin_type,
                block_builder.account_index,
                internal as u32,
                address_index,
            ])),
            bech32_address: unlock_address.to_bech32(&bech32_hrp),
        });
    }

    Ok(utxo_chain_inputs)
}
