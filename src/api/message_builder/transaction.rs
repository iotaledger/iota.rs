// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use crate::{
    api::{types::PreparedTransactionData, ClientMessageBuilder},
    signing::{verify_unlock_blocks, Network, SignMessageMetadata},
    Error, Result,
};

use crate::bee_message::output::AliasId;
use bee_message::{
    address::Address,
    input::{Input, UtxoInput},
    output::{dto::OutputDto, Output},
    payload::{
        transaction::{RegularTransactionEssence, TransactionEssence, TransactionId, TransactionPayload},
        Payload, TaggedDataPayload,
    },
    unlock_block::UnlockBlocks,
};
use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::PackableExt;

use std::{collections::HashSet, str::FromStr};

/// Prepare a transaction
pub async fn prepare_transaction(message_builder: &ClientMessageBuilder<'_>) -> Result<PreparedTransactionData> {
    log::debug!("[prepare_transaction]");
    // To be enabled when https://github.com/iotaledger/iota.rs/issues/830 gets implemented also for the difficulty
    // #[cfg(feature = "wasm")]
    // // With wasm we don't have the background syncing of nodes which updates the NetworkInfo in an interval, so we
    // need // to get the data every time we send a function, because the data could otherwise be outdated (the
    // values could be // changed via milestones later)
    // let rent_structure = message_builder
    //     .client
    //     .get_info()
    //     .await?
    //     .nodeinfo
    //     .protocol
    //     .rent_structure;
    // #[cfg(not(feature = "wasm"))]
    let byte_cost_config = message_builder.client.get_byte_cost_config().await?;

    let mut governance_transition: Option<HashSet<AliasId>> = None;
    for output in &message_builder.outputs {
        // Check if the outputs have enough amount to cover the storage deposit
        output.verify_storage_deposit(&byte_cost_config)?;
        if let Output::Alias(x) = output {
            if x.state_index() > 0 {
                // Check if the transaction is a governance_transition, by checking if the new index is the same as
                // the previous index
                let output_id = message_builder.client.alias_output_id(*x.alias_id()).await?;
                let output_response = message_builder.client.get_output(&output_id).await?;
                if let OutputDto::Alias(output) = output_response.output {
                    // A governance transition is identified by an unchanged State Index in next state.
                    if x.state_index() == output.state_index {
                        let mut transitions = HashSet::new();
                        transitions.insert(AliasId::try_from(&output.alias_id)?);
                        governance_transition.replace(transitions);
                    }
                }
            }
        }
    }

    // Inputselection
    let selected_transaction_data = if message_builder.inputs.is_some() {
        message_builder
            .get_custom_inputs(governance_transition, &byte_cost_config)
            .await?
    } else {
        message_builder.get_inputs(&byte_cost_config).await?
    };

    // Build transaction payload
    let input_outputs = selected_transaction_data
        .inputs
        .iter()
        .map(|i| Ok(Output::try_from(&i.output_response.output)?.pack_to_vec()))
        .collect::<Result<Vec<Vec<u8>>>>()?;
    let input_outputs = input_outputs.into_iter().flatten().collect::<Vec<u8>>();
    let inputs_commitment = Blake2b256::digest(&input_outputs)
        .try_into()
        .map_err(|_e| crate::Error::Blake2b256Error("Hashing outputs for inputs_commitment failed."))?;

    let mut essence =
        RegularTransactionEssence::builder(message_builder.client.get_network_id().await?, inputs_commitment);
    let inputs = selected_transaction_data
        .inputs
        .iter()
        .map(|i| {
            Ok(Input::Utxo(UtxoInput::new(
                TransactionId::from_str(&i.output_response.transaction_id)?,
                i.output_response.output_index,
            )?))
        })
        .collect::<Result<Vec<Input>>>()?;
    essence = essence.with_inputs(inputs);

    essence = essence.with_outputs(selected_transaction_data.outputs);

    // Add tagged data payload if tag set
    if let Some(index) = message_builder.tag.clone() {
        let tagged_data_payload =
            TaggedDataPayload::new((&index).to_vec(), message_builder.data.clone().unwrap_or_default())?;
        essence = essence.with_payload(Payload::TaggedData(Box::new(tagged_data_payload)))
    }
    let regular_essence = essence.finish()?;
    let essence = TransactionEssence::Regular(regular_essence);

    Ok(PreparedTransactionData {
        essence,
        input_signing_data_entrys: selected_transaction_data.inputs,
    })
}

/// Sign the transaction
pub async fn sign_transaction(
    message_builder: &ClientMessageBuilder<'_>,
    mut prepared_transaction_data: PreparedTransactionData,
) -> Result<Payload> {
    log::debug!("[sign_transaction]");
    let mut input_addresses = Vec::new();
    for input_signing_data in &prepared_transaction_data.input_signing_data_entrys {
        let (_bech32_hrp, address) = Address::try_from_bech32(&input_signing_data.bech32_address)?;
        input_addresses.push(address);
    }
    let signer = message_builder.signer.ok_or(Error::MissingParameter("signer"))?;
    #[cfg(feature = "wasm")]
    let mut signer = signer.lock().unwrap();
    #[cfg(not(feature = "wasm"))]
    let mut signer = signer.lock().await;
    let unlock_blocks = signer
        .sign_transaction_essence(
            // IOTA_COIN_TYPE,
            // message_builder.account_index.unwrap_or(0),
            &prepared_transaction_data.essence,
            &mut prepared_transaction_data.input_signing_data_entrys,
            // todo set correct data
            SignMessageMetadata {
                remainder_value: 0,
                remainder_deposit_address: None,
                network: match message_builder.client.get_network_id().await? {
                    1454675179895816119 => Network::Mainnet,
                    _ => Network::Testnet,
                },
            },
        )
        .await?;
    let unlock_blocks = UnlockBlocks::new(unlock_blocks)?;
    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence, unlock_blocks)?;

    // validate the signatures in the unlock blocks so we don't send invalid transactions
    verify_unlock_blocks(&tx_payload, input_addresses)?;
    Ok(Payload::Transaction(Box::new(tx_payload)))
}
