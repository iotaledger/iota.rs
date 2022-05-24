// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use std::collections::HashSet;

use bee_block::{
    address::Address,
    input::{Input, UtxoInput},
    output::{dto::OutputDto, InputsCommitment, Output, OutputId},
    payload::{
        milestone::MilestoneIndex,
        transaction::{RegularTransactionEssence, TransactionEssence, TransactionPayload},
        Payload, TaggedDataPayload,
    },
    semantic::{semantic_validation, ConflictReason, ValidationContext},
};

use crate::{
    api::{types::PreparedTransactionData, ClientBlockBuilder},
    bee_block::output::AliasId,
    secret::{types::InputSigningData, SecretManageExt},
    Error, Result,
};

/// Prepare a transaction
pub async fn prepare_transaction(block_builder: &ClientBlockBuilder<'_>) -> Result<PreparedTransactionData> {
    log::debug!("[prepare_transaction]");
    let byte_cost_config = block_builder.client.get_byte_cost_config().await?;

    let mut governance_transition: Option<HashSet<AliasId>> = None;
    for output in &block_builder.outputs {
        // Check if the outputs have enough amount to cover the storage deposit
        output.verify_storage_deposit(&byte_cost_config)?;
        if let Output::Alias(x) = output {
            if x.state_index() > 0 {
                // Check if the transaction is a governance_transition, by checking if the new index is the same as
                // the previous index
                let output_id = block_builder.client.alias_output_id(*x.alias_id()).await?;
                let output_response = block_builder.client.get_output(&output_id).await?;
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
    let selected_transaction_data = if block_builder.inputs.is_some() {
        block_builder
            .get_custom_inputs(governance_transition, &byte_cost_config, block_builder.allow_burning)
            .await?
    } else {
        block_builder.get_inputs(&byte_cost_config).await?
    };

    // Build transaction payload
    let inputs_commitment = InputsCommitment::new(selected_transaction_data.inputs.iter().map(|i| &i.output));

    let mut essence =
        RegularTransactionEssence::builder(block_builder.client.get_network_id().await?, inputs_commitment);
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
    if let Some(index) = block_builder.tag.clone() {
        let tagged_data_payload =
            TaggedDataPayload::new((&index).to_vec(), block_builder.data.clone().unwrap_or_default())?;
        essence = essence.with_payload(Payload::TaggedData(Box::new(tagged_data_payload)))
    }
    let regular_essence = essence.finish()?;
    let essence = TransactionEssence::Regular(regular_essence);

    Ok(PreparedTransactionData {
        essence,
        inputs_data: selected_transaction_data.inputs,
        remainder: selected_transaction_data.remainder,
    })
}

/// Sign the transaction
pub async fn sign_transaction(
    block_builder: &ClientBlockBuilder<'_>,
    prepared_transaction_data: PreparedTransactionData,
) -> Result<Payload> {
    log::debug!("[sign_transaction]");
    let mut input_addresses = Vec::new();
    for input_signing_data in &prepared_transaction_data.inputs_data {
        let (_bech32_hrp, address) = Address::try_from_bech32(&input_signing_data.bech32_address)?;
        input_addresses.push(address);
    }
    let secret_manager = block_builder
        .secret_manager
        .ok_or(Error::MissingParameter("secret manager"))?;
    let unlocks = secret_manager
        .sign_transaction_essence(
            // IOTA_COIN_TYPE,
            // block_builder.account_index.unwrap_or(0),
            &prepared_transaction_data,
        )
        .await?;
    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    let (local_time, milestone_index) = block_builder.client.get_time_and_milestone_checked().await?;

    let conflict = verify_semantic(
        &prepared_transaction_data.inputs_data,
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
        transaction.unlocks(),
        MilestoneIndex(milestone_index),
        local_time,
    );

    semantic_validation(context, inputs.as_slice(), transaction.unlocks()).map_err(Error::BlockError)
}
