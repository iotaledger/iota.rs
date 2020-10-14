//! Types of several IOTA APIs related objects
use crate::{Error, Result};

use bee_message::prelude::{
    Address, Indexation, Input, Message, MessageId, Output, Payload, SignatureUnlock, Transaction,
    TransactionEssence, UnlockBlock,
};

use std::convert::{From, TryFrom};

/// Marker trait for response
pub trait ResponseType {}

impl ResponseType for Message {}

/// Try to convert a hex string to MessageID
pub fn hex_to_message_id<T: ToString>(value: T) -> Result<MessageId> {
    let string = value.to_string();
    if string.len() != 64 {
        return Err(Error::InvalidParameter("string length".to_string()));
    }

    for c in string.chars() {
        match c {
            '0'..='9' | 'a'..='z' => (),
            _ => return Err(Error::InvalidParameter("hex character".to_string())),
        }
    }

    let mut bytes = [0u8; 32];
    hex::decode_to_slice(string.as_bytes(), &mut bytes)?;

    Ok(MessageId::new(bytes))
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
    pub(crate) amount: u64,
}

impl ResponseType for RawOutput {}

#[derive(Debug, Deserialize)]
pub(crate) struct SLS {
    pub(crate) type_: u8,
    pub(crate) address: EdAddress,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EdAddress {
    pub(crate) type_: u8,
    pub(crate) ed25519: String,
}

/// Output data
#[derive(Debug)]
pub struct OutputContext {
    /// Message ID of the output
    pub message_id: String,
    /// Transaction ID of the output
    pub transaction_id: String,
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
    pub output_ids: Box<[String]>,
}

impl ResponseType for AddressOutputs {}

/// Milestone from Iota node
#[derive(Debug, Deserialize)]
pub struct Milestone {
    /// Milestone index
    #[serde(rename = "milestoneIndex")]
    pub milestone_index: u64,
    /// Milestone ID
    #[serde(rename = "messageId")]
    pub message_ids: String,
    /// Timestamp
    pub timestamp: u64,
}

impl ResponseType for Milestone {}

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

#[derive(Debug, Serialize)]
pub struct MessageJson {
    version: u8,
    #[serde(rename = "parent1MessageId")]
    parent1: String,
    #[serde(rename = "parent2MessageId")]
    parent2: String,
    payload: PayloadJson,
    nonce: u64,
}

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

#[derive(Debug, Serialize)]
struct PayloadJson {
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
                essence: Some((&i.essence).into()),
                unlock_blocks: Some(i.unlock_blocks.iter().map(|input| input.into()).collect()),
            },
            Payload::Indexation(_i) => Self {
                type_: 2,
                index: Some(String::from("TEST")),
                data: Some(String::from("TESTING")),
                essence: None,
                unlock_blocks: None,
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize)]
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
            inputs: i.inputs().into_iter().map(|input| input.into()).collect(),
            outputs: i.outputs().into_iter().map(|input| input.into()).collect(),
            payload: serde_json::Value::Null,
        }
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
struct SignatureJson {
    #[serde(rename = "type")]
    type_: u8,
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
