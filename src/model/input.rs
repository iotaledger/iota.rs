use serde_json;
use std::fmt;

/// Represents an address associated with a seed, that can be used as
/// an "input" when trying to meet a minimum threshold of funds for a
/// transaction
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    pub fn address_mut(&mut self) -> &mut String {
        &mut self.address
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_address<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.address = new_value.into();
    }

    /// Provides a view of the inputs balance
    pub fn balance(&self) -> i64 {
        self.balance
    }

    /// Provides a mutable view of the inputs balance
    pub fn balance_mut(&mut self) -> &mut i64 {
        &mut self.balance
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_balance<T>(&mut self, new_value: T)
    where
        T: Into<i64>,
    {
        self.balance = new_value.into();
    }

    /// Provides a view of the inputs key_index
    pub fn key_index(&self) -> usize {
        self.key_index
    }

    /// Provides a mutable view of the inputs key_index
    pub fn key_index_mut(&mut self) -> &mut usize {
        &mut self.key_index
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_key_index<T>(&mut self, new_value: T)
    where
        T: Into<usize>,
    {
        self.key_index = new_value.into();
    }

    /// Provides a view of the inputs security
    pub fn security(&self) -> usize {
        self.security
    }

    /// Provides a mutable view of the inputs security
    pub fn security_mut(&mut self) -> &mut usize {
        &mut self.security
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_security<T>(&mut self, new_value: T)
    where
        T: Into<usize>,
    {
        self.security = new_value.into();
    }
}
