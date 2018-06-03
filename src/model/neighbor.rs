struct Neighbor {
    address: String,
    number_of_all_transactions: i32,
    number_of_invalid_transactions: i32,
    number_of_new_transactions: i32,
    number_of_random_transactions: i32,
    number_of_sent_transactions: i32,
    connection_type: String,
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

    fn get_address(&self) -> &str {
        &self.address
    }

    fn get_number_of_all_transactions(&self) -> i32 {
        self.number_of_all_transactions
    }

    fn get_number_of_invalid_transactions(&self) -> i32 {
        self.number_of_invalid_transactions
    }

    fn get_number_of_new_transactions(&self) -> i32 {
        self.number_of_new_transactions
    }

    fn get_number_of_random_transactions(&self) -> i32 {
        self.number_of_random_transactions
    }

    fn get_number_of_sent_transactions(&self) -> i32 {
        self.number_of_sent_transactions
    }

    fn get_connection_type(&self) -> &str {
        &self.connection_type
    }
}
