// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Multi-threaded PoW miner

use iota_pow::providers::{
    miner::{MinerBuilder, MinerCancel},
    NonceProvider, NonceProviderBuilder,
};

/// The miner builder.
#[must_use]
pub struct ClientMinerBuilder {
    local_pow: bool,
    cancel: MinerCancel,
    worker_count: usize,
}

impl ClientMinerBuilder {
    /// Sets the local PoW config
    pub fn with_local_pow(mut self, value: bool) -> Self {
        self.local_pow = value;
        self
    }
    /// Set cancel miner
    pub fn with_cancel(mut self, cancel: MinerCancel) -> Self {
        self.cancel = cancel;
        self
    }
    /// Set the amount of workers for PoW
    pub fn with_worker_count(mut self, worker_count: usize) -> Self {
        self.worker_count = worker_count;
        self
    }
}

impl Default for ClientMinerBuilder {
    fn default() -> Self {
        ClientMinerBuilder::new()
    }
}

impl NonceProviderBuilder for ClientMinerBuilder {
    type Provider = ClientMiner;

    fn new() -> Self {
        Self {
            worker_count: num_cpus::get(),
            local_pow: true,
            cancel: MinerCancel::default(),
        }
    }

    fn finish(self) -> ClientMiner {
        ClientMiner {
            local_pow: self.local_pow,
            cancel: self.cancel,
            worker_count: self.worker_count,
        }
    }
}

/// The miner used for PoW
pub struct ClientMiner {
    local_pow: bool,
    cancel: MinerCancel,
    worker_count: usize,
}

impl NonceProvider for ClientMiner {
    type Builder = ClientMinerBuilder;
    type Error = crate::Error;

    fn nonce(&self, bytes: &[u8], target_score: u32) -> std::result::Result<u64, Self::Error> {
        if self.local_pow {
            MinerBuilder::new()
                .with_num_workers(self.worker_count)
                .with_cancel(self.cancel.clone())
                .finish()
                .nonce(bytes, target_score)
                .map_err(|e| crate::Error::Pow(e.to_string()))
        } else {
            Ok(0)
        }
    }
}
