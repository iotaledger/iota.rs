//! Types of several IOTA APIs related objects
use crate::{Error, Result};

use bee_message::prelude::*;

use std::convert::{From, TryFrom, TryInto};

/// Marker trait for response
pub trait ResponseType {}

impl ResponseType for Message {}

/// Try to convert a hex string to MessageID
pub fn hex_to_message_id<T: ToString>(value: T) -> Result<MessageId> {
    let string = value.to_string();
    if string.len() != 64 {
        return Err(Error::InvalidParameter("string length".to_string()));
    }

    let mut bytes = [0u8; 32];
    hex::decode_to_slice(string, &mut bytes)?;

    Ok(MessageId::new(bytes))
}

/// Try to convert a hex string to TransactionID
pub fn hex_to_transaction_id<T: ToString>(value: T) -> Result<TransactionId> {
    let string = value.to_string();
    if string.len() != 64 {
        return Err(Error::InvalidParameter("string length".to_string()));
    }

    let mut bytes = [0u8; 32];
    hex::decode_to_slice(string, &mut bytes)?;

    Ok(TransactionId::new(bytes))
}

/// Try to convert a hex string to Address
pub fn hex_to_address<T: ToString>(value: T) -> Result<Address> {
    let string = value.to_string();
    if string.len() != 64 {
        return Err(Error::InvalidParameter("string length".to_string()));
    }

    let mut bytes = [0u8; 32];
    hex::decode_to_slice(string.as_bytes(), &mut bytes)?;

    Ok(Ed25519Address::new(bytes).into())
}

/// Response from the Iota node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T: ResponseType> {
    pub(crate) data: T,
}

impl<T: ResponseType> Response<T> {
    /// Get data of the response.
    pub fn data(&self) -> &T {
        &self.data
    }
}

/// Response of GET /api/v1/info endpoint
#[derive(Clone, Debug, Deserialize)]
pub struct NodeInfo {
    /// Iota node Name
    pub name: String,
    /// Iota node version
    pub version: String,
    /// Connection status
    #[serde(rename = "isHealthy")]
    pub is_healthy: bool,
    /// coordinator public key
    #[serde(rename = "coordinatorPublicKey")]
    pub coordinator_public_key: String,
    /// latest milestone message id
    #[serde(rename = "latestMilestoneMessageId")]
    pub latest_milestone_message_id: String,
    /// latest milestone index
    #[serde(rename = "latestMilestoneIndex")]
    pub latest_milestone_index: usize,
    /// latest milestone message id
    #[serde(rename = "solidMilestoneMessageId")]
    pub solid_milestone_message_id: String,
    /// solid milestone index
    #[serde(rename = "solidMilestoneIndex")]
    pub solid_milestone_index: usize,
    /// pruning index
    #[serde(rename = "pruningIndex")]
    pub pruning_index: usize,
    /// features
    pub features: Vec<String>,
}

impl ResponseType for NodeInfo {}

/// Response of GET /api/v1/tips endpoint
#[derive(Debug, Deserialize)]
pub(crate) struct Tips {
    /// Message ID of tip 1
    #[serde(rename = "tip1MessageId")]
    pub(crate) tip1: String,
    /// Message ID of tip 2
    #[serde(rename = "tip2MessageId")]
    pub(crate) tip2: String,
}

impl ResponseType for Tips {}

#[derive(Debug, Deserialize)]
pub(crate) struct PostMessageId {
    #[serde(rename = "messageId")]
    pub(crate) message_id: String,
}

impl ResponseType for PostMessageId {}

/// Collection of meesage ID
#[derive(Debug, Deserialize)]
pub(crate) struct MessageIds {
    #[serde(rename = "messageIds")]
    pub(crate) inner: Box<[String]>,
}

impl ResponseType for MessageIds {}

