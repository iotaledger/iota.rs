// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! sender and issuer features input selection

use std::collections::HashSet;

use crypto::keys::slip10::Chain;
use iota_types::block::{
    address::Address,
    output::{dto::OutputDto, feature::Features, AliasOutput, NftOutput, Output, OutputMetadata},
};

use crate::{
    api::{
        address::search_address,
        block_builder::input_selection::core::{
            error::Error as InputSelectionError, requirement::alias::is_alias_transition,
        },
        ClientBlockBuilder,
    },
    constants::HD_WALLET_TYPE,
    secret::types::InputSigningData,
    Error, Result,
};

impl<'a> ClientBlockBuilder<'a> {
    pub(crate) async fn get_inputs_for_sender_and_issuer(
        &self,
        utxo_chain_inputs: &[InputSigningData],
    ) -> Result<Vec<InputSigningData>> {
        log::debug!("[get_inputs_for_sender_and_issuer]");

        let mut required_inputs = Vec::new();
        let bech32_hrp = self.client.get_bech32_hrp().await?;
        let current_time = self.client.get_time_checked().await?;
        let token_supply = self.client.get_token_supply().await?;

        let required_sender_or_issuer_addresses =
            get_required_addresses_for_sender_and_issuer(&[], &self.outputs, current_time)?;

        for sender_or_issuer_address in required_sender_or_issuer_addresses {
            match sender_or_issuer_address {
                Address::Ed25519(_) => {
                    // Check if the address is derived from the seed
                    let (address_index, internal) = search_address(
                        self.secret_manager.ok_or(Error::MissingParameter("secret manager"))?,
                        &bech32_hrp,
                        self.coin_type,
                        self.account_index,
                        self.input_range.clone(),
                        &sender_or_issuer_address,
                    )
                    .await?;
                    let address_outputs = self
                        .basic_address_outputs(sender_or_issuer_address.to_bech32(&bech32_hrp))
                        .await?;

                    let mut found_output = false;
                    for output_response in address_outputs {
                        let output = Output::try_from_dto(&output_response.output, token_supply)?;

                        // We can ignore the unlocked_alias_or_nft_address, since we only requested basic outputs
                        let (required_unlock_address, _unlocked_alias_or_nft_address) = output
                            .required_and_unlocked_address(
                                current_time,
                                &output_response.metadata.output_id()?,
                                None,
                            )?;

                        if required_unlock_address == sender_or_issuer_address {
                            required_inputs.push(InputSigningData {
                                output,
                                output_metadata: OutputMetadata::try_from(&output_response.metadata)?,
                                chain: Some(Chain::from_u32_hardened(vec![
                                    HD_WALLET_TYPE,
                                    self.coin_type,
                                    self.account_index,
                                    internal as u32,
                                    address_index,
                                ])),
                            });
                            found_output = true;
                            break;
                        }
                    }

                    if !found_output {
                        return Err(InputSelectionError::MissingInputWithEd25519Address)?;
                    }
                }
                Address::Alias(alias_address) => {
                    // Check if output is alias address.
                    let alias_id = alias_address.alias_id();

                    // check if already found or request new.
                    if !utxo_chain_inputs.iter().chain(required_inputs.iter()).any(|input| {
                        if let Output::Alias(alias_output) = &input.output {
                            *alias_id == alias_output.alias_id_non_null(input.output_id())
                        } else {
                            false
                        }
                    }) {
                        let output_id = self.client.alias_output_id(*alias_id).await?;
                        let output_response = self.client.get_output(&output_id).await?;
                        if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                            let alias_output = AliasOutput::try_from_dto(alias_output_dto, token_supply)?;
                            // State transition if we add them to inputs
                            let unlock_address = alias_output.state_controller_address();
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
                                                unlock_address,
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

                            required_inputs.push(InputSigningData {
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
                            });
                        }
                    }
                }
                Address::Nft(nft_address) => {
                    // Check if output is nft address.
                    let nft_id = nft_address.nft_id();

                    // Check if already found or request new.
                    if !utxo_chain_inputs.iter().chain(required_inputs.iter()).any(|input| {
                        if let Output::Nft(nft_output) = &input.output {
                            nft_id == nft_output.nft_id()
                        } else {
                            false
                        }
                    }) {
                        let output_id = self.client.nft_output_id(*nft_id).await?;
                        let output_response = self.client.get_output(&output_id).await?;
                        if let OutputDto::Nft(nft_output) = &output_response.output {
                            let nft_output = NftOutput::try_from_dto(nft_output, token_supply)?;

                            let unlock_address = nft_output
                                .unlock_conditions()
                                .locked_address(nft_output.address(), current_time);

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
                                                unlock_address,
                                            )
                                            .await?,
                                        ),
                                        // Alias and NFT addresses can't be generated from a private key.
                                        _ => None,
                                    }
                                }
                                // Assuming default for offline signing.
                                None => Some((0, false)),
                            };

