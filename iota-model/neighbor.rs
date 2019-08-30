use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

/// This represents a neighbor node
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Neighbor {
    /// IP address of neighbors
    pub address: String,
    /// Number of all transactions
    #[serde(rename = "numberOfAllTransactions")]
    pub number_of_all_transactions: i32,
    /// Number of invalid transactions
    #[serde(rename = "numberOfInvalidTransactions")]
    pub number_of_invalid_transactions: i32,
    /// Number of new transactions
    #[serde(rename = "numberOfNewTransactions")]
    pub number_of_new_transactions: i32,
    /// Number of random transaction requests
    #[serde(rename = "numberOfRandomTransactionRequests")]
    pub number_of_random_transactions: i32,
    /// Number of sent transactions
    #[serde(rename = "numberOfSentTransactions")]
    pub number_of_sent_transactions: i32,
}

impl fmt::Display for Neighbor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
