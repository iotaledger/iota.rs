// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use std::collections::HashSet;

use bee_block::{
    input::{Input, UtxoInput},
    output::{dto::OutputDto, InputsCommitment, Output, OutputId},
    payload::{
        transaction::{RegularTransactionEssence, TransactionEssence, TransactionPayload},
        Payload, TaggedDataPayload,
    },
    semantic::{semantic_validation, ConflictReason, ValidationContext},
};

use crate::{
    api::{types::PreparedTransactionData, ClientBlockBuilder},
    block::output::AliasId,
    secret::{types::InputSigningData, SecretManageExt},
    Error, Result,
};

impl<'a> ClientBlockBuilder<'a> {
    /// Prepare a transaction
    pub async fn prepare_transaction(&self) -> Result<PreparedTransactionData> {
        log::debug!("[prepare_transaction]");
        let rent_structure = self.client.get_rent_structure()?;
        let token_supply = self.client.get_token_supply()?;

        let mut governance_transition: Option<HashSet<AliasId>> = None;
        for output in &self.outputs {
            // Check if the outputs have enough amount to cover the storage deposit
            output.verify_storage_deposit(rent_structure.clone(), token_supply)?;
            if let Output::Alias(x) = output {
                if x.state_index() > 0 {
                    // Check if the transaction is a governance_transition, by checking if the new index is the same as
                    // the previous index
                    let output_id = self.client.alias_output_id(*x.alias_id()).await?;
                    let output_response = self.client.get_output(&output_id).await?;
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

        // Input selection
        let selected_transaction_data = if self.inputs.is_some() {
            self.get_custom_inputs(governance_transition, &rent_structure, self.allow_burning)
                .await?
        } else {
            self.get_inputs(&rent_structure).await?
        };

        // Build transaction payload
        let inputs_commitment = InputsCommitment::new(selected_transaction_data.inputs.iter().map(|i| &i.output));

        let mut essence = RegularTransactionEssence::builder(inputs_commitment);
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
        if let Some(index) = self.tag.clone() {
            let tagged_data_payload = TaggedDataPayload::new(index.to_vec(), self.data.clone().unwrap_or_default())?;
            essence = essence.with_payload(Payload::TaggedData(Box::new(tagged_data_payload)));
        }

        let regular_essence = essence.finish(&self.client.get_protocol_parameters()?)?;
        let essence = TransactionEssence::Regular(regular_essence);

        Ok(PreparedTransactionData {
            essence,
            inputs_data: selected_transaction_data.inputs,
            remainder: selected_transaction_data.remainder,
        })
    }

    /// Sign the transaction
    pub async fn sign_transaction(&self, prepared_transaction_data: PreparedTransactionData) -> Result<Payload> {
        log::debug!("[sign_transaction] {:?}", prepared_transaction_data);
        let secret_manager = self.secret_manager.ok_or(Error::MissingParameter("secret manager"))?;
        let unlocks = secret_manager
            .sign_transaction_essence(&prepared_transaction_data)
            .await?;
        let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

        let current_time = self.client.get_time_checked().await?;

        let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

        if conflict != ConflictReason::None {
            log::debug!("[sign_transaction] conflict: {conflict:?} for {:#?}", tx_payload);
            return Err(Error::TransactionSemantic(conflict));
        }

        Ok(Payload::Transaction(Box::new(tx_payload)))
    }
}

// TODO @thibault-martinez: this is very cumbersome with the current state, will refactor.
/// Verifies the semantic of a prepared transaction.
pub fn verify_semantic(
    input_signing_data: &[InputSigningData],
    transaction: &TransactionPayload,
    current_time: u32,
) -> crate::Result<ConflictReason> {
    let transaction_id = transaction.id();
    let TransactionEssence::Regular(essence) = transaction.essence();
    let output_ids = input_signing_data
        .iter()
        .map(InputSigningData::output_id)
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
        current_time,
    );

    semantic_validation(context, inputs.as_slice(), transaction.unlocks()).map_err(Error::BlockError)
}