                            required_inputs.push(InputSigningData {
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
                            });
                        }
                    }
                }
            }
        }

        // Check required Alias and NFT outputs with new added outputs.
        // No need to check for sender and issuer again, since these outputs already exist and we don't set new features
        // for them.
        let utxo_chain_inputs = self
            .get_utxo_chains_inputs(required_inputs.iter().map(|i| &i.output))
            .await?;
        required_inputs.extend(utxo_chain_inputs.into_iter());

        Ok(required_inputs)
    }
}

// Returns required addresses for sender and issuer features that aren't already unlocked with the selected_inputs
fn get_required_addresses_for_sender_and_issuer(
    selected_inputs: &[InputSigningData],
    outputs: &Vec<Output>,
    current_time: u32,
) -> crate::Result<HashSet<Address>> {
    log::debug!("[get_required_addresses_for_sender_and_issuer]");

    // Addresses in the inputs that will be unlocked in the transaction
    let mut unlocked_addresses = HashSet::new();
    for input_signing_data in selected_inputs {
        let alias_transition = is_alias_transition(input_signing_data, outputs);
        let (required_unlock_address, unlocked_alias_or_nft_address) =
            input_signing_data.output.required_and_unlocked_address(
                current_time,
                input_signing_data.output_id(),
                alias_transition.map(|(alias_transition, _)| alias_transition),
            )?;
        unlocked_addresses.insert(required_unlock_address);
        if let Some(unlocked_alias_or_nft_address) = unlocked_alias_or_nft_address {
            unlocked_addresses.insert(unlocked_alias_or_nft_address);
        }
    }

    let mut required_sender_or_issuer_addresses = HashSet::new();

    for output in outputs {
        if let Some(sender_feature) = output.features().and_then(Features::sender) {
            if !required_sender_or_issuer_addresses.contains(sender_feature.address()) {
                // Only add if not already present in the selected inputs.
                if !unlocked_addresses.contains(sender_feature.address()) {
                    required_sender_or_issuer_addresses.insert(*sender_feature.address());
                }
            }
        }

        // Issuer address only needs to be unlocked when the utxo chain is newly created.
        let utxo_chain_creation = match &output {
            Output::Alias(alias_output) => alias_output.alias_id().is_null(),
            Output::Nft(nft_output) => nft_output.nft_id().is_null(),
            _ => false,
        };
        if utxo_chain_creation {
            if let Some(issuer_feature) = output.immutable_features().and_then(Features::issuer) {
                if !required_sender_or_issuer_addresses.contains(issuer_feature.address()) {
                    // Only add if not already present in the selected inputs.
                    if !unlocked_addresses.contains(issuer_feature.address()) {
                        required_sender_or_issuer_addresses.insert(*issuer_feature.address());
                    }
                }
            }
        }
    }

    Ok(required_sender_or_issuer_addresses)
}
