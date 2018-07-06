use serde_json;
use std::fmt;

/// This represents a neighbor node
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
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

impl Default for Neighbor {
    fn default() -> Neighbor {
        Neighbor {
            address: "127.0.0.1:8080".to_string(),
            number_of_all_transactions: 0,
            number_of_invalid_transactions: 0,
            number_of_new_transactions: 0,
            number_of_random_transactions: 0,
            number_of_sent_transactions: 0,
        }
    }
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
    pub fn new(
        address: &str,
        number_of_all_transactions: i32,
        number_of_invalid_transactions: i32,
        number_of_new_transactions: i32,
        number_of_random_transactions: i32,
        number_of_sent_transactions: i32,
    ) -> Neighbor {
        Neighbor {
            address: address.to_string(),
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

    /// Provides a view of the number_of_all_transactions
    pub fn number_of_all_transactions(&self) -> i32 {
        self.number_of_all_transactions
    }

    /// Provides a mutable view of the number_of_all_transactions
    pub fn number_of_all_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_all_transactions
    }

    /// Provides a view of the number_of_invalid_transactions
    pub fn number_of_invalid_transactions(&self) -> i32 {
        self.number_of_invalid_transactions
    }

    /// Provides a mutable view of the number_of_invalid_transactions
    pub fn number_of_invalid_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_invalid_transactions
    }

    /// Provides a view of the number_of_new_transactions
    pub fn number_of_new_transactions(&self) -> i32 {
        self.number_of_new_transactions
    }

    /// Provides a mutable view of the number_of_new_transactions
    pub fn number_of_new_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_new_transactions
    }

    /// Provides a view of the number_of_random_transactions
    pub fn number_of_random_transactions(&self) -> i32 {
        self.number_of_random_transactions
    }

    /// Provides a mutable view of the number_of_random_transactions
    pub fn number_of_random_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_random_transactions
    }

    /// Provides a view of the number_of_sent_transactions
    pub fn number_of_sent_transactions(&self) -> i32 {
        self.number_of_sent_transactions
    }

    /// Provides a mutable view of the number_of_sent_transactions
    pub fn number_of_sent_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_sent_transactions
    }
}
