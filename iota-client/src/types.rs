//! Types of several IOTA APIs related objects

use bee_transaction::prelude::{Address, TransactionId};

/// Marker trait for response
pub trait ResponseType {}

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

/// Hex string of message ID
#[derive(Debug, Deserialize)]
pub struct MessageIdHex(pub(crate) String);

/// Hex string of output ID
#[derive(Debug, Deserialize)]
pub struct OutputIdHex(pub String);

/// Response of GET /api/v1/tips endpoint
#[derive(Debug, Deserialize)]
pub(crate) struct Tips {
    /// Message ID of tip 1
    #[serde(rename = "tip1MessageId")]
    pub(crate) tip1: MessageIdHex,
    /// Message ID of tip 2
    #[serde(rename = "tip2MessageId")]
    pub(crate) tip2: MessageIdHex,
}

impl ResponseType for Tips {}

/// Collection of meesage ID
#[derive(Debug, Deserialize)]
pub(crate) struct MessageIds {
    #[serde(rename = "messageIds")]
    pub(crate) inner: Box<[MessageIdHex]>,
}

impl ResponseType for MessageIds {}

/// Response of GET /api/v1/messages/{messageId} endpoint
#[derive(Debug, Deserialize)]
pub struct MessageMetadata {
    /// Message ID
    #[serde(rename = "messageId")]
    pub message_id: MessageIdHex,
    /// Message ID of parent1
    #[serde(rename = "parent1MessageId")]
    pub parent1: MessageIdHex,
    /// Message ID of parent2
    #[serde(rename = "parent2MessageId")]
    pub parent2: MessageIdHex,
    /// Solid status
    #[serde(rename = "isSolid")]
    pub is_solid: bool,
    /// Promote status
    #[serde(rename = "shouldPromote")]
    pub should_prmote: bool,
    /// Reattach status
    #[serde(rename = "shouldReattach")]
    pub should_reattach: bool,
}

impl ResponseType for MessageMetadata {}

#[derive(Debug, Deserialize)]
pub(crate) struct ChildrenMessageIds {
    #[serde(rename = "childrenMessageIds")]
    pub(crate) inner: Box<[MessageIdHex]>,
}

impl ResponseType for ChildrenMessageIds {}

#[derive(Debug, Deserialize)]
pub(crate) struct AddressBalance {
    pub(crate) balance: u64,
}

impl ResponseType for AddressBalance {}

/// Output data
#[derive(Debug, Deserialize)]
pub struct Output {
    /// Producer message of the output
    pub producer: TransactionId,
    /// Corresponding address
    pub address: Address,
    /// Balance amount
    pub amount: u64,
    /// Spend status of the output
    pub spent: bool,
    /// Output index.
    pub output_index: u16,
}

/// Outputs that use a given address.
#[derive(Debug, Deserialize)]
pub struct AddressOutputs {
    /// Outputs used by the address.
    pub output_ids: Box<[OutputIdHex]>,
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
    pub message_ids: MessageIdHex,
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
