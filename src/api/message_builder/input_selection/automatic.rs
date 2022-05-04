// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Automatic input selection for transactions

use std::str::FromStr;

use bee_message::{
    address::Address,
    output::{feature_block::FeatureBlock, ByteCostConfig, Output},
    payload::transaction::TransactionId,
    MessageId,
};
use crypto::keys::slip10::Chain;

use crate::{
    api::{
        address::search_address,
        input_selection::try_select_inputs,
        message_builder::input_selection::{output_data::get_utxo_chains_inputs, types::SelectedTransactionData},
        ClientMessageBuilder, ADDRESS_GAP_RANGE,
    },
    constants::HD_WALLET_TYPE,
    node_api::indexer::query_parameters::QueryParameter,
    secret::types::{InputSigningData, OutputMetadata},
    Error, Result,
};

/// Searches inputs for provided outputs, by requesting the outputs from the account addresses or for alias/foundry/nft
/// outputs get the latest state with their alias/nft id. Forwards to [try_select_inputs()]
pub(crate) async fn get_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    byte_cost_config: &ByteCostConfig,
) -> Result<SelectedTransactionData> {
    log::debug!("[get_inputs]");
    let account_index = message_builder.account_index;
    let mut gap_index = message_builder.initial_address_index;
    let mut empty_address_count: u64 = 0;
    let mut cached_error = None;

    // First get inputs for utxo chains (alias, foundry, nft outputs)
    let mut available_inputs = get_utxo_chains_inputs(message_builder, &message_builder.outputs).await?;
    let (force_use_all_inputs, required_ed25519_inputs) = get_inputs_for_sender_and_issuer(message_builder).await?;
    available_inputs.extend(required_ed25519_inputs.into_iter());

    // Try to select inputs with required inputs for utxo chains alone before requesting more inputs from addresses
    if let Ok(selected_transaction_data) = try_select_inputs(
        available_inputs.clone(),
        message_builder.outputs.clone(),
        force_use_all_inputs,
        message_builder.custom_remainder_address,
        byte_cost_config,
        // Don't allow burning of native tokens during automatic input selection, because otherwise it
        // could lead to burned native tokens by accident
        false,
    )
    .await
    {
        return Ok(selected_transaction_data);
    };

    log::debug!("[get_inputs from addresses]");
    // then select inputs with outputs from addresses
    let selected_transaction_data = 'input_selection: loop {
        // Get the addresses in the BIP path/index ~ path/index+20
        let addresses = message_builder
            .client
            .get_addresses(
                message_builder
                    .secret_manager
                    .ok_or(crate::Error::MissingParameter("secret manager"))?,
            )
            .with_account_index(account_index)
            .with_range(gap_index..gap_index + ADDRESS_GAP_RANGE)
            .get_all()
            .await?;
        // Have public and internal addresses with the index ascending ordered
        let mut public_and_internal_addresses = Vec::new();
        for index in 0..addresses.public.len() {
            public_and_internal_addresses.push((addresses.public[index].clone(), false));
            public_and_internal_addresses.push((addresses.internal[index].clone(), true));
        }

        // For each address, get the address outputs
        let mut address_index = gap_index;
        for (index, (str_address, internal)) in public_and_internal_addresses.iter().enumerate() {
            let output_ids = crate::node_api::indexer::routes::output_ids(
                message_builder.client,
                vec![
                    QueryParameter::Address(str_address.to_string()),
                    QueryParameter::HasExpirationCondition(false),
                    QueryParameter::HasTimelockCondition(false),
                    QueryParameter::HasStorageDepositReturnCondition(false),
                ],
            )
            .await?;

            let address_outputs = crate::node_api::core::get_outputs(message_builder.client, output_ids).await?;

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
                    available_inputs.push(InputSigningData {
                        output: Output::try_from(&output_response.output)?,
                        output_metadata: OutputMetadata {
                            message_id: MessageId::from_str(&output_response.message_id)?,
                            transaction_id: TransactionId::from_str(&output_response.transaction_id)?,
                            output_index: output_response.output_index,
                            is_spent: output_response.is_spent,
                            milestone_index_spent: output_response.milestone_index_spent,
                            milestone_timestamp_spent: output_response.milestone_timestamp_spent,
                            transaction_id_spent: output_response
                                .transaction_id_spent
                                .map(|s| TransactionId::from_str(&s))
                                .transpose()?,
                            milestone_index_booked: output_response.milestone_index_booked,
                            milestone_timestamp_booked: output_response.milestone_timestamp_booked,
                            ledger_index: output_response.ledger_index,
                        },
                        chain: Some(Chain::from_u32_hardened(vec![
                            HD_WALLET_TYPE,
                            message_builder.coin_type,
                            account_index,
                            *internal as u32,
                            address_index,
                        ])),
                        bech32_address: str_address.to_owned(),
                    });
                }
                let selected_transaction_data = match try_select_inputs(
                    available_inputs.clone(),
                    message_builder.outputs.clone(),
                    force_use_all_inputs,
                    message_builder.custom_remainder_address,
                    byte_cost_config,
                    // Don't allow burning of native tokens during automatic input selection, because otherwise it
                    // could lead to burned native tokens by accident
                    false,
                )
                .await
                {
                    Ok(r) => r,
                    // for these errors ,just try again in the next round with more addresses which might have more
                    // outputs
                    Err(crate::Error::NotEnoughBalance(a, b)) => {
                        cached_error.replace(crate::Error::NotEnoughBalance(a, b));
                        continue;
                    }
                    Err(crate::Error::NotEnoughNativeTokens(a)) => {
                        cached_error.replace(crate::Error::NotEnoughNativeTokens(a));
                        continue;
                    }
                    // Native tokens left, but no balance for the storage deposit for a remainder
                    Err(crate::Error::NoBalanceForNativeTokenRemainder) => {
                        cached_error.replace(crate::Error::NoBalanceForNativeTokenRemainder);
                        continue;
                    }
                    // Currently too many inputs, by scanning for more inputs, we might find some with more amount
                    Err(crate::Error::ConsolidationRequired(v)) => {
                        cached_error.replace(crate::Error::ConsolidationRequired(v));
                        continue;
                    }
                    // Not enough balance for a remainder
                    Err(crate::Error::MessageError(message_error)) => match message_error {
                        bee_message::Error::InvalidStorageDepositAmount(v) => {
                            cached_error.replace(bee_message::Error::InvalidStorageDepositAmount(v).into());
                            continue;
                        }
                        _ => return Err(message_error.into()),
                    },
                    Err(e) => return Err(e),
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
            return Err(cached_error.unwrap_or(Error::NoInputs));
        }
    };

    Ok(selected_transaction_data)
}