/// Response of GET /api/v1/messages/{messageId} endpoint
#[derive(Debug, Deserialize)]
pub struct MessageMetadata {
    /// Message ID
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Message ID of parent1
    #[serde(rename = "parent1MessageId")]
    pub parent1: String,
    /// Message ID of parent2
    #[serde(rename = "parent2MessageId")]
    pub parent2: String,
    /// Solid status
    #[serde(rename = "isSolid")]
    pub is_solid: bool,
}

impl ResponseType for MessageMetadata {}

#[derive(Debug, Deserialize)]
pub(crate) struct ChildrenMessageIds {
    #[serde(rename = "childrenMessageIds")]
    pub(crate) inner: Box<[String]>,
}

impl ResponseType for ChildrenMessageIds {}

#[derive(Debug, Deserialize)]
pub(crate) struct AddressBalance {
    pub(crate) count: usize,
    pub(crate) balance: u64,
}

impl ResponseType for AddressBalance {}

/// Output raw data
#[derive(Debug, Deserialize)]
pub(crate) struct RawOutput {
    #[serde(rename = "messageId")]
    pub(crate) message_id: String,
    #[serde(rename = "transactionId")]
    pub(crate) transaction_id: String,
    #[serde(rename = "outputIndex")]
    pub(crate) output_index: u16,
    #[serde(rename = "isSpent")]
    pub(crate) is_spent: bool,
    pub(crate) output: SLS,
}

impl ResponseType for RawOutput {}

#[derive(Debug, Deserialize)]
pub(crate) struct SLS {
    #[serde(rename = "type")]
    pub(crate) type_: u8,
    pub(crate) address: SLSAddress,
    pub(crate) amount: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SLSAddress {
    #[serde(rename = "type")]
    pub(crate) type_: u8,
    pub(crate) address: String,
}

/// Output data
#[derive(Debug)]
pub struct OutputMetadata {
    /// Message ID of the output
    pub message_id: Vec<u8>,
    /// Transaction ID of the output
    pub transaction_id: Vec<u8>,
    /// Output index.
    pub output_index: u16,
    /// Spend status of the output
    pub is_spent: bool,
    /// Corresponding address
    pub address: Address,
    /// Balance amount
    pub amount: u64,
}

/// Outputs that use a given address.
#[derive(Debug, Deserialize)]
pub struct AddressOutputs {
    /// Outputs used by the address.
    #[serde(rename = "outputIds")]
    pub output_ids: Box<[String]>,
}

impl ResponseType for AddressOutputs {}

/// Milestone from Iota node
#[derive(Debug, Deserialize)]
pub struct MilestoneMetadata {
    /// Milestone index
    #[serde(rename = "milestoneIndex")]
    pub milestone_index: u64,
    /// Milestone ID
    #[serde(rename = "messageId")]
    pub message_ids: String,
    /// Timestamp
    pub timestamp: u64,
}

impl ResponseType for MilestoneMetadata {}

/// Transfers structure
///
/// Users could use this to construct output address with amount of iota they want to get.
#[derive(Debug)]
pub struct Transfers(pub Vec<(Address, u64)>);

impl Transfers {
    /// Create Transfers starting with one address
    pub fn new(address: Address, amount: u64) -> Self {
        Self(vec![(address, amount)])
    }

