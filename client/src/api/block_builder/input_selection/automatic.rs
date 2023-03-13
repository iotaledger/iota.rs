// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Automatic input selection for transactions

use std::collections::HashSet;

use crypto::keys::slip10::Chain;
use iota_types::{
    api::core::response::OutputWithMetadataResponse,
    block::{
        address::Address,
        output::{Output, OutputMetadata},
        protocol::ProtocolParameters,
    },
};

use crate::{
    api::{
        block_builder::input_selection::core::{Error as InputSelectionError, InputSelection, Selected},
        input_selection::is_alias_transition,
        ClientBlockBuilder, ADDRESS_GAP_RANGE,
    },
    constants::HD_WALLET_TYPE,
    node_api::indexer::query_parameters::QueryParameter,
    secret::types::InputSigningData,
    unix_timestamp_now, Error, Result,
};

impl<'a> ClientBlockBuilder<'a> {
    // Get basic outputs for an address without storage deposit return unlock condition
    pub(crate) async fn basic_address_outputs(&self, address: String) -> Result<Vec<OutputWithMetadataResponse>> {
        let mut output_ids = Vec::new();

        // First request to get all basic outputs that can directly be unlocked by the address.
        output_ids.extend(
            self.client
                .basic_output_ids(vec![
                    QueryParameter::Address(address.clone()),
                    QueryParameter::HasStorageDepositReturn(false),
                ])
                .await?
                .items,
        );

        // Second request to get all basic outputs that can be unlocked by the address through the expiration condition.
        output_ids.extend(
            self.client
                .basic_output_ids(vec![
                    QueryParameter::ExpirationReturnAddress(address),
                    QueryParameter::HasExpiration(true),
                    QueryParameter::HasStorageDepositReturn(false),
                    // Ignore outputs that aren't expired yet
                    QueryParameter::ExpiresBefore(unix_timestamp_now()),
                ])
                .await?
                .items,
        );

        self.client.get_outputs(output_ids).await
    }

