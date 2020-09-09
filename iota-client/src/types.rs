//! Types of several IOTA APIs related objects

use bee_transaction::atomic::{
    payload::signed_transaction::Address,
    Hash
};
/// Response of GET /info endpoint
pub struct NodeInfo {
    /// Iota node Name
    pub name: String,
    /// Iota node version
    pub version: String,
    /// Connection status
    pub is_healthy: bool,
    /// latest solid subtangle milestone index
    pub latest_solid_subtangle_milestone_index: usize,
}

/// Output data
pub struct Output {
    /// Producer message of the output
    pub producer: Hash,
    /// Corresponding address
    pub address: Address,
    /// Balance amount
    pub amount: u64,
    /// Spend status of the output
    pub spent: bool,
}
