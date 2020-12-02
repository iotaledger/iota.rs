use iota::{
  AddressBalancePair, Ed25519Signature, Indexation, Input, Output, OutputMetadata, Payload,
  ReferenceUnlock, SignatureLockedSingleOutput, SignatureUnlock, Transaction, TransactionEssence,
  UTXOInput, UnlockBlock,
};
use serde::{Deserialize, Serialize};

use std::{
  convert::{TryFrom, TryInto},
  num::NonZeroU64,
  str::FromStr,
};

#[derive(Serialize, Deserialize)]
pub(super) struct OutputDto {
  address: String,
  amount: u64,
}

#[derive(Serialize, Deserialize)]
pub(super) struct MessageTransactionEssenceDto {
  inputs: Box<[String]>,
  outputs: Box<[OutputDto]>,
  payload: Option<Box<MessagePayloadDto>>,
}

impl TryFrom<MessageTransactionEssenceDto> for TransactionEssence {
  type Error = crate::Error;
  fn try_from(value: MessageTransactionEssenceDto) -> crate::Result<Self> {
    let mut builder = TransactionEssence::builder();

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
      .map(|output| {
        SignatureLockedSingleOutput::new(
          super::parse_address(output.address.clone())
            .unwrap_or_else(|_| panic!("invalid output address: {}", output.address)),
          NonZeroU64::new(output.amount).expect("output amount can't be zero"),
        )
        .into()
      })
      .collect();
    for output in outputs {
      builder = builder.add_output(output);
    }

    builder = match value.payload {
      Some(indexation) => builder.with_payload(
        (*indexation)
          .try_into()
          .expect("Invalid indexation in TransactionEssenceJson"),
      ),
      _ => builder,
    };

    Ok(builder.finish()?)
  }
}

#[derive(Serialize, Deserialize)]
pub(super) struct MessageSignatureUnlockDto {
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

#[derive(Serialize, Deserialize)]
pub(super) struct MessageUnlockBlockJsonDto {
  signature: Option<MessageSignatureUnlockDto>,
  reference: Option<u16>,
}

impl TryFrom<MessageUnlockBlockJsonDto> for UnlockBlock {
  type Error = crate::Error;

  fn try_from(value: MessageUnlockBlockJsonDto) -> crate::Result<Self> {
    let type_ = if value.signature.is_some() { 0 } else { 1 };
    match type_ {
      0 => {
        let sig: SignatureUnlock = value
          .signature
          .expect("Must contain signature.")
          .try_into()?;
        Ok(sig.into())
      }
      1 => {
        let reference: ReferenceUnlock = value
          .reference
          .expect("Must contain reference.")
          .try_into()?;
        Ok(reference.into())
      }
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub(super) struct MessageTransactionPayloadDto {
  essence: MessageTransactionEssenceDto,
  #[serde(rename = "unlockBlocks")]
  unlock_blocks: Box<[MessageUnlockBlockJsonDto]>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct MessageIndexationPayloadDto {
  index: String,
  data: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum MessagePayloadDto {
  /// The transaction payload.
  Transaction(MessageTransactionPayloadDto),
  /// The indexation payload.
  Indexation(MessageIndexationPayloadDto),
}

#[derive(Serialize, Deserialize)]
pub(super) struct MessageDto {
  pub parent1: String,
  pub parent2: String,
  pub payload: MessagePayloadDto,
  pub nonce: u64,
}

impl TryFrom<MessagePayloadDto> for Payload {
  type Error = crate::Error;
  fn try_from(payload: MessagePayloadDto) -> crate::Result<Self> {
    match payload {
      MessagePayloadDto::Transaction(transaction_payload) => {
        let mut transaction = Transaction::builder();
        transaction = transaction.with_essence(transaction_payload.essence.try_into()?);

        let unlock_blocks = transaction_payload.unlock_blocks.into_vec();
        for unlock_block in unlock_blocks {
          transaction = transaction.add_unlock_block(unlock_block.try_into()?);
        }

        Ok(Payload::Transaction(Box::new(transaction.finish()?)))
      }
      MessagePayloadDto::Indexation(indexation_payload) => {
        let indexation =
          Indexation::new(indexation_payload.index, indexation_payload.data.as_bytes()).unwrap();
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
    Self {
      message_id: hex::encode(value.message_id),
      transaction_id: hex::encode(value.transaction_id),
      output_index: value.output_index,
      is_spent: value.is_spent,
      address: value.address.to_bech32(),
      amount: value.amount,
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
      address: value.address.to_bech32(),
      balance: value.balance,
    }
  }
}
