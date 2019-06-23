use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

/// Represents an address associated with a seed, that can be used as
/// an "input" when trying to meet a minimum threshold of funds for a
/// transaction
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub address: String,
    pub balance: i64,
    pub key_index: usize,
    pub security: usize,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
