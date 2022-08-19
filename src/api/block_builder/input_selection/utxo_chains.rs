// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! input selection for utxo chains

use bee_api_types::responses::OutputResponse;
use bee_block::{
    address::Address,
    output::{dto::OutputDto, unlock_condition::dto::UnlockConditionDto, NftOutput, Output},
};
use crypto::keys::slip10::Chain;

use crate::{
    api::{block_builder::ClientBlockBuilder, search_address},
    constants::HD_WALLET_TYPE,
    secret::types::{InputSigningData, OutputMetadata},
    Client, Result,
};

impl<'a> ClientBlockBuilder<'a> {
    /// Get inputs for utxo chains
    pub(crate) async fn get_utxo_chains_inputs(&self, outputs: &[Output]) -> Result<Vec<InputSigningData>> {
        log::debug!("[get_utxo_chains_inputs]");
        let client = self.client;
        let bech32_hrp = client.get_bech32_hrp().await?;
        let current_time = self.client.get_time_checked().await?;

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

                            let unlock_address = nft_output
                                .unlock_conditions()
                                .locked_address(output_address, current_time);

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
                                    let address =
                                        Address::try_from(&immutable_alias_address_unlock_condition_dto.address)?;
                                    utxo_chains.push((address, output_response.clone()));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Get recursively owned alias or nft outputs
        get_alias_and_nft_outputs_recursively(self.client, &mut utxo_chains).await?;

        let mut utxo_chain_inputs = Vec::new();
        for (unlock_address, output_response) in utxo_chains {
            let (address_index, internal) = match self.secret_manager {
                Some(secret_manager) => {
                    match unlock_address {
                        Address::Ed25519(_) => {
                            search_address(
                                secret_manager,
                                &bech32_hrp,
                                self.coin_type,
                                self.account_index,
                                self.input_range.clone(),
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
                    self.coin_type,
                    self.account_index,
                    internal as u32,
                    address_index,
                ])),
                bech32_address: unlock_address.to_bech32(&bech32_hrp),
            });
        }

        Ok(utxo_chain_inputs)
    }
}

/// Get recursively owned alias and nft outputs and add them to the utxo_chains
pub(crate) async fn get_alias_and_nft_outputs_recursively(
    client: &Client,
    utxo_chains: &mut Vec<(Address, OutputResponse)>,
) -> Result<()> {
    log::debug!("[get_alias_and_nft_outputs_recursively]");
    let current_time = client.get_time_checked().await?;

    let mut unprocessed_alias_nft_addresses = std::collections::HashSet::new();

    for (unlock_address, _output_response) in utxo_chains.clone() {
        unprocessed_alias_nft_addresses.insert(unlock_address);
    }

    while !unprocessed_alias_nft_addresses.is_empty() {
        for (unlock_address, _output_response) in utxo_chains.clone() {
            // Skip already processed addresses
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
                                if address.is_alias() || address.is_nft() {
                                    unprocessed_alias_nft_addresses.insert(address);
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

                        let unlock_address = nft_output
                            .unlock_conditions()
                            .locked_address(output_address, current_time);
                        // Add address to unprocessed_alias_nft_addresses so we get the required output there also
                        if unlock_address.is_alias() || unlock_address.is_nft() {
                            unprocessed_alias_nft_addresses.insert(*unlock_address);
                        }
                        utxo_chains.push((*unlock_address, output_response.clone()));
                    }
                }
                _ => {} // do nothing with ed25519 addresses
            }
            // Remove processed addresses
            unprocessed_alias_nft_addresses.remove(&unlock_address);
        }
    }

    Ok(())
}
