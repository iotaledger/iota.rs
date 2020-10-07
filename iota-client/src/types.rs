//! Types of several IOTA APIs related objects

use bee_transaction::prelude::{Address, Hash};
/// Response of GET /info endpoint
#[derive(Clone, Debug)]
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
#[derive(Debug)]
pub struct Output {
    /// Producer message of the output
    pub producer: Hash,
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
