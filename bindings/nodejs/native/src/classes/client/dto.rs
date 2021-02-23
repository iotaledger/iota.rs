// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{
    AddressDto, BalanceForAddressResponse as AddressBalancePair, Ed25519Signature, Essence, IndexationPayload, Input,
    Message, MessageId, Output, OutputDto as BeeOutput, OutputResponse as OutputMetadata, Payload, ReferenceUnlock,
    RegularEssence, SignatureUnlock, TransactionPayload, UTXOInput, UnlockBlock,
};
use serde::{Deserialize, Serialize};

use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageWrapper {
    pub message: Message,
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
                UTXOInput::from_str(&input)
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
        let signature = hex::decode(value.signature)?.into_boxed_slice();
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

                let unlock_blocks = transaction_payload.unlock_blocks.into_vec();
                for unlock_block in unlock_blocks {
                    transaction = transaction.add_unlock_block(unlock_block.try_into()?);
                }

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
pub(super) struct OutputMetadataDto {
    /// Message ID of the output
    #[serde(rename = "messageId")]
    message_id: String,
    /// Transaction ID of the output
    #[serde(rename = "transactionId")]
    transaction_id: String,
    /// Output index.
    #[serde(rename = "outputIndex")]
    output_index: u16,
    /// Spend status of the output
    #[serde(rename = "isSpent")]
    is_spent: bool,
    /// Corresponding address
    address: String,
    /// Balance amount
    amount: u64,
}

impl From<OutputMetadata> for OutputMetadataDto {
    fn from(value: OutputMetadata) -> Self {
        let (output_amount, output_address) = match value.output {
            BeeOutput::Treasury(t) => (t.amount, "".to_string()),
            BeeOutput::SignatureLockedSingle(r) => match r.address {
                AddressDto::Ed25519(addr) => (r.amount, addr.address),
            },
            BeeOutput::SignatureLockedDustAllowance(r) => match r.address {
                AddressDto::Ed25519(addr) => (r.amount, addr.address),
            },
        };

        Self {
            message_id: value.message_id,
            transaction_id: value.transaction_id,
            output_index: value.output_index,
            is_spent: value.is_spent,
            address: output_address,
            amount: output_amount,
        }
    }
}

#[derive(Serialize)]
pub(super) struct AddressBalanceDto {
    address: String,
    balance: u64,
}

impl From<AddressBalancePair> for AddressBalanceDto {
    fn from(value: AddressBalancePair) -> Self {
        Self {
            address: value.address.to_string(),
            balance: value.balance,
        }
    }
}
