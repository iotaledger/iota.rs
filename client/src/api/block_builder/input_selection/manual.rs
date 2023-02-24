// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Manual input selection for transactions

use std::collections::HashSet;

use crypto::keys::slip10::Chain;
use iota_types::block::{
    address::Address,
    output::{Output, OutputMetadata},
    protocol::ProtocolParameters,
};

use crate::{
    api::{
        address::search_address,
        block_builder::input_selection::{Burn, InputSelection, Selected},
        input_selection::{core::requirement::alias::is_alias_transition_internal, is_alias_transition},
        ClientBlockBuilder,
    },
    constants::HD_WALLET_TYPE,
    secret::types::InputSigningData,
    Result,
};

impl<'a> ClientBlockBuilder<'a> {
    /// If custom inputs are provided we check if they are unspent, get the balance and search the Ed25519 addresses for
    /// them with the provided input_range so we can later sign them.
    /// Forwards to [try_select_inputs()] with all inputs in `mandatory_inputs`, so they will all be included in the
    /// transaction, even if not required for the provided outputs.
    pub(crate) async fn get_custom_inputs(
        &self,
        protocol_parameters: &ProtocolParameters,
        burn: Option<Burn>,
    ) -> Result<Selected> {
        log::debug!("[get_custom_inputs]");

        let mut inputs_data = Vec::new();
        let current_time = self.client.get_time_checked().await?;
        let token_supply = self.client.get_token_supply().await?;

        if let Some(inputs) = &self.inputs {
            for input in inputs {
                let output_response = self.client.get_output(input.output_id()).await?;
                let output = Output::try_from_dto(&output_response.output, token_supply)?;

                if !output_response.metadata.is_spent {
                    let alias_transition = is_alias_transition_internal(&output, *input.output_id(), &self.outputs);
                    let (unlock_address, _) = output.required_and_unlocked_address(
                        current_time,
                        input.output_id(),
                        alias_transition.map(|g| g.0),
                    )?;

                    let bech32_hrp = self.client.get_bech32_hrp().await?;
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
                                // Alias and NFT addresses can't be generated from a private key.
                                _ => None,
                            }
                        }
                        // Assuming default for offline signing.
                        None => Some((0, false)),
                    };

                    inputs_data.push(InputSigningData {
                        output,
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

        let required_inputs = inputs_data
            .iter()
            .map(|input| *input.output_id())
            .collect::<HashSet<_>>();

        // Assume that we own the addresses for inputs that are provided
        let mut available_input_addresses = Vec::new();
        for input in &inputs_data {
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

        inputs_data.sort_unstable_by_key(|input| *input.output_id());
        inputs_data.dedup_by_key(|input| *input.output_id());

        let mut input_selection = InputSelection::new(
            inputs_data,
            self.outputs.clone(),
            available_input_addresses,
            protocol_parameters.clone(),
        )
        .required_inputs(required_inputs)
        .timestamp(current_time);

        if let Some(address) = self.custom_remainder_address {
            input_selection = input_selection.remainder_address(address);
        }

        if let Some(burn) = burn {
            input_selection = input_selection.burn(burn);
        }

        Ok(input_selection.select()?)
    }
}
