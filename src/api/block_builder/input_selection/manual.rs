// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Manual input selection for transactions

use std::collections::HashSet;

use bee_block::{
    address::Address,
    output::{AliasId, Output, RentStructure},
};
use crypto::keys::slip10::Chain;

use crate::{
    api::{
        address::search_address, block_builder::input_selection::types::SelectedTransactionData,
        input_selection::try_select_inputs, ClientBlockBuilder,
    },
    constants::HD_WALLET_TYPE,
    secret::types::{InputSigningData, OutputMetadata},
    Result,
};

impl<'a> ClientBlockBuilder<'a> {
    /// If custom inputs are provided we check if they are unspent, get the balance and search the Ed25519 addresses for
    /// them with the provided input_range so we can later sign them.
    /// Forwards to [try_select_inputs()] with `force_use_all_inputs` set to true, so all inputs will be included in the
    /// transaction, even if not required for the provided outputs.
    /// Careful with setting `allow_burning` to `true`, native tokens, nfts or alias outputs can get easily burned by
    /// accident.
    pub(crate) async fn get_custom_inputs(
        &self,
        governance_transition: Option<HashSet<AliasId>>,
        rent_structure: &RentStructure,
        allow_burning: bool,
    ) -> Result<SelectedTransactionData> {
        log::debug!("[get_custom_inputs]");
        let mut inputs_data = Vec::new();

        let current_time = self.client.get_time_checked().await?;
        if let Some(inputs) = &self.inputs {
            for input in inputs {
                let output_response = self.client.get_output(input.output_id()).await?;
                let output = Output::try_from(&output_response.output)?;

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
                                // Alias and NFT addresses can't be generated from a private key
                                _ => None,
                            }
                        }
                        // Assuming default for offline signing
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
        let selected_transaction_data = try_select_inputs(
            inputs_data,
            self.outputs.clone(),
            true,
            self.custom_remainder_address,
            rent_structure,
            allow_burning,
            current_time,
        )?;
        Ok(selected_transaction_data)
    }
}
