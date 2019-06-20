use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use failure;

use iota_client;
use iota_model;

mod deposit;
mod storage;

pub struct Settings {
    mwm: u64,
    depth: u64,
}

impl Settings {}

pub struct Account {
    id: String,
    running: AtomicBool,
    settings: Settings,
    last_key_index: u64,
}

impl Account {
    pub fn id(&self) -> &str {
        &self.id
    }

    fn start(&mut self) -> Result<(), failure::Error> {
        let state = storage::load_account(self.id())?;
        self.last_key_index = state.key_index();
        self.start_plugins()?;
        self.running.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn shutdown(&self) -> Result<(), failure::Error> {
        let running = self.running.load(Ordering::Relaxed);
        if !running {
            return Err(format_err!(""));
        }
        self.running.store(false, Ordering::Relaxed);
        self.shutdown_plugins()?;
        // TODO event machine
        Ok(())
    }

    fn send(
        &self,
        recipients: &[iota_model::Transfer],
    ) -> Result<iota_model::Bundle, failure::Error> {
        let running = self.running.load(Ordering::Relaxed);
        if !running {
            return Err(format_err!(""));
        }

        for target in recipients {
            if !iota_validation::is_trytes_with_length(
                target.address(),
                iota_constants::HASH_TRYTES_SIZE + iota_constants::ADDRESS_CHECKSUM_TRYTES_SIZE,
            ) {
                return Err(format_err!(""));
            }
        }

        let transfer_sum: i64 = recipients.iter().fold(0, |sum, val| sum + *val.value());
        let current_time = Instant::now();

        if transfer_sum > 0 {}

        unimplemented!()
    }

    fn new_deposit_address(
        &self,
        conditions: deposit::Conditions,
    ) -> Result<deposit::CDA, failure::Error> {
        unimplemented!()
    }

    fn available_balance(&self) -> Result<u64, failure::Error> {
        unimplemented!()
    }

    fn total_balance(&self) -> Result<u64, failure::Error> {
        unimplemented!()
    }

    fn is_new(&self) -> Result<bool, failure::Error> {
        unimplemented!()
    }

    fn update_settings(&self) -> Result<(), failure::Error> {
        unimplemented!()
    }

    fn start_plugins(&self) -> Result<(), failure::Error> {
        unimplemented!()
    }

    fn shutdown_plugins(&self) -> Result<(), failure::Error> {
        unimplemented!()
    }
}

pub fn new_account(settings: Option<Settings>) -> Result<Account, failure::Error> {
    unimplemented!()
}
