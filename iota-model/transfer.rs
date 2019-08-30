use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

/// Represents a transfer in IOTA
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    /// Unix epoch: Seconds since Jan 1, 1970
    pub timestamp: String,
    /// Contains either the sender or recipient's address
    pub address: String,
    /// Transaction hash
    pub hash: String,
    /// Persistence
    pub persistence: bool,
    /// Amount of IOTA tokens to deposit to or withdraw from the address
    pub value: i64,
    /// Message of the transfer
    pub message: String,
    /// User-defined tag
    pub tag: String,
    /// User-defined tag (soon to be removed)
    pub obsolete_tag: String,
}

impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl Into<Vec<Transfer>> for Transfer {
    fn into(self) -> Vec<Transfer> {
        vec![self]
    }
}

impl Into<Vec<Transfer>> for &Transfer {
    fn into(self) -> Vec<Transfer> {
        vec![self.clone()]
    }
}
