use super::structs::*;
use crate::Result;
use iota_model::Bundle;
use std::collections::HashMap;

pub trait Account: Clone + Default {
    fn id(&self) -> String;
    fn load(&self) -> Result<()>;
    fn start(&self) -> Result<()>;
    fn shutdown(&self) -> Result<()>;
    fn send(&self) -> Result<Bundle>;
    fn new_deposit_address(&self) -> Result<ConditionalDepositAddress>;
    fn usable_balance(&self) -> Result<u64>;
    fn total_balance(&self) -> Result<u64>;
    fn is_new(&self) -> bool;
    fn update_settings(&self) -> Result<()>;
}

pub trait AccountStore {
    fn load_account(id: &str) -> Result<AccountState>;
    fn save_account(id: &str, state: AccountState) -> Result<()>;
    fn remove_account(id: &str) -> Result<()>;
    fn read_index(id: &str) -> Result<usize>;
    fn write_index(id: &str, index: usize) -> Result<()>;
    fn add_deposit_address(
        id: &str,
        index: usize,
        deposit_address: StoredDepositAddress,
    ) -> Result<()>;
    fn remove_deposit_address(id: &str, index: usize) -> Result<()>;
    fn get_deposit_addresses(id: &str) -> Result<HashMap<usize, StoredDepositAddress>>;
    fn add_pending_transfer(
        id: &str,
        origin_tail_tx_hash: &str,
        bundle_trytes: Vec<String>,
        indicies: Option<Vec<usize>>,
    ) -> Result<()>;
    fn remove_pending_transfer(id: &str, origin_tail_tx_hash: &str) -> Result<()>;
    fn add_tail_hash(id: &str, origin_tail_tx_hash: &str, new_tail_tx_hash: &str) -> Result<()>;
    fn get_pending_transfers(id: &str) -> Result<HashMap<String, PendingTransfer>>;
    fn import_account(id: ExportedAccountState) -> Result<()>;
    fn export_account(id: &str) -> Result<ExportedAccountState>;
}

pub trait Plugin<T: Account> {
    fn account(&self) -> &T;
    fn name(&self) -> &str;
}