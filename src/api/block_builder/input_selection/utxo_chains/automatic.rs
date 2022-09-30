// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! automatic input selection for utxo chains

use bee_api_types::responses::OutputResponse;
use bee_block::{
    address::Address,
    output::{dto::OutputDto, AliasOutput, FoundryOutput, NftOutput, Output},
};
use crypto::keys::slip10::Chain;

use super::get_alias_and_nft_outputs_recursively;
use crate::{
    api::{block_builder::ClientBlockBuilder, search_address},
    constants::HD_WALLET_TYPE,
    secret::types::{InputSigningData, OutputMetadata},
    Result,
};

impl<'a> ClientBlockBuilder<'a> {
    /// Get inputs for utxo chains
    pub(crate) async fn get_utxo_chains_inputs(
        &self,
        outputs: impl Iterator<Item = &'a Output> + Clone,
    ) -> Result<Vec<InputSigningData>> {
        log::debug!("[get_utxo_chains_inputs]");
        let client = self.client;
        let bech32_hrp = client.get_bech32_hrp()?;
        let current_time = self.client.get_time_checked().await?;
        let token_supply = client.get_token_supply()?;

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
                            let alias_output = AliasOutput::try_from_dto(alias_output_dto, token_supply)?;

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
                            let nft_output = NftOutput::try_from_dto(nft_output, token_supply)?;

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
                            let foundry_output = FoundryOutput::try_from_dto(foundry_output_dto, token_supply)?;
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
                output: Output::try_from_dto(&output_response.output, token_supply)?,
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