    /// Add more address to the Transfers
    pub fn add(&mut self, address: Address, amount: u64) {
        self.0.push((address, amount));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MessageJson {
    version: u8,
    #[serde(rename = "parent1MessageId")]
    parent1: String,
    #[serde(rename = "parent2MessageId")]
    parent2: String,
    payload: PayloadJson,
    nonce: u64,
}

impl ResponseType for MessageJson {}

impl From<&Message> for MessageJson {
    fn from(i: &Message) -> Self {
        Self {
            version: 1,
            parent1: i.parent1().to_string(),
            parent2: i.parent2().to_string(),
            payload: i.payload().into(),
            nonce: i.nonce(),
        }
    }
}

impl TryFrom<MessageJson> for Message {
    type Error = crate::Error;

    fn try_from(value: MessageJson) -> Result<Self> {
        let mut parent1 = [0u8; 32];
        hex::decode_to_slice(value.parent1, &mut parent1)?;
        let mut parent2 = [0u8; 32];
        hex::decode_to_slice(value.parent2, &mut parent2)?;
        Ok(Message::builder()
            .with_parent1(MessageId::new(parent1))
            .with_parent2(MessageId::new(parent2))
            .with_payload(value.payload.try_into()?)
            .finish()?)
        // .nonce(value.nonce) TODO: Missing nounce method
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PayloadJson {
    #[serde(rename = "type")]
    type_: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    essence: Option<TransactionEssenceJson>,
    #[serde(rename = "unlockBlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unlock_blocks: Option<Box<[UnlockBlockJson]>>,
}

impl From<&Payload> for PayloadJson {
    fn from(i: &Payload) -> Self {
        match i {
            Payload::Transaction(i) => Self {
                type_: 0,
                index: None,
                data: None,
                essence: Some((i.essence()).into()),
                unlock_blocks: Some(i.unlock_blocks().iter().map(|input| input.into()).collect()),
            },
            Payload::Indexation(i) => Self {
                type_: 2,
                index: Some(i.index().to_string()),
                data: Some(hex::encode(i.data())),
                essence: None,
                unlock_blocks: None,
            },
            _ => todo!(),
        }
    }
}

impl TryFrom<PayloadJson> for Payload {
    type Error = crate::Error;

    fn try_from(value: PayloadJson) -> Result<Self> {
        match value.type_ {
            0 => {
                let mut transaction = Transaction::builder();
                transaction = transaction
                    .with_essence(value.essence.expect("Must have essence.").try_into()?);

                let unlock_blocks = value
                    .unlock_blocks
                    .expect("Must have unlcok blocks.")
                    .into_vec();
                for unlock_block in unlock_blocks {
                    transaction = transaction.add_unlock_block(unlock_block.try_into()?);
                }

                Ok(Payload::Transaction(Box::new(transaction.finish()?)))
            }
            2 => {
                let indexation = Indexation::new(
                    value.index.expect("Must have index."),
                    value
                        .data
                        .expect("Must have data.")
                        .as_bytes()
                        .to_vec()
                        .into_boxed_slice(),
                );
                Ok(Payload::Indexation(Box::new(indexation)))
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionEssenceJson {
    #[serde(rename = "type")]
    type_: u8,
    inputs: Box<[InputJson]>,
    outputs: Box<[OutputJson]>,
    payload: serde_json::Value,
}

impl From<&TransactionEssence> for TransactionEssenceJson {
    fn from(i: &TransactionEssence) -> Self {
        Self {
            type_: 0,
            inputs: i.inputs().iter().map(|input| input.into()).collect(),
            outputs: i.outputs().iter().map(|input| input.into()).collect(),
            payload: serde_json::Value::Null,
        }
    }
}

impl TryFrom<TransactionEssenceJson> for TransactionEssence {
    type Error = crate::Error;

    fn try_from(value: TransactionEssenceJson) -> Result<Self> {
        let mut builder = TransactionEssence::builder();

        let inputs: Vec<Input> = value
            .inputs
            .into_vec()
            .into_iter()
            .map(|input| input.try_into())
            .filter_map(|i| i.ok())
            .collect();
        for input in inputs {
            builder = builder.add_input(input);
        }

        let outputs: Vec<Output> = value
            .outputs
            .into_vec()
            .into_iter()
            .map(|output| output.try_into())
            .filter_map(|i| i.ok())
            .collect();
        for output in outputs {
            builder = builder.add_output(output);
        }

        Ok(builder.finish()?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct InputJson {
    #[serde(rename = "type")]
    type_: u8,
    #[serde(rename = "transactionId")]
    transaction_id: String,
    #[serde(rename = "transactionOutputIndex")]
    transaction_output_index: u16,
}

impl From<&Input> for InputJson {
    fn from(i: &Input) -> Self {
        match i {
            Input::UTXO(i) => Self {
                type_: 0,
                transaction_id: i.id().to_string(),
                transaction_output_index: i.index(),
            },
        }
    }
}

impl TryFrom<InputJson> for Input {
    type Error = crate::Error;

    fn try_from(value: InputJson) -> Result<Self> {
        let mut id = [0u8; 32];
        hex::decode_to_slice(value.transaction_id, &mut id)?;
        let input = UTXOInput::new(TransactionId::from(id), value.transaction_output_index)?;
        Ok(input.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OutputJson {
    #[serde(rename = "type")]
    type_: u8,
    address: AddressJson,
    amount: u64,
}

impl From<&Output> for OutputJson {
    fn from(i: &Output) -> Self {
        match i {
            Output::SignatureLockedSingle(s) => Self {
                type_: 0,
                address: s.address().into(),
                amount: s.amount().get(),
            },
        }
    }
}

impl TryFrom<OutputJson> for Output {
    type Error = crate::Error;

    fn try_from(value: OutputJson) -> Result<Self> {
        let output = SignatureLockedSingleOutput::new(
            value.address.try_into()?,
            value
                .amount
                .try_into()
                .expect("Output amount cannot be zero."),
        );
        Ok(output.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AddressJson {
    #[serde(rename = "type")]
    type_: u8,
    address: String,
}

impl From<&Address> for AddressJson {
    fn from(i: &Address) -> Self {
        match i {
            Address::Ed25519(a) => Self {
                type_: 1,
                address: a.to_string(),
            },
            _ => panic!("This library doesn't support WOTS."),
        }
    }
}

impl TryFrom<AddressJson> for Address {
    type Error = crate::Error;

    fn try_from(value: AddressJson) -> Result<Self> {
        match value.type_ {
            1 => {
                let mut address = [0u8; 32];
                hex::decode_to_slice(value.address, &mut address)?;
                let address = Ed25519Address::from(address);
                Ok(address.into())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UnlockBlockJson {
    #[serde(rename = "type")]
    type_: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<SignatureJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference: Option<u16>,
}

impl From<&UnlockBlock> for UnlockBlockJson {
    fn from(i: &UnlockBlock) -> Self {
        match i {
            UnlockBlock::Signature(s) => Self {
                type_: 0,
                signature: Some(s.into()),
                reference: None,
            },
            UnlockBlock::Reference(s) => Self {
                type_: 1,
                signature: None,
                reference: Some(s.index()),
            },
        }
    }
}

impl TryFrom<UnlockBlockJson> for UnlockBlock {
    type Error = crate::Error;

    fn try_from(value: UnlockBlockJson) -> Result<Self> {
        match value.type_ {
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

#[derive(Debug, Serialize, Deserialize)]
struct SignatureJson {
    #[serde(rename = "type")]
    type_: u8,
    #[serde(rename = "publicKey")]
    publickey: String,
    signature: String,
}

impl From<&SignatureUnlock> for SignatureJson {
    fn from(i: &SignatureUnlock) -> Self {
        match i {
            SignatureUnlock::Ed25519(a) => Self {
                type_: 1,
                publickey: hex::encode(a.public_key()),
                signature: hex::encode(a.signature()),
            },
            _ => panic!("This library doesn't support WOTS."),
        }
    }
}

impl TryFrom<SignatureJson> for SignatureUnlock {
    type Error = crate::Error;

    fn try_from(value: SignatureJson) -> Result<Self> {
        let mut public_key = [0u8; 32];
        hex::decode_to_slice(value.publickey, &mut public_key)?;
        let signature = hex::decode(value.signature)?.into_boxed_slice();
        Ok(Ed25519Signature::new(public_key, signature).into())
    }
}
