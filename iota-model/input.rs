use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

/// Represents an address associated with a seed, that can be used as
/// an "input" when trying to meet a minimum threshold of funds for a
/// transaction
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Input {
    /// Transaction address
    pub address: String,
    /// Address balance
    pub balance: i64,
    /// Key index of a seed
    pub key_index: usize,
    /// Security level
    pub security: usize,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
