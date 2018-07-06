use serde_json;
use std::fmt;

/// Represents an address associated with a seed, that can be used as
/// an "input" when trying to meet a minimum threshold of funds for a
/// transaction
#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    address: String,
    balance: i64,
    key_index: usize,
    security: usize,
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

impl Input {
    /// Creates a new input
    pub fn new(address: String, balance: i64, key_index: usize, security: usize) -> Input {
        Input {
            address,
            balance,
            key_index,
            security,
        }
    }

    /// Provides a view of the inputs address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Provides a mutable view of the inputs address
    pub fn address_mut(&mut self) -> &mut str {
        &mut self.address
    }

    /// Provides a view of the inputs balance
    pub fn balance(&self) -> i64 {
        self.balance
    }

    /// Provides a mutable view of the inputs balance
    pub fn balance_mut(&mut self) -> &mut i64 {
        &mut self.balance
    }

    /// Provides a view of the inputs key_index
    pub fn key_index(&self) -> usize {
        self.key_index
    }

    /// Provides a mutable view of the inputs key_index
    pub fn key_index_mut(&mut self) -> &mut usize {
        &mut self.key_index
    }

    /// Provides a view of the inputs security
    pub fn security(&self) -> usize {
        self.security
    }

    /// Provides a mutable view of the inputs security
    pub fn security_mut(&mut self) -> &mut usize {
        &mut self.security
    }
}
