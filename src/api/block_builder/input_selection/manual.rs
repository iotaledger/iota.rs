// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Manual input selection for transactions

use std::collections::HashSet;

use bee_block::{
    address::Address,
    output::{AliasId, ByteCostConfig, Output},
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

/// If custom inputs are provided we check if they are unspent, get the balance and search the Ed25519 addresses for
/// them with the provided input_range so we can later sign them.
/// Forwards to [try_select_inputs()] with `force_use_all_inputs` set to true, so all inputs will be included in the
/// transaction, even if not required for the provided outputs.
/// Careful with setting `allow_burning` to `true`, native tokens can get easily burned by accident.
pub(crate) async fn get_custom_inputs(
    block_builder: &ClientBlockBuilder<'_>,
    governance_transition: Option<HashSet<AliasId>>,
    byte_cost_config: &ByteCostConfig,
    allow_burning: bool,
) -> Result<SelectedTransactionData> {
    log::debug!("[get_custom_inputs]");
    let mut inputs_data = Vec::new();

    if let Some(inputs) = &block_builder.inputs {
        let local_time = block_builder.client.get_time_checked().await?;

        for input in inputs {
            let output_response = block_builder.client.get_output(input.output_id()).await?;

            if !output_response.metadata.is_spent {
                let (_output_amount, output_address) = ClientBlockBuilder::get_output_amount_and_address(
                    &output_response.output,
                    governance_transition.clone(),
                    local_time,
                )?;

                let bech32_hrp = block_builder.client.get_bech32_hrp().await?;
                let (address_index, internal) = match block_builder.secret_manager {
                    Some(secret_manager) => {
                        match output_address {
                            Address::Ed25519(_) => {
                                search_address(
                                    secret_manager,
                                    &bech32_hrp,
                                    block_builder.coin_type,
                                    block_builder.account_index,
                                    block_builder.input_range.clone(),
                                    &output_address,
                                )
                                .await?
                            }
                            // Alias and NFT addresses can't be generated from a private key
                            _ => (0, false),
                        }
                    }
                    None => (0, false),
                };
                inputs_data.push(InputSigningData {
                    output: Output::try_from(&output_response.output)?,
                    output_metadata: OutputMetadata::try_from(&output_response.metadata)?,
                    chain: Some(Chain::from_u32_hardened(vec![
                        HD_WALLET_TYPE,
                        block_builder.coin_type,
                        block_builder.account_index,
                        internal as u32,
                        address_index,
                    ])),
                    bech32_address: output_address.to_bech32(&bech32_hrp),
                });
            }
        }
    }
    let selected_transaction_data = try_select_inputs(
        inputs_data,
        block_builder.outputs.clone(),
        true,
        block_builder.custom_remainder_address,
        byte_cost_config,
        allow_burning,
    )
    .await?;
    Ok(selected_transaction_data)
}
