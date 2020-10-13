//! Types of several IOTA APIs related objects
use crate::{Result, Error};

use bee_message::prelude::{Address, Message, MessageId};

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
    /// Payload detail
    pub payload: Option<serde_json::Value>,
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
