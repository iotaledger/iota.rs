use bee_transaction::atomic::{
    Hash, Message, 
};

/// Response of GET /info endpoint
pub struct GetInfoResponse {
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
    /// Producer of the output
    producer: Hash,
    /// Corresponding address
    address: Hash,
    /// Balance amount
    amount: u64,
    /// Spend status of the output
    spent: bool,
}