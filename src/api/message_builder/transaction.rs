// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use crate::{
    api::{types::PreparedTransactionData, ClientMessageBuilder},
    signing::{ verify_unlock_blocks, Network, SignMessageMetadata},
    Error, Result,
};

use crate::bee_message::output::AliasId;
use bee_message::{
    address::{Address},
    input::INPUT_COUNT_MAX,
    output::Output,
    payload::{
        transaction::{RegularTransactionEssence, TransactionEssence, TransactionPayloadBuilder},
        Payload, TaggedDataPayload,
    },
    unlock_block::UnlockBlocks,
};
use bee_rest_api::types::dtos::OutputDto;
use packable::PackableExt;

use std::collections::{hash_map::Entry, HashMap, HashSet};

/// Prepare a transaction
pub async fn prepare_transaction(message_builder: &ClientMessageBuilder<'_>) -> Result<PreparedTransactionData> {
    let mut governance_transition: Option<HashSet<AliasId>> = None;
    // Calculate the total tokens to spend
    let mut total_to_spend = 0;
    let mut native_tokens = HashMap::new();
    for output in &message_builder.outputs {
        total_to_spend += output.amount();
        if let Some(output_native_tokens) = output.native_tokens() {
            for native_token in output_native_tokens {
                match native_tokens.entry(*native_token.token_id()) {
                    Entry::Vacant(e) => {
                        e.insert(*native_token.amount());
                    }
                    Entry::Occupied(mut e) => {
                        *e.get_mut() += *native_token.amount();
                    }
                }
            }
        }
        if let Output::Alias(x) = output {
            if x.state_index() > 0 {
                // Check if the transaction is a governance_transition, by checking if the new index is the same as
                // the previous index
                let output_ids = message_builder.client.alias_output_ids(*x.alias_id()).await?;
                let outputs = message_builder.client.get_outputs(output_ids).await?;
                for output in outputs {
                    if let OutputDto::Alias(output) = output.output {
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
    }

    // Inputselection
    let (inputs_for_essence, mut outputs_for_essence, input_signing_data_entrys) = match &message_builder.inputs {
        Some(inputs) => {
            // 128 is the maximum input amount
            if inputs.len() > INPUT_COUNT_MAX.into() {
                return Err(Error::ConsolidationRequired(inputs.len()));
            }
            message_builder
                .get_custom_inputs(inputs, total_to_spend, native_tokens, governance_transition)
                .await?
        }
        None => message_builder.get_inputs(total_to_spend, native_tokens).await?,
    };

    // Build signed transaction payload
    for output in message_builder.outputs.clone() {
        outputs_for_essence.push(output);
    }

    let mut essence = RegularTransactionEssence::builder(message_builder.client.get_network_id().await?);
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
        input_signing_data_entrys,
    })
}

/// Sign the transaction
pub async fn sign_transaction(
    message_builder: &ClientMessageBuilder<'_>,
    prepared_transaction_data: PreparedTransactionData,
) -> Result<Payload> {
    let mut tx_inputs = Vec::new();
    let mut input_addresses = Vec::new();
    for input_signing_data in prepared_transaction_data.input_signing_data_entrys {
        // let output = Output::try_from(&input_signing_data.output_response.output)?;
        // let alias_or_nft_address: Option<Address> = match &output {
        //     Output::Alias(a) => Some(Address::Alias(AliasAddress::new(*a.alias_id()))),
        //     Output::Nft(a) => Some(Address::Nft(NftAddress::new(*a.nft_id()))),
        //     _ => None,
        // };
        let address = Address::try_from_bech32(&input_signing_data.bech32_address)?;
        // tx_inputs.push(InputSigningData {
        //     input: input_signing_data.input,
        //     address_index: input_signing_data.address_index,
        //     address_internal: input_signing_data.internal,
        //     output_kind: output.kind(),
        //     address,
        //     alias_or_nft_address,
        // });
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
