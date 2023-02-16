// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use iota_types::block::{
    input::{Input, UtxoInput},
    output::{InputsCommitment, Output, OutputId},
    payload::{
        transaction::{RegularTransactionEssence, TransactionEssence, TransactionPayload},
        Payload, TaggedDataPayload,
    },
    semantic::{semantic_validation, ConflictReason, ValidationContext},
    signature::Ed25519Signature,
    Block, BlockId,
};
use packable::PackableExt;

use crate::{
    api::{types::PreparedTransactionData, ClientBlockBuilder},
    secret::{types::InputSigningData, SecretManageExt},
    Error, Result,
};

const MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS: usize = Block::LENGTH_MAX - Block::LENGTH_MIN - (7 * BlockId::LENGTH);
// Length for unlocks with a single signature unlock (unlocks length + unlock type + signature type + public key +
// signature)
const SINGLE_UNLOCK_LENGTH: usize = 1 + 1 + Ed25519Signature::PUBLIC_KEY_LENGTH + Ed25519Signature::SIGNATURE_LENGTH;
// Type + reference index
const REFERENCE_ALIAS_NFT_UNLOCK_LENGTH: usize = 1 + 2;

impl<'a> ClientBlockBuilder<'a> {
    /// Prepare a transaction
    pub async fn prepare_transaction(&self) -> Result<PreparedTransactionData> {
        log::debug!("[prepare_transaction]");
        let protocol_parameters = self.client.get_protocol_parameters().await?;
        let token_supply = self.client.get_token_supply().await?;

        for output in &self.outputs {
            // Check if the outputs have enough amount to cover the storage deposit
            output.verify_storage_deposit(protocol_parameters.rent_structure().clone(), token_supply)?;
        }

        // Input selection
        let selected_transaction_data = if self.inputs.is_some() {
            self.get_custom_inputs(&protocol_parameters, self.burn.clone()).await?
        } else {
            self.get_inputs(&protocol_parameters).await?
        };

        // Build transaction payload
        let inputs_commitment = InputsCommitment::new(selected_transaction_data.inputs.iter().map(|i| &i.output));

        let mut essence = RegularTransactionEssence::builder(self.client.get_network_id().await?, inputs_commitment);
        let inputs = selected_transaction_data
            .inputs
            .iter()
            .map(|i| {
                Ok(Input::Utxo(UtxoInput::new(
                    *i.output_metadata.transaction_id(),
                    i.output_metadata.output_index(),
                )?))
            })
            .collect::<Result<Vec<Input>>>()?;
        essence = essence.with_inputs(inputs);

        essence = essence.with_outputs(selected_transaction_data.outputs);

        // Add tagged data payload if tag set
        if let Some(index) = self.tag.clone() {
            let tagged_data_payload = TaggedDataPayload::new(index.to_vec(), self.data.clone().unwrap_or_default())?;
            essence = essence.with_payload(Payload::from(tagged_data_payload));
        }

        let regular_essence = essence.finish(&self.client.get_protocol_parameters().await?)?;

        validate_regular_transaction_essence_length(&regular_essence)?;

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
        let current_time = self.client.get_time_checked().await?;

        let unlocks = secret_manager
            .sign_transaction_essence(&prepared_transaction_data, Some(current_time))
            .await?;
        let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

        validate_transaction_payload_length(&tx_payload)?;

        let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

        if conflict != ConflictReason::None {
            log::debug!("[sign_transaction] conflict: {conflict:?} for {:#?}", tx_payload);
            return Err(Error::TransactionSemantic(conflict));
        }

        Ok(Payload::from(tx_payload))
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
    let output_ids = input_signing_data.iter().map(|input| *input.output_id());
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

    Ok(semantic_validation(context, inputs.as_slice(), transaction.unlocks())?)
}

/// Verifies that the transaction payload doesn't exceed the block size limit with 8 parents.
pub fn validate_transaction_payload_length(transaction_payload: &TransactionPayload) -> Result<()> {
    let transaction_payload_bytes = transaction_payload.pack_to_vec();
    if transaction_payload_bytes.len() > MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS {
        return Err(Error::InvalidTransactionPayloadLength {
            length: transaction_payload_bytes.len(),
            max_length: MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS,
        });
    }
    Ok(())
}

/// Verifies that the transaction essence doesn't exceed the block size limit with 8 parents.
/// Assuming one signature unlock and otherwise reference/alias/nft unlocks. `validate_transaction_payload_length()`
/// should later be used to check the length again with the correct unlocks.
pub fn validate_regular_transaction_essence_length(
    regular_transaction_essence: &RegularTransactionEssence,
) -> Result<()> {
    let regular_transaction_essence_bytes = regular_transaction_essence.pack_to_vec();

    // Assuming there is only 1 signature unlock and the rest is reference/alias/nft unlocks
    let reference_alias_nft_unlocks_amount = regular_transaction_essence.inputs().len() - 1;

    // Max tx payload length - length for one signature unlock (there might be more unlocks, we check with them
    // later again, when we built the transaction payload)
    let max_length = MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS
        - SINGLE_UNLOCK_LENGTH
        - (reference_alias_nft_unlocks_amount * REFERENCE_ALIAS_NFT_UNLOCK_LENGTH);

    if regular_transaction_essence_bytes.len() > max_length {
        return Err(Error::InvalidRegularTransactionEssenceLength {
            length: regular_transaction_essence_bytes.len(),
            max_length,
        });
    }
    Ok(())
}
