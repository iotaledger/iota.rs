// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! sender and issuer features input selection

use std::collections::HashSet;

use bee_block::{
    address::Address,
    output::{dto::OutputDto, feature::Features, AliasOutput, NftOutput, Output, OutputId},
};
use crypto::keys::slip10::Chain;

use crate::{
    api::{
        address::search_address,
        input_selection::{helpers::is_basic_output_address_unlockable, output_contains_address},
        ClientBlockBuilder,
    },
    constants::HD_WALLET_TYPE,
    secret::types::{InputSigningData, OutputMetadata},
    Error, Result,
};

impl<'a> ClientBlockBuilder<'a> {
    pub(crate) async fn get_inputs_for_sender_and_issuer(
        &self,
        utxo_chain_inputs: &[InputSigningData],
    ) -> Result<Vec<InputSigningData>> {
        log::debug!("[get_inputs_for_sender_and_issuer]");

        let mut required_inputs = Vec::new();
        let bech32_hrp = self.client.get_bech32_hrp()?;
        let current_time = self.client.get_time_checked().await?;
        let token_supply = self.client.get_token_supply()?;

        let all_required_addresses = get_required_addresses_for_sender_and_issuer(&[], &self.outputs, current_time)?;

        for address in all_required_addresses {
            match address {
                Address::Ed25519(_) => {
                    let (address_index, internal) = search_address(
                        self.secret_manager.ok_or(Error::MissingParameter("secret manager"))?,
                        &bech32_hrp,
                        self.coin_type,
                        self.account_index,
                        self.input_range.clone(),
                        &address,
                    )
                    .await?;
                    // If it didn't return with an error, then the address was found.

                    let address_outputs = self.address_outputs(address.to_bech32(&bech32_hrp)).await?;

                    let mut found_output = false;
                    for output_response in address_outputs {
                        let output = Output::try_from_dto(&output_response.output, token_supply)?;

                        if is_basic_output_address_unlockable(&output, &address, current_time) {
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
                                bech32_address: address.to_bech32(&bech32_hrp),
                            });
                            // we want to include all outputs, because another output might be better balance wise,
                            // but will not unlock the address we need
                            found_output = true;
                            break;
                        }
                    }

                    if !found_output {
                        return Err(Error::MissingInputWithEd25519Address);
                    }
                }
                Address::Alias(alias_address) => {
                    // Check if output is alias address.
                    let alias_id = alias_address.alias_id();

                    // check if already found or request new.
                    if !utxo_chain_inputs.iter().chain(required_inputs.iter()).any(|input| {
                        if let Output::Alias(alias_output) = &input.output {
                            alias_id == alias_output.alias_id()
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
                                bech32_address: unlock_address.to_bech32(&bech32_hrp),
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
                                bech32_address: unlock_address.to_bech32(&bech32_hrp),
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

// Select inputs for sender and issuer features
pub(crate) fn select_inputs_for_sender_and_issuer<'a>(
    inputs: impl Iterator<Item = &'a InputSigningData> + Clone,
    selected_inputs: &mut Vec<InputSigningData>,
    selected_inputs_output_ids: &mut HashSet<OutputId>,
    outputs: &mut Vec<Output>,
    current_time: u32,
) -> Result<()> {
    log::debug!("[select_inputs_for_sender_and_issuer]");

    let all_required_addresses = get_required_addresses_for_sender_and_issuer(selected_inputs, outputs, current_time)?;
    'addresses_loop: for address in all_required_addresses {
        match address {
            Address::Ed25519(_) => {
                // first check already selected outputs
                for input_signing_data in selected_inputs.iter() {
                    if output_contains_address(
                        &input_signing_data.output,
                        Some(input_signing_data.output_id()?),
                        &address,
                        current_time,
                    ) {
                        continue 'addresses_loop;
                    }
                }

                // if not found, check currently not selected outputs
                for input_signing_data in inputs.clone() {
                    // Skip already added inputs
                    let output_id = input_signing_data.output_id()?;
                    if selected_inputs_output_ids.contains(&output_id) {
                        continue;
                    }
                    if output_contains_address(&input_signing_data.output, Some(output_id), &address, current_time) {
                        selected_inputs.push(input_signing_data.clone());
                        selected_inputs_output_ids.insert(output_id);
                        // break when we added the required output to the selected_inputs
                        continue 'addresses_loop;
                    }
                }

                return Err(Error::MissingInput(format!(
                    "missing input with {address:?} for sender or issuer feature"
                )));
            }
            Address::Alias(alias_address) => {
                // Check if output is alias address.
                let alias_id = alias_address.alias_id();

                // check if already selected.
                for selected_input in selected_inputs.iter() {
                    if let Output::Alias(alias_output) = &selected_input.output {
                        if alias_id == &alias_output.alias_id().or_from_output_id(selected_input.output_id()?) {
                            continue 'addresses_loop;
                        }
                    }
                }

                // if not found, check currently not selected outputs
                for input_signing_data in inputs.clone() {
                    // Skip already added inputs
                    let output_id = input_signing_data.output_id()?;
                    if selected_inputs_output_ids.contains(&output_id) {
                        continue;
                    }
                    if let Output::Alias(alias_output) = &input_signing_data.output {
                        if alias_id
                            == &alias_output
                                .alias_id()
                                .or_from_output_id(input_signing_data.output_id()?)
                        {
                            selected_inputs.push(input_signing_data.clone());
                            selected_inputs_output_ids.insert(output_id);
                            // break when we added the required output to the selected_inputs
                            continue 'addresses_loop;
                        }
                    }
                }
                return Err(Error::MissingInput(format!(
                    "missing alias {alias_id} for sender or issuer feature"
                )));
            }
            Address::Nft(nft_address) => {
                // Check if output is nft address.
                let nft_id = nft_address.nft_id();

                // check if already selected.
                for selected_input in selected_inputs.iter() {
                    if let Output::Nft(nft_output) = &selected_input.output {
                        if nft_id == &nft_output.nft_id().or_from_output_id(selected_input.output_id()?) {
                            continue 'addresses_loop;
                        }
                    }
                }

                // if not found, check currently not selected outputs
                for input_signing_data in inputs.clone() {
                    // Skip already added inputs
                    let output_id = input_signing_data.output_id()?;
                    if selected_inputs_output_ids.contains(&output_id) {
                        continue;
                    }
                    if let Output::Nft(nft_output) = &input_signing_data.output {
                        if nft_id == &nft_output.nft_id().or_from_output_id(input_signing_data.output_id()?) {
                            selected_inputs.push(input_signing_data.clone());
                            selected_inputs_output_ids.insert(output_id);
                            // break when we added the required output to the selected_inputs
                            continue 'addresses_loop;
                        }
                    }
                }
                return Err(Error::MissingInput(format!(
                    "missing nft {nft_id} for sender or issuer feature"
                )));
            }
        }
    }

    Ok(())
}

// Returns required addresses for sender and issuer features that aren't already unlocked with the selected_inputs
fn get_required_addresses_for_sender_and_issuer(
    selected_inputs: &[InputSigningData],
    outputs: &Vec<Output>,
    current_time: u32,
) -> crate::Result<HashSet<Address>> {
    log::debug!("[get_required_addresses_for_sender_and_issuer]");

    let mut all_required_addresses = HashSet::new();

    for output in outputs {
        if let Some(sender_feature) = output.features().and_then(Features::sender) {
            if !all_required_addresses.contains(sender_feature.address()) {
                // Only add if not already present in the selected inputs.
                if !selected_inputs.iter().any(|input_data| {
                    output_contains_address(
                        &input_data.output,
                        Some(input_data.output_id().expect("invalid output id in input signing data")),
                        sender_feature.address(),
                        current_time,
                    )
                }) {
                    all_required_addresses.insert(*sender_feature.address());
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
                if !all_required_addresses.contains(issuer_feature.address()) {
                    // Only add if not already present in the selected inputs.
                    if !selected_inputs.iter().any(|input_data| {
                        output_contains_address(
                            &input_data.output,
                            Some(input_data.output_id().expect("invalid output id in input signing data")),
                            issuer_feature.address(),
                            current_time,
                        )
                    }) {
                        all_required_addresses.insert(*issuer_feature.address());
                    }
                }
            }
        }
    }

    Ok(all_required_addresses)
}
