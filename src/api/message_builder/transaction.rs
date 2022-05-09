// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use std::collections::HashSet;

use bee_message::{
    address::Address,
    input::{Input, UtxoInput},
    output::{dto::OutputDto, InputsCommitment, Output, OutputId},
    payload::{
        milestone::MilestoneIndex,
        transaction::{RegularTransactionEssence, TransactionEssence, TransactionPayload},
        Payload, TaggedDataPayload,
    },
    semantic::{semantic_validation, ConflictReason, ValidationContext},
    unlock_block::UnlockBlocks,
};

use crate::{
    api::{types::PreparedTransactionData, ClientMessageBuilder},
    bee_message::output::AliasId,
    secret::{types::InputSigningData, SecretManageExt, SignMessageMetadata},
    Error, Result,
};

/// Prepare a transaction
pub async fn prepare_transaction(message_builder: &ClientMessageBuilder<'_>) -> Result<PreparedTransactionData> {
    log::debug!("[prepare_transaction]");
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
            .get_custom_inputs(governance_transition, &byte_cost_config, message_builder.allow_burning)
            .await?
    } else {
        message_builder.get_inputs(&byte_cost_config).await?
    };

    // Build transaction payload
    let inputs_commitment = InputsCommitment::new(selected_transaction_data.inputs.iter().map(|i| &i.output));

    let mut essence =
        RegularTransactionEssence::builder(message_builder.client.get_network_id().await?, inputs_commitment);
    let inputs = selected_transaction_data
        .inputs
        .iter()
        .map(|i| {
            Ok(Input::Utxo(UtxoInput::new(
                i.output_metadata.transaction_id,
                i.output_metadata.output_index,
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
        input_signing_data_entries: selected_transaction_data.inputs,
    })
}

/// Sign the transaction
pub async fn sign_transaction(
    message_builder: &ClientMessageBuilder<'_>,
    prepared_transaction_data: PreparedTransactionData,
) -> Result<Payload> {
    log::debug!("[sign_transaction]");
    let mut input_addresses = Vec::new();
    for input_signing_data in &prepared_transaction_data.input_signing_data_entries {
        let (_bech32_hrp, address) = Address::try_from_bech32(&input_signing_data.bech32_address)?;
        input_addresses.push(address);
    }
    let secret_manager = message_builder
        .secret_manager
        .ok_or(Error::MissingParameter("secret manager"))?;
    let unlock_blocks = secret_manager
        .sign_transaction_essence(
            // IOTA_COIN_TYPE,
            // message_builder.account_index.unwrap_or(0),
            &prepared_transaction_data.essence,
            &prepared_transaction_data.input_signing_data_entries,
            // todo set correct data
            SignMessageMetadata {
                remainder_value: 0,
                remainder_deposit_address: None,
            },
        )
        .await?;
    let unlock_blocks = UnlockBlocks::new(unlock_blocks)?;
    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlock_blocks)?;

    let (local_time, milestone_index) = message_builder.client.get_time_and_milestone_checked().await?;

    let conflict = verify_semantic(
        &prepared_transaction_data.input_signing_data_entries,
        &tx_payload,
        milestone_index,
        local_time,
    )?;

    if conflict != ConflictReason::None {
        return Err(Error::TransactionSemantic(conflict));
    }

    Ok(Payload::Transaction(Box::new(tx_payload)))
}

// TODO @thibault-martinez: this is very cumbersome with the current state, will refactor.
/// Verifies the semantic of a prepared transaction.
pub fn verify_semantic(
    input_signing_data: &[InputSigningData],
    transaction: &TransactionPayload,
    milestone_index: u32,
    local_time: u32,
) -> crate::Result<ConflictReason> {
    let transaction_id = transaction.id();
    let TransactionEssence::Regular(essence) = transaction.essence();
    let output_ids = input_signing_data
        .iter()
        .map(|i| i.output_id())
        .collect::<Result<Vec<OutputId>>>()?;
    let outputs = input_signing_data
        .iter()
        .map(|i| i.output.clone())
        .collect::<Vec<Output>>();
    let inputs = output_ids
        .into_iter()
        .zip(outputs.iter())
        .collect::<Vec<(OutputId, &Output)>>();

    let context = ValidationContext::new(
        &transaction_id,
        essence,
        inputs.iter().map(|(id, input)| (id, *input)),
        transaction.unlock_blocks(),
        MilestoneIndex(milestone_index),
        local_time,
    );

    semantic_validation(context, inputs.as_slice(), transaction.unlock_blocks()).map_err(Error::MessageError)
}
