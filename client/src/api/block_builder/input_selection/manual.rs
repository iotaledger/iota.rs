// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Manual input selection for transactions

use std::collections::HashSet;

use crypto::keys::slip10::Chain;
use iota_types::block::{
    address::Address,
    output::{AliasId, Output, OutputMetadata},
    protocol::ProtocolParameters,
};

use crate::{
    api::{
        address::search_address,
        block_builder::input_selection::{Burn, InputSelection, Selected},
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
        governance_transition: Option<HashSet<AliasId>>,
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
                    let (_output_amount, output_address) = ClientBlockBuilder::get_output_amount_and_address(
                        &output,
                        governance_transition.clone(),
                        current_time,
                    )?;

                    let bech32_hrp = self.client.get_bech32_hrp().await?;
                    let address_index_internal = match self.secret_manager {
                        Some(secret_manager) => {
                            match output_address {
                                Address::Ed25519(_) => Some(
                                    search_address(
                                        secret_manager,
                                        &bech32_hrp,
                                        self.coin_type,
                                        self.account_index,
                                        self.input_range.clone(),
                                        &output_address,
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
                        bech32_address: output_address.to_bech32(&bech32_hrp),
                    });
                }
            }
        }

        let required_inputs = inputs_data
            .iter()
            .map(|input| *input.output_id())
            .collect::<HashSet<_>>();

        // Assume that we own the addresses for inputs that are provided
        let available_input_addresses = inputs_data
            .iter()
            .map(|input| Ok(Address::try_from_bech32(&input.bech32_address)?.1))
            .collect::<Result<Vec<Address>>>()?;

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

        input_selection.select()
    }
}
