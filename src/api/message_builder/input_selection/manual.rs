// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Manual input selection for transactions

use crate::{
    api::{
        address::search_address, input_selection::try_select_inputs,
        message_builder::input_selection::types::SelectedTransactionData, ClientMessageBuilder,
    },
    constants::HD_WALLET_TYPE,
    signing::types::InputSigningData,
    Result,
};

use bee_message::{
    address::Address,
    output::{AliasId, ByteCostConfig},
};
use crypto::keys::slip10::Chain;

use std::collections::HashSet;

/// If custom inputs are provided we check if they are unspent, get the balance and search the Ed25519 addresses for
/// them with the provided input_range so we can later sign them.
/// Forwards to [try_select_inputs()] with `force_use_all_inputs` set to true, so all inputs will be included in the
/// transaction, even if not required for the provided outputs.
pub(crate) async fn get_custom_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    governance_transition: Option<HashSet<AliasId>>,
    byte_cost_config: &ByteCostConfig,
) -> Result<SelectedTransactionData> {
    log::debug!("[get_custom_inputs]");
    let mut input_signing_data_entries = Vec::new();

    if let Some(inputs) = &message_builder.inputs {
        for input in inputs {
            let output_response = message_builder.client.get_output(input.output_id()).await?;

            if !output_response.is_spent {
                let (_output_amount, output_address) = ClientMessageBuilder::get_output_amount_and_address(
                    &output_response.output,
                    governance_transition.clone(),
                )?;

                let bech32_hrp = message_builder.client.get_bech32_hrp().await?;
                let (address_index, internal) = match message_builder.signer {
                    Some(signer) => {
                        match output_address {
                            Address::Ed25519(_) => {
                                search_address(
                                    signer,
                                    &bech32_hrp,
                                    message_builder.coin_type,
                                    message_builder.account_index,
                                    message_builder.input_range.clone(),
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
                input_signing_data_entries.push(InputSigningData {
                    output_response: output_response.clone(),
                    chain: Some(Chain::from_u32_hardened(vec![
                        HD_WALLET_TYPE,
                        message_builder.coin_type,
                        message_builder.account_index,
                        internal as u32,
                        address_index,
                    ])),
                    bech32_address: output_address.to_bech32(&bech32_hrp),
                });
            }
        }
    }
    let selected_transaction_data = try_select_inputs(
        input_signing_data_entries,
        message_builder.outputs.clone(),
        true,
        message_builder.custom_remainder_address,
        byte_cost_config,
    )
    .await?;
    Ok(selected_transaction_data)
}
