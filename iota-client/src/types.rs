//! Types of several IOTA APIs related objects

use bee_transaction::prelude::{Address, MessageId};

/// Marker trait for response
pub trait ResponseType {}

/// Response from the Iota node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T: ResponseType>{
    data: T
}

impl<T: ResponseType> Response<T> {
    /// Get data of the response.
    pub fn data(&self) -> &T {
        &self.data
    }
}

/// Response of GET /info endpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// Output data
#[derive(Debug)]
pub struct Output {
    /// Producer message of the output
    pub producer: MessageId,
    /// Corresponding address
    pub address: Address,
    /// Balance amount
    pub amount: u64,
    /// Spend status of the output
    pub spent: bool,
    /// Output index.
    pub output_index: u8,
}

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

/// Transaction identifier
pub type TransactionId = MessageId;