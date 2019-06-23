use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

/// Represents a transfer in IOTA
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    pub timestamp: String,
    pub address: String,
    pub hash: String,
    pub persistence: bool,
    pub value: i64,
    pub message: String,
    pub tag: String,
    pub obsolete_tag: String,
}

impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
