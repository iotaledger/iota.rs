use std::collections::HashMap;
use std::time::Instant;

use iota_constants::SecurityLevel;

use crate::account::deposit;

#[derive(Clone, Debug, Default)]
pub struct DepositRequest {
    pub multi_use: bool,
    pub expected_amount: u64,
    pub timeout: Option<Instant>,
}

#[derive(Clone, Debug, Default)]
pub struct ConditionalDepositAddress {
    pub deposit_address: String,
    pub deposit_request: DepositRequest,
}

#[derive(Clone, Debug, Default)]
pub struct PendingTransfer {
    pub bundle_trits: Vec<Vec<i8>>,
    pub tail_hashes: Vec<String>,
}

pub struct AccountState {
    pub key_index: usize,
    pub deposit_addresses: HashMap<usize, StoredDepositAddress>,
    pub pending_transfers: HashMap<String, PendingTransfer>,
}

impl AccountState {
    pub fn is_new(&self) -> bool {
        self.deposit_addresses.is_empty() && self.pending_transfers.is_empty()
    }
}

pub struct ExportedAccountState {
    pub id: String,
    pub date: Instant,
    pub state: AccountState,
}

pub struct StoredDepositAddress {
    conditions: deposit::Conditions,
    security_level: SecurityLevel,
}

#[derive(Debug, Fail)]
enum AccountError {
    #[fail(display = "account not found: {}", id)]
    NotFound { id: String },
}
