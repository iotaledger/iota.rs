use super::structs::*;
use super::traits::AccountStore;
use crate::Result;
use iota_conversion::Trinary;
use std::collections::HashMap;
use std::time::Instant;

pub struct FileAccountStore;

impl AccountStore for FileAccountStore {
    fn load_account(id: &str) -> Result<AccountState> {
        unimplemented!();
    }

    fn save_account(id: &str, state: AccountState) -> Result<()> {
        unimplemented!();
    }

    fn remove_account(id: &str) -> Result<()> {
        unimplemented!();
    }

    fn read_index(id: &str) -> Result<usize> {
        Ok(Self::load_account(&id)?.key_index)
    }

    fn write_index(id: &str, index: usize) -> Result<()> {
        let mut state = Self::load_account(&id)?;
        state.key_index = index;
        Self::save_account(&id, state)
    }

    fn add_deposit_address(
        id: &str,
        index: usize,
        deposit_address: StoredDepositAddress,
    ) -> Result<()> {
        let mut state = Self::load_account(&id)?;
        state.deposit_addresses.insert(index, deposit_address);
        Self::save_account(&id, state)
    }

    fn remove_deposit_address(id: &str, index: usize) -> Result<()> {
        let mut state = Self::load_account(&id)?;
        state
            .deposit_addresses
            .remove(&index)
            .ok_or_else(|| format_err!("Given index was not valid"))?;
        Self::save_account(&id, state)
    }

    fn get_deposit_addresses(id: &str) -> Result<HashMap<usize, StoredDepositAddress>> {
        Ok(Self::load_account(&id)?.deposit_addresses)
    }

    fn add_pending_transfer(
        id: &str,
        tail_hash: &str,
        bundle_trytes: Vec<String>,
        indices: Option<Vec<usize>>,
    ) -> Result<()> {
        let mut pending_transfer = PendingTransfer::default();
        pending_transfer.bundle_trits = bundle_trytes.iter().map(|trytes| trytes.trits()).collect();

        let mut state = Self::load_account(&id)?;
        state
            .pending_transfers
            .insert(tail_hash.into(), pending_transfer);
        Self::save_account(&id, state)
    }

    fn remove_pending_transfer(id: &str, tail_hash: &str) -> Result<()> {
        let mut state = Self::load_account(&id)?;
        state
            .pending_transfers
            .remove(tail_hash)
            .ok_or_else(|| format_err!("Given tail hash was not pending"))?;
        Self::save_account(&id, state)
    }

    fn add_tail_hash(id: &str, tail_hash: &str, new_tail: &str) -> Result<()> {
        let mut state = Self::load_account(&id)?;
        state
            .pending_transfers
            .get_mut(tail_hash)
            .ok_or_else(|| format_err!("No pending transfers with the given tail hash"))?
            .tail_hashes
            .push(new_tail.into());
        Self::save_account(&id, state)
    }

    fn get_pending_transfers(id: &str) -> Result<HashMap<String, PendingTransfer>> {
        Ok(Self::load_account(&id)?.pending_transfers)
    }

    fn import_account(state: ExportedAccountState) -> Result<()> {
        Self::save_account(&state.id, state.state)
    }

    fn export_account(id: &str) -> Result<ExportedAccountState> {
        let state = Self::load_account(&id)?;
        Ok(ExportedAccountState {
            id: id.to_string(),
            state,
            date: Instant::now(),
        })
    }
}
