// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use crate::{
    api::{types::PreparedTransactionData, ClientMessageBuilder},
    signing::{mnemonic::IOTA_COIN_TYPE, verify_unlock_blocks, Network, SignMessageMetadata, TransactionInput},
    Error, Result,
};

use bee_message::{
    address::Address,
    input::INPUT_COUNT_MAX,
    output::Output,
    payload::{
        transaction::{RegularTransactionEssence, TransactionEssence, TransactionPayloadBuilder},
        Payload, TaggedDataPayload,
    },
    unlock_block::UnlockBlocks,
};
use packable::PackableExt;

/// Prepare a transaction
pub async fn prepare_transaction(message_builder: &ClientMessageBuilder<'_>) -> Result<PreparedTransactionData> {
    // Calculate the total tokens to spend
    let mut total_to_spend = 0;
    for output in &message_builder.outputs {
        match output {
            Output::Extended(x) => {
                total_to_spend += x.amount();
            }
            Output::Alias(x) => {
                total_to_spend += x.amount();
            }
            Output::Foundry(x) => {
                total_to_spend += x.amount();
            }
            Output::Nft(x) => {
                total_to_spend += x.amount();
            }
            // todo: support other output types
            _ => {}
        }
    }

    // Inputselection
    let (inputs_for_essence, mut outputs_for_essence, address_index_recorders) = match &message_builder.inputs {
        Some(inputs) => {
            // 127 is the maximum input amount
            if inputs.len() > INPUT_COUNT_MAX.into() {
                return Err(Error::ConsolidationRequired(inputs.len()));
            }
            message_builder.get_custom_inputs(inputs, total_to_spend).await?
        }
        None => message_builder.get_inputs(total_to_spend).await?,
    };

    // Build signed transaction payload
    for output in message_builder.outputs.clone() {
        outputs_for_essence.push(output);
    }

    // let mut essence = RegularTransactionEssence::builder(message_builder.client.get_network_id().await?);
    let mut essence = RegularTransactionEssence::builder();
    essence = essence.with_inputs(inputs_for_essence);

    // todo remove this, because ordering isn't required anymore?
    // Order outputs and add them to the essence
    outputs_for_essence.sort_unstable_by_key(|a| a.pack_to_vec());
    essence = essence.with_outputs(outputs_for_essence);

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
        address_index_recorders,
    })
}

/// Sign the transaction
pub async fn sign_transaction(
    message_builder: &ClientMessageBuilder<'_>,
    prepared_transaction_data: PreparedTransactionData,
) -> Result<Payload> {
    let mut tx_inputs = Vec::new();
    let mut input_addresses = Vec::new();
    for address_index_recorder in prepared_transaction_data.address_index_recorders {
        tx_inputs.push(TransactionInput {
            input: address_index_recorder.input,
            address_index: address_index_recorder.address_index,
            address_internal: address_index_recorder.internal,
            output_kind: Output::try_from(&address_index_recorder.output.output)?.kind(),
        });
        input_addresses.push(Address::try_from_bech32(&address_index_recorder.bech32_address)?);
    }
    let signer = message_builder.signer.ok_or(Error::MissingParameter("signer"))?;
    #[cfg(feature = "wasm")]
    let mut signer = signer.lock().unwrap();
    #[cfg(not(feature = "wasm"))]
    let mut signer = signer.lock().await;
    let unlock_blocks = signer
        .sign_transaction_essence(
            IOTA_COIN_TYPE,
            message_builder.account_index.unwrap_or(0),
            &prepared_transaction_data.essence,
            &mut tx_inputs,
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
    let tx_payload = TransactionPayloadBuilder::new()
        .with_essence(prepared_transaction_data.essence)
        .with_unlock_blocks(unlock_blocks)
        .finish()
        .map_err(|_| Error::TransactionError)?;

    // validate the signatures in the unlock blocks so we don't send invalid transactions
    verify_unlock_blocks(&tx_payload, input_addresses)?;
    Ok(Payload::Transaction(Box::new(tx_payload)))
}
