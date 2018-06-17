use std::fmt;
use serde_json;
use failure::{Compat, Error};

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    address: String,
    balance: u64,
    key_index: usize,
    security: usize,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

impl Input {
    pub fn new(address: String, balance: u64, key_index: usize, security: usize) -> Input {
        Input {
            address,
            balance,
            key_index,
            security,
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn address_mut(&mut self) -> &mut str {
        &mut self.address
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn balance_mut(&mut self) -> &mut u64 {
        &mut self.balance
    }

    pub fn key_index(&self) -> usize {
        self.key_index
    }

    pub fn key_index_mut(&mut self) -> &mut usize {
        &mut self.key_index
    }

    pub fn security(&self) -> usize {
        self.security
    }

    pub fn security_mut(&mut self) -> &mut usize {
        &mut self.security
    }
}
