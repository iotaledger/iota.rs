use std::fmt;
use serde_json;

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct Neighbor {
    address: String,
    number_of_all_transactions: i32,
    number_of_invalid_transactions: i32,
    number_of_new_transactions: i32,
    number_of_random_transactions: i32,
    number_of_sent_transactions: i32,
    connection_type: String,
}

impl fmt::Display for Neighbor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

impl Neighbor {
    fn new(
        address: String,
        number_of_all_transactions: i32,
        number_of_invalid_transactions: i32,
        number_of_new_transactions: i32,
        number_of_random_transactions: i32,
        number_of_sent_transactions: i32,
        connection_type: String,
    ) -> Neighbor {
        Neighbor {
            address,
            number_of_all_transactions,
            number_of_invalid_transactions,
            number_of_new_transactions,
            number_of_random_transactions,
            number_of_sent_transactions,
            connection_type,
        }
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn address_mut(&mut self) -> &mut String {
        &mut self.address
    }

    fn number_of_all_transactions(&self) -> i32 {
        self.number_of_all_transactions
    }

    fn number_of_all_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_all_transactions
    }

    fn number_of_invalid_transactions(&self) -> i32 {
        self.number_of_invalid_transactions
    }

    fn number_of_invalid_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_invalid_transactions
    }

    fn number_of_new_transactions(&self) -> i32 {
        self.number_of_new_transactions
    }

    fn number_of_new_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_new_transactions
    }

    fn number_of_random_transactions(&self) -> i32 {
        self.number_of_random_transactions
    }

    fn number_of_random_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_random_transactions
    }

    fn number_of_sent_transactions(&self) -> i32 {
        self.number_of_sent_transactions
    }

    fn number_of_sent_transactions_mut(&mut self) -> &mut i32 {
        &mut self.number_of_sent_transactions
    }

    fn connection_type(&self) -> &str {
        &self.connection_type
    }

    fn connection_type_mut(&mut self) -> &mut String {
        &mut self.connection_type
    }
}