    /// Searches inputs for provided outputs, by requesting the outputs from the account addresses or for
    /// alias/foundry/nft outputs get the latest state with their alias/nft id. Forwards to [try_select_inputs()].
    pub(crate) async fn get_inputs(&self, protocol_parameters: &ProtocolParameters) -> Result<Selected> {
        log::debug!("[get_inputs]");

        let account_index = self.account_index;
        let mut gap_index = self.initial_address_index;
        let mut empty_address_count: u64 = 0;
        let mut cached_error = None;
        let token_supply = self.client.get_token_supply().await?;

        log::debug!("[get_inputs from utxo chains]");

        // First get inputs for utxo chains (Alias, Foundry, NFT outputs).
        let mut available_inputs = self.get_utxo_chains_inputs(self.outputs.iter()).await?;

        let required_inputs_for_sender_or_issuer = self.get_inputs_for_sender_and_issuer(&available_inputs).await?;
        let required_inputs_for_sender_or_issuer_ids = required_inputs_for_sender_or_issuer
            .iter()
            .map(|input| *input.output_id())
            .collect::<HashSet<_>>();

        available_inputs.extend(required_inputs_for_sender_or_issuer);
        available_inputs.sort_unstable_by_key(|input| *input.output_id());
        available_inputs.dedup_by_key(|input| *input.output_id());

        let current_time = self.client.get_time_checked().await?;
        // Assume that we own the addresses for inputs that are required for the provided outputs
        let mut available_input_addresses = Vec::new();
        for input in &available_inputs {
            let alias_transition = is_alias_transition(input, &self.outputs);
            let (required_unlock_address, unlocked_alias_or_nft_address) = input.output.required_and_unlocked_address(
                current_time,
                input.output_id(),
                alias_transition.map(|(alias_transition, _)| alias_transition),
            )?;
            available_input_addresses.push(required_unlock_address);
            if let Some(unlocked_alias_or_nft_address) = unlocked_alias_or_nft_address {
                available_input_addresses.push(unlocked_alias_or_nft_address);
            }
        }

        // Try to select inputs with required inputs for utxo chains alone before requesting more inputs from addresses.
        let mut input_selection = InputSelection::new(
            available_inputs.clone(),
            self.outputs.clone(),
            available_input_addresses.clone(),
            protocol_parameters.clone(),
        )
        .required_inputs(required_inputs_for_sender_or_issuer_ids.clone())
        .timestamp(current_time);

        if let Some(address) = self.custom_remainder_address {
            input_selection = input_selection.remainder_address(address);
        }

        if let Ok(selected_transaction_data) = input_selection.select() {
            return Ok(selected_transaction_data);
        }

        log::debug!("[get_inputs from addresses]");

        // Then select inputs with outputs from addresses.
        let selected_transaction_data = 'input_selection: loop {
            // Get the addresses in the BIP path/index ~ path/index+20.
            let addresses = self
                .client
                .get_addresses(
                    self.secret_manager
                        .ok_or(crate::Error::MissingParameter("secret manager"))?,
                )
                .with_account_index(account_index)
                .with_range(gap_index..gap_index + ADDRESS_GAP_RANGE)
                .get_all()
                .await?;

            available_input_addresses.extend(
                addresses
                    .public
                    .iter()
                    .map(|bech32_address| Ok(Address::try_from_bech32(bech32_address)?.1))
                    .collect::<Result<Vec<Address>>>()?,
            );
            available_input_addresses.extend(
                addresses
                    .internal
                    .iter()
                    .map(|bech32_address| Ok(Address::try_from_bech32(bech32_address)?.1))
                    .collect::<Result<Vec<Address>>>()?,
            );

            // Have public and internal addresses with the index ascending ordered.
            let mut public_and_internal_addresses = Vec::new();

            for index in 0..addresses.public.len() {
                public_and_internal_addresses.push((addresses.public[index].clone(), false));
                public_and_internal_addresses.push((addresses.internal[index].clone(), true));
            }

            // For each address, get the address outputs.
            let mut address_index = gap_index;

            for (index, (str_address, internal)) in public_and_internal_addresses.iter().enumerate() {
                let address_outputs = self.basic_address_outputs(str_address.to_string()).await?;

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

                    for output_response in address_outputs {
                        let output = Output::try_from_dto(&output_response.output, token_supply)?;
                        let address = Address::try_from_bech32(str_address)?.1;

                        // We can ignore the unlocked_alias_or_nft_address, since we only requested basic outputs
                        let (required_unlock_address, _unlocked_alias_or_nft_address) = output
                            .required_and_unlocked_address(
                                current_time,
                                &output_response.metadata.output_id()?,
                                None,
                            )?;
                        if required_unlock_address == address {
                            available_inputs.push(InputSigningData {
                                output,
                                output_metadata: OutputMetadata::try_from(&output_response.metadata)?,
                                chain: Some(Chain::from_u32_hardened(vec![
                                    HD_WALLET_TYPE,
                                    self.coin_type,
                                    account_index,
                                    *internal as u32,
                                    address_index,
                                ])),
                            });
                        }
                    }

                    available_inputs.sort_unstable_by_key(|input| *input.output_id());
                    available_inputs.dedup_by_key(|input| *input.output_id());

                    let mut input_selection = InputSelection::new(
                        available_inputs.clone(),
                        self.outputs.clone(),
                        available_input_addresses.clone(),
                        protocol_parameters.clone(),
                    )
                    .required_inputs(required_inputs_for_sender_or_issuer_ids.clone())
                    .timestamp(current_time);

                    if let Some(address) = self.custom_remainder_address {
                        input_selection = input_selection.remainder_address(address);
                    }

                    let selected_transaction_data = match input_selection.select() {
                        Ok(r) => r,
                        // for these errors, just try again in the next round with more addresses which might have more
                        // outputs.
                        Err(err @ InputSelectionError::InsufficientAmount { .. }) => {
                            cached_error.replace(Error::from(err));
                            continue;
                        }
                        Err(err @ InputSelectionError::InsufficientNativeTokenAmount { .. }) => {
                            cached_error.replace(Error::from(err));
                            continue;
                        }
                        Err(err @ InputSelectionError::NoAvailableInputsProvided { .. }) => {
                            cached_error.replace(Error::from(err));
                            continue;
                        }
                        // Not enough balance for a remainder.
                        Err(InputSelectionError::Block(block_error)) => match block_error {
                            iota_types::block::Error::InvalidStorageDepositAmount { .. } => {
                                cached_error.replace(Error::from(InputSelectionError::Block(block_error)));
                                continue;
                            }
                            _ => return Err(block_error.into()),
                        },
                        Err(e) => return Err(e)?,
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
                // returned last cached error
                return Err(cached_error.unwrap_or_else(|| Error::from(InputSelectionError::NoAvailableInputsProvided)));
            }
        };

        Ok(selected_transaction_data)
    }
}
