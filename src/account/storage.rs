use std::collections::HashMap;
use std::time::Instant;

use iota_constants::SecurityLevel;

use crate::account::deposit;

pub struct PendingTransfer {
    bundle: Vec<String>,
    tails: Vec<String>,
}

pub struct AccountState {
    key_index: u64,
    deposit_addresses: HashMap<u64, String>,
    pending_transfers: HashMap<String, PendingTransfer>,
}

impl AccountState {
    pub fn key_index(&self) -> u64 {
        self.key_index
    }
}

pub struct ExportedAccountState {
    id: String,
    date: Instant,
    state: AccountState,
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

pub fn load_account(id: &str) -> Result<AccountState, failure::Error> {
    unimplemented!()
}

pub fn remove_account(id: &str) -> Result<(), failure::Error> {
    unimplemented!()
}

pub fn import_account(id: ExportedAccountState) -> Result<(), failure::Error> {
    unimplemented!()
}

pub fn export_account(id: &str) -> Result<ExportedAccountState, failure::Error> {
    unimplemented!()
}

pub fn read_index(id: &str) -> Result<u64, failure::Error> {
    unimplemented!()
}

pub fn write_index(id: &str, index: usize) -> Result<(), failure::Error> {
    unimplemented!()
}

pub fn add_deposit_address(
    id: &str,
    index: usize,
    deposit_address: StoredDepositAddress,
) -> Result<(), failure::Error> {
    unimplemented!()
}

pub fn remove_deposit_address(id: &str, index: usize) -> Result<(), failure::Error> {
    unimplemented!()
}

pub fn get_deposit_addresses(
    id: &str,
) -> Result<HashMap<u64, StoredDepositAddress>, failure::Error> {
    unimplemented!()
}

pub fn add_pending_transfer(
    id: &str,
    origin_tail_tx_hash: &str,
    bundle_trytes: Vec<String>,
    indicies: Option<Vec<usize>>,
) -> Result<(), failure::Error> {
    unimplemented!()
}

pub fn remove_pending_transfer(id: &str, origin_tail_tx_hash: &str) -> Result<(), failure::Error> {
    unimplemented!()
}

pub fn add_tail_hash(
    id: &str,
    origin_tail_tx_hash: &str,
    new_tail_tx_hash: &str,
) -> Result<(), failure::Error> {
    unimplemented!()
}

pub fn get_pending_transfers(id: &str) -> Result<HashMap<String, PendingTransfer>, failure::Error> {
    unimplemented!()
}
