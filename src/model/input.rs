use std::fmt;

#[derive(Default, Serialize, Deserialize)]
pub struct Input {
    address: String,
    balance: i64,
    key_index: i32,
    security: i32,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.address)
    }
}

impl Input {
    pub fn new(address: String, balance: i64, key_index: i32, security: i32) -> Input {
        Input {
            address,
            balance,
            key_index,
            security,
        }
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn get_balance(&self) -> i64 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: i64) {
        self.balance = balance;
    }

    pub fn get_key_index(&self) -> i32 {
        self.key_index
    }

    pub fn set_key_index(&mut self, key_index: i32) {
        self.key_index = key_index;
    }

    pub fn get_security(&self) -> i32 {
        self.security
    }

    pub fn set_security(&mut self, security: i32) {
        self.security = security;
    }
}
