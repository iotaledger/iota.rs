// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Multi-threaded PoW miner.

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use crypto::{
    encoding::ternary::{b1t6, T1B1Buf, TritBuf},
    hashes::{
        blake2b::Blake2b256,
        ternary::{
            curl_p::{CurlPBatchHasher, BATCH_SIZE},
            HASH_LENGTH,
        },
        Digest,
    },
};

use crate::{score::count_trailing_zeros, LN_3};

const DEFAULT_NUM_WORKERS: usize = 1;

/// A type to cancel a [`Miner`] to abort operations.
#[derive(Default, Clone)]
pub struct MinerCancel(Arc<AtomicBool>);

impl MinerCancel {
    /// Creates a new [`MinerCancel`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Cancels the [`Miner`].
    pub fn trigger(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    /// Checks if cancellation has been triggered.
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }

    /// Resets the cancel flag.
    fn reset(&self) {
        self.0.store(false, Ordering::Relaxed);
    }
}

/// Builder for a [`Miner`].
#[derive(Default)]
#[must_use]
pub struct MinerBuilder {
    num_workers: Option<usize>,
    cancel: Option<MinerCancel>,
}

impl MinerBuilder {
    /// Creates a new [`MinerBuilder`].
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    /// Sets the desired number of workers for the [`Miner`].
    pub fn with_num_workers(mut self, num_workers: usize) -> Self {
        self.num_workers.replace(num_workers);
        self
    }

    /// Sets a `MinerCancel to abort the [`Miner`].
    pub fn with_cancel(mut self, cancel: MinerCancel) -> Self {
        self.cancel.replace(cancel);
        self
    }

    /// Builds the [`Miner`].
    pub fn finish(self) -> Miner {
        Miner {
            num_workers: self.num_workers.unwrap_or(DEFAULT_NUM_WORKERS),
            cancel: self.cancel.unwrap_or_else(MinerCancel::new),
        }
    }
}

/// A multi-threaded pow nonce miner.
pub struct Miner {
    num_workers: usize,
    cancel: MinerCancel,
}

impl Miner {
    fn worker(cancel: MinerCancel, pow_digest: TritBuf<T1B1Buf>, start_nonce: u64, target_zeros: usize) -> Option<u64> {
        let mut nonce = start_nonce;
        let mut hasher = CurlPBatchHasher::<T1B1Buf>::new(HASH_LENGTH);
        let mut buffers = Vec::<TritBuf<T1B1Buf>>::with_capacity(BATCH_SIZE);

        for _ in 0..BATCH_SIZE {
            let mut buffer = TritBuf::<T1B1Buf>::zeros(HASH_LENGTH);
            buffer[..pow_digest.len()].copy_from(&pow_digest);
            buffers.push(buffer);
        }

        while !cancel.is_cancelled() {
            for (i, buffer) in buffers.iter_mut().enumerate() {
                let nonce_trits = b1t6::encode::<T1B1Buf>(&(nonce + i as u64).to_le_bytes());
                buffer[pow_digest.len()..pow_digest.len() + nonce_trits.len()].copy_from(&nonce_trits);
                hasher.add(buffer.clone());
            }

            for (i, hash) in hasher.hash().enumerate() {
                if count_trailing_zeros(&hash) >= target_zeros {
                    cancel.trigger();
                    return Some(nonce + i as u64);
                }
            }

            nonce += BATCH_SIZE as u64;
        }

        None
    }

    /// Mines a nonce for provided bytes.
    pub fn nonce(&self, bytes: &[u8], target_score: u32) -> Option<u64> {
        self.cancel.reset();

        let mut nonce = None;
        let mut pow_digest = TritBuf::<T1B1Buf>::new();
        // This should not be more than HASH_LENGTH but given the types of `bytes` and `target_score`, its maximum value
        // depending on user input is ceil(ln(usize::MAX * u32::MAX) / ln(3)) = 61.
        let target_zeros = ((((bytes.len() + std::mem::size_of::<u64>()) as f64).ln() + (target_score as f64).ln())
            / LN_3)
            .ceil() as usize;

        let worker_width = u64::MAX / self.num_workers as u64;
        let mut workers = Vec::with_capacity(self.num_workers);
        let hash = Blake2b256::digest(bytes);

        b1t6::encode::<T1B1Buf>(&hash).iter().for_each(|t| pow_digest.push(t));

        for i in 0..self.num_workers {
            let start_nonce = i as u64 * worker_width;
            let _cancel = self.cancel.clone();
            let _pow_digest = pow_digest.clone();

            workers.push(thread::spawn(move || {
                Self::worker(_cancel, _pow_digest, start_nonce, target_zeros)
            }));
        }

        for worker in workers {
            if let Some(mined_nonce) = worker.join().unwrap() {
                nonce.replace(mined_nonce);
            }
        }

        nonce
    }
}

fn _get_miner(bytes: &[u8], min_pow_score: u32, num_workers: usize) -> Option<u64> {
    MinerBuilder::new()
        .with_num_workers(num_workers)
        .finish()
        .nonce(bytes, min_pow_score)
}

/// Returns a closure for a miner with `num_cpus` workers.
pub fn get_miner(min_pow_score: u32) -> impl Fn(&[u8]) -> Option<u64> {
    move |bytes| _get_miner(bytes, min_pow_score, num_cpus::get())
}

/// Returns a closure for a miner with `num_workers` workers.
pub fn get_miner_num_workers(min_pow_score: u32, num_workers: usize) -> impl Fn(&[u8]) -> Option<u64> {
    move |bytes| _get_miner(bytes, min_pow_score, num_workers)
}