async fn get_inputs_for_sender_and_issuer(
    message_builder: &ClientMessageBuilder<'_>,
) -> Result<(bool, Vec<InputSigningData>)> {
    log::debug!("[get_inputs_for_sender_and_issuer]");
    let mut force_use_all_inputs = false;
    let mut required_ed25519_inputs = Vec::new();
    let bech32_hrp = message_builder.client.get_bech32_hrp().await?;

    // get Ed25519 address if there is a Sender or Issuer block, because we then need to unlock an output with this
    // address
    let mut required_ed25519_addresses = Vec::new();
    for output in &message_builder.outputs {
        if let Some(features_blocks) = output.feature_blocks() {
            for feature_block in features_blocks.iter() {
                if let FeatureBlock::Sender(sender_feature_block) = feature_block {
                    required_ed25519_addresses.push(sender_feature_block.address());
                }
            }
        }
        if let Some(features_blocks) = output.immutable_feature_blocks() {
            for feature_block in features_blocks.iter() {
                if let FeatureBlock::Issuer(issuer_feature_block) = feature_block {
                    required_ed25519_addresses.push(issuer_feature_block.address());
                }
            }
        }
    }
    required_ed25519_addresses.dedup();
    for address in required_ed25519_addresses {
        if let Address::Ed25519(address) = address {
            let (address_index, internal) = search_address(
                message_builder
                    .secret_manager
                    .ok_or(Error::MissingParameter("secret manager"))?,
                &bech32_hrp,
                message_builder.coin_type,
                message_builder.account_index,
                message_builder.input_range.clone(),
                &Address::Ed25519(*address),
            )
            .await?;
            // if we didn't return with an error, then the address was found

            let output_ids = crate::node_api::indexer::routes::output_ids(
                message_builder.client,
                vec![
                    QueryParameter::Address(Address::Ed25519(*address).to_bech32(&bech32_hrp)),
                    QueryParameter::HasExpirationCondition(false),
                    QueryParameter::HasTimelockCondition(false),
                    QueryParameter::HasStorageDepositReturnCondition(false),
                ],
            )
            .await?;

            let address_outputs = crate::node_api::core::get_outputs(message_builder.client, output_ids).await?;
            match address_outputs.first() {
                Some(output_response) => {
                    required_ed25519_inputs.push(InputSigningData {
                        output: Output::try_from(&output_response.output)?,
                        output_metadata: OutputMetadata {
                            message_id: MessageId::from_str(&output_response.message_id)?,
                            transaction_id: TransactionId::from_str(&output_response.transaction_id)?,
                            output_index: output_response.output_index,
                            is_spent: output_response.is_spent,
                            milestone_index_spent: output_response.milestone_index_spent,
                            milestone_timestamp_spent: output_response.milestone_timestamp_spent,
                            transaction_id_spent: output_response
                                .transaction_id_spent
                                .as_ref()
                                .map(|s| TransactionId::from_str(s))
                                .transpose()?,
                            milestone_index_booked: output_response.milestone_index_booked,
                            milestone_timestamp_booked: output_response.milestone_timestamp_booked,
                            ledger_index: output_response.ledger_index,
                        },
                        chain: Some(Chain::from_u32_hardened(vec![
                            HD_WALLET_TYPE,
                            message_builder.coin_type,
                            message_builder.account_index,
                            internal as u32,
                            address_index,
                        ])),
                        bech32_address: Address::Ed25519(*address).to_bech32(&bech32_hrp),
                    });
                    // we want to include all outputs, because another output might be better balance wise,
                    // but will not unlock the address we need
                    force_use_all_inputs = true;
                }
                None => return Err(Error::MissingInputWithEd25519UnlockCondition),
            }
        }
    }
    Ok((force_use_all_inputs, required_ed25519_inputs))
}
