use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

/// This represents a neighbor node
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Neighbor {
    address: String,
    #[serde(rename = "numberOfAllTransactions")]
    number_of_all_transactions: i32,
    #[serde(rename = "numberOfInvalidTransactions")]
    number_of_invalid_transactions: i32,
    #[serde(rename = "numberOfNewTransactions")]
    number_of_new_transactions: i32,
    #[serde(rename = "numberOfRandomTransactionRequests")]
    number_of_random_transactions: i32,
    #[serde(rename = "numberOfSentTransactions")]
    number_of_sent_transactions: i32,
}

impl fmt::Display for Neighbor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl Neighbor {
    /// Creates a new instance of neighbor
    pub fn new<S>(
        address: S,
        number_of_all_transactions: i32,
        number_of_invalid_transactions: i32,
        number_of_new_transactions: i32,
        number_of_random_transactions: i32,
        number_of_sent_transactions: i32,
    ) -> Neighbor
    where
        S: Into<String>,
    {
        Neighbor {
            address: address.into(),
            number_of_all_transactions,
            number_of_invalid_transactions,
            number_of_new_transactions,
            number_of_random_transactions,
            number_of_sent_transactions,
        }
    }

    /// Provides a view of the address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Provides a mutable view of the address
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

    /// Provides a view of the number_of_all_transactions
    pub fn number_of_all_transactions(&self) -> i32 {
        self.number_of_all_transactions
    }

    /// Provides a mutable view of the number_of_all_transactions
    pub fn number_of_all_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_all_transactions
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_number_of_all_transactions<T>(&mut self, new_value: T)
    where
        T: Into<i32>,
    {
        self.number_of_all_transactions = new_value.into();
    }

    /// Provides a view of the number_of_invalid_transactions
    pub fn number_of_invalid_transactions(&self) -> i32 {
        self.number_of_invalid_transactions
    }

    /// Provides a mutable view of the number_of_invalid_transactions
    pub fn number_of_invalid_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_invalid_transactions
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_number_of_invalid_transactions<T>(&mut self, new_value: T)
    where
        T: Into<i32>,
    {
        self.number_of_invalid_transactions = new_value.into();
    }

    /// Provides a view of the number_of_new_transactions
    pub fn number_of_new_transactions(&self) -> i32 {
        self.number_of_new_transactions
    }

    /// Provides a mutable view of the number_of_new_transactions
    pub fn number_of_new_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_new_transactions
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_number_of_new_transactions<T>(&mut self, new_value: T)
    where
        T: Into<i32>,
    {
        self.number_of_new_transactions = new_value.into();
    }

    /// Provides a view of the number_of_random_transactions
    pub fn number_of_random_transactions(&self) -> i32 {
        self.number_of_random_transactions
    }

    /// Provides a mutable view of the number_of_random_transactions
    pub fn number_of_random_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_random_transactions
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_number_of_random_transactions<T>(&mut self, new_value: T)
    where
        T: Into<i32>,
    {
        self.number_of_random_transactions = new_value.into();
    }

    /// Provides a view of the number_of_sent_transactions
    pub fn number_of_sent_transactions(&self) -> i32 {
        self.number_of_sent_transactions
    }

    /// Provides a mutable view of the number_of_sent_transactions
    pub fn number_of_sent_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_sent_transactions
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_number_of_sent_transactions<T>(&mut self, new_value: T)
    where
        T: Into<i32>,
    {
        self.number_of_sent_transactions = new_value.into();
    }
}
