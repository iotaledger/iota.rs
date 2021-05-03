// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    bee_message::prelude::{
        Ed25519Signature, Essence, IndexationPayload, Input, MessageId, Output, Payload, ReferenceUnlock,
        RegularEssence, SignatureUnlock, TransactionPayload, UnlockBlock, UnlockBlocks, UtxoInput,
    },
    bee_rest_api::types::dtos::{MessageDto as BeeMessageDto, OutputDto as BeeOutput},
};
use serde::{Deserialize, Serialize};

use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageWrapper {
    pub message: BeeMessageDto,
    #[serde(rename = "messageId")]
    pub message_id: MessageId,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageRegularEssenceDto {
    inputs: Box<[String]>,
    outputs: Box<[BeeOutput]>,
    payload: Option<Box<MessagePayloadDto>>,
}

impl TryFrom<MessageRegularEssenceDto> for RegularEssence {
    type Error = crate::Error;
    fn try_from(value: MessageRegularEssenceDto) -> crate::Result<Self> {
        let mut builder = RegularEssence::builder();

        let inputs: Vec<Input> = value
            .inputs
            .into_vec()
            .into_iter()
            .map(|input| {
                UtxoInput::from_str(&input)
                    .unwrap_or_else(|_| panic!("invalid input: {}", input))
                    .into()
            })
            .collect();
        for input in inputs {
            builder = builder.add_input(input);
        }

        let outputs: Vec<Output> = value
            .outputs
            .into_vec()
            .into_iter()
            .map(|output| Output::try_from(&output).unwrap())
            .collect();
        for output in outputs {
            builder = builder.add_output(output);
        }

        builder = match value.payload {
            Some(indexation) => builder.with_payload(
                (*indexation)
                    .try_into()
                    .expect("Invalid indexation in RegularEssenceJson"),
            ),
            _ => builder,
        };

        Ok(builder.finish()?)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageSignatureUnlockDto {
    #[serde(rename = "publicKey")]
    public_key: String,
    signature: String,
}

impl TryFrom<MessageSignatureUnlockDto> for SignatureUnlock {
    type Error = crate::Error;

    fn try_from(value: MessageSignatureUnlockDto) -> crate::Result<Self> {
        let mut public_key = [0u8; 32];
        hex::decode_to_slice(value.public_key, &mut public_key)?;
        let mut signature = [0u8; 64];
        hex::decode_to_slice(value.signature, &mut signature)?;
        Ok(Ed25519Signature::new(public_key, signature).into())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageUnlockBlockJsonDto {
    signature: Option<MessageSignatureUnlockDto>,
    reference: Option<u16>,
}

impl TryFrom<MessageUnlockBlockJsonDto> for UnlockBlock {
    type Error = crate::Error;

    fn try_from(value: MessageUnlockBlockJsonDto) -> crate::Result<Self> {
        let type_ = if value.signature.is_some() { 0 } else { 1 };
        match type_ {
            0 => {
                let sig: SignatureUnlock = value.signature.expect("Must contain signature.").try_into()?;
                Ok(sig.into())
            }
            1 => {
                let reference: ReferenceUnlock = value.reference.expect("Must contain reference.").try_into()?;
                Ok(reference.into())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageTransactionPayloadDto {
    essence: MessageRegularEssenceDto,
    #[serde(rename = "unlockBlocks")]
    unlock_blocks: Box<[MessageUnlockBlockJsonDto]>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageIndexationPayloadDto {
    index: Vec<u8>,
    data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessagePayloadDto {
    /// The transaction payload.
    Transaction(MessageTransactionPayloadDto),
    /// The indexation payload.
    Indexation(MessageIndexationPayloadDto),
}

#[derive(Serialize, Deserialize)]
pub struct MessageDto {
    pub parents: Option<Vec<String>>,
    pub payload: MessagePayloadDto,
}

impl TryFrom<MessagePayloadDto> for Payload {
    type Error = crate::Error;
    fn try_from(payload: MessagePayloadDto) -> crate::Result<Self> {
        match payload {
            MessagePayloadDto::Transaction(transaction_payload) => {
                let mut transaction = TransactionPayload::builder();
                transaction = transaction.with_essence(Essence::Regular(transaction_payload.essence.try_into()?));

                let unlock_blocks: Result<Vec<UnlockBlock>, crate::Error> = transaction_payload
                    .unlock_blocks
                    .into_vec()
                    .into_iter()
                    .map(|u| u.try_into())
                    .collect();

                transaction = transaction.with_unlock_blocks(UnlockBlocks::new(unlock_blocks?)?);

                Ok(Payload::Transaction(Box::new(transaction.finish()?)))
            }
            MessagePayloadDto::Indexation(indexation_payload) => {
                let indexation = IndexationPayload::new(&indexation_payload.index, &indexation_payload.data)?;
                Ok(Payload::Indexation(Box::new(indexation)))
            }
        }
    }
}

#[derive(Serialize)]
pub(crate) struct OutputMetadataDto {
    /// Message ID of the output
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Transaction ID of the output
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Output index.
    #[serde(rename = "outputIndex")]
    pub output_index: u16,
    /// Spend status of the output
    #[serde(rename = "isSpent")]
    pub is_spent: bool,
    /// Corresponding address
    pub address: String,
    /// Balance amount
    pub amount: u64,
}

#[derive(Serialize)]
pub(crate) struct AddressBalanceDto {
    pub address: String,
    pub balance: u64,
    #[serde(rename = "dustAllowed")]
    pub dust_allowed: bool,
}
