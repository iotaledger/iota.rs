// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! input selection for utxo chains

use std::str::FromStr;

use bee_api_types::responses::OutputResponse;
use bee_block::{
    address::{Address, AliasAddress, NftAddress},
    output::{dto::OutputDto, AliasOutput, FoundryOutput, NftOutput, Output, OutputId},
    payload::transaction::TransactionId,
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
    pub(crate) async fn get_utxo_chains_inputs(
        &self,
        outputs: impl Iterator<Item = &'a Output> + Clone,
    ) -> Result<Vec<InputSigningData>> {
        log::debug!("[get_utxo_chains_inputs]");
        let client = self.client;
        let bech32_hrp = client.get_bech32_hrp().await?;
        let current_time = self.client.get_time_checked().await?;

        let mut utxo_chains: Vec<(Address, OutputResponse)> = Vec::new();
        for output in outputs {
            match output {
                Output::Alias(alias_output) => {
                    // if the alias id is null then there can't be a previous output and it can also not be a
                    // governance transition
                    if !alias_output.alias_id().is_null() {
                        // Check if the transaction is a governance_transition, by checking if the new index is the same
                        // as the previous index
                        let output_id = client.alias_output_id(*alias_output.alias_id()).await?;
                        let output_response = client.get_output(&output_id).await?;
                        if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                            let alias_output = AliasOutput::try_from(alias_output_dto)?;

                            // A governance transition is identified by an unchanged State Index in next
                            // state.
                            if alias_output.state_index() == alias_output.state_index() {
                                utxo_chains.push((*alias_output.governor_address(), output_response));
                            } else {
                                utxo_chains.push((*alias_output.state_controller_address(), output_response));
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

                            let unlock_address = nft_output
                                .unlock_conditions()
                                .locked_address(nft_output.address(), current_time);

                            utxo_chains.push((*unlock_address, output_response));
                        }
                    }
                }
                Output::Foundry(foundry_output) => {
                    // if it's the first foundry output, then we can't have it as input
                    if let Ok(output_id) = client.foundry_output_id(foundry_output.id()).await {
                        let output_response = client.get_output(&output_id).await?;
                        if let OutputDto::Foundry(foundry_output_dto) = &output_response.output {
                            let foundry_output = FoundryOutput::try_from(foundry_output_dto)?;
                            utxo_chains.push((Address::Alias(*foundry_output.alias_address()), output_response));
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
            let address_index_internal = match self.secret_manager {
                Some(secret_manager) => {
                    match unlock_address {
                        Address::Ed25519(_) => Some(
                            search_address(
                                secret_manager,
                                &bech32_hrp,
                                self.coin_type,
                                self.account_index,
                                self.input_range.clone(),
                                &unlock_address,
                            )
                            .await?,
                        ),
                        // Alias and NFT addresses can't be generated from a private key
                        _ => None,
                    }
                }
                // Assuming default for offline signing
                None => Some((0, false)),
            };

            utxo_chain_inputs.push(InputSigningData {
                output: Output::try_from(&output_response.output)?,
                output_metadata: OutputMetadata::try_from(&output_response.metadata)?,
                chain: address_index_internal.map(|(address_index, internal)| {
                    Chain::from_u32_hardened(vec![
                        HD_WALLET_TYPE,
                        self.coin_type,
                        self.account_index,
                        internal as u32,
                        address_index,
                    ])
                }),
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

    let mut processed_alias_nft_addresses = std::collections::HashSet::new();

    // Add addresses for alias and nft outputs we already have
    for (_unlock_address, output_response) in utxo_chains.iter() {
        let output_id = OutputId::new(
            TransactionId::from_str(&output_response.metadata.transaction_id)?,
            output_response.metadata.output_index,
        )?;

        match Output::try_from(&output_response.output)? {
            Output::Alias(alias_output) => {
                processed_alias_nft_addresses.insert(Address::Alias(AliasAddress::new(
                    alias_output.alias_id().or_from_output_id(output_id),
                )));
            }
            Output::Nft(nft_output) => {
                processed_alias_nft_addresses.insert(Address::Nft(NftAddress::new(
                    nft_output.nft_id().or_from_output_id(output_id),
                )));
            }
            _ => {}
        }
    }

    let mut processed_utxo_chains = Vec::new();

    // Make the outputs response optional, because we don't know it yet for new required outputs
    let mut utxo_chain_optional_response: Vec<(Address, Option<OutputResponse>)> =
        utxo_chains.iter_mut().map(|(a, o)| (*a, Some(o.clone()))).collect();

    // Get alias or nft addresses when needed or just add the input again
    while let Some((unlock_address, output_response)) = utxo_chain_optional_response.pop() {
        // Don't request outputs for addresses where we already have the output
        if processed_alias_nft_addresses.insert(unlock_address) {
            match unlock_address {
                Address::Alias(address) => {
                    let output_id = client.alias_output_id(*address.alias_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                        let alias_output = AliasOutput::try_from(alias_output_dto)?;
                        // State transition if we add them to inputs
                        let alias_unlock_address = alias_output.state_controller_address();
                        // Add address to unprocessed_alias_nft_addresses so we get the required output there
                        // also
                        if alias_unlock_address.is_alias() || alias_unlock_address.is_nft() {
                            utxo_chain_optional_response.push((*alias_unlock_address, None));
                        }
                        processed_utxo_chains.push((*alias_unlock_address, output_response));
                    }
                }
                Address::Nft(address) => {
                    let output_id = client.nft_output_id(*address.nft_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Nft(nft_output) = &output_response.output {
                        let nft_output = NftOutput::try_from(nft_output)?;
                        let unlock_address = nft_output
                            .unlock_conditions()
                            .locked_address(nft_output.address(), current_time);
                        // Add address to unprocessed_alias_nft_addresses so we get the required output there also
                        if unlock_address.is_alias() || unlock_address.is_nft() {
                            utxo_chain_optional_response.push((*unlock_address, None));
                        }
                        processed_utxo_chains.push((*unlock_address, output_response));
                    }
                }
                _ => {}
            }
        }

        // Add if the output_response is available
        if let Some(output_response) = output_response {
            processed_utxo_chains.push((unlock_address, output_response));
        }
    }

    *utxo_chains = processed_utxo_chains;

    Ok(())
}
