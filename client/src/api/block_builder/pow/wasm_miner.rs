// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Single-threaded PoW miner

use bee_pow::providers::{NonceProvider, NonceProviderBuilder};
use bee_ternary::{b1t6, Btrit, T1B1Buf, TritBuf};
use crypto::hashes::{
    blake2b::Blake2b256,
    ternary::{
        curl_p::{CurlPBatchHasher, BATCH_SIZE},
        HASH_LENGTH,
    },
    Digest,
};

// Precomputed natural logarithm of 3 for performance reasons.
// See https://oeis.org/A002391.
const LN_3: f64 = 1.098_612_288_668_109;

// Should take around one second to reach on an average CPU,
// so shouldn't cause a noticeable delay on tips_interval.
const POW_ROUNDS_BEFORE_INTERVAL_CHECK: usize = 3000;

/// Single-threaded proof-of-work for Wasm.
pub struct SingleThreadedMiner {
    local_pow: bool,
    tips_interval_secs: Option<u64>,
}

/// Builder for [`SingleThreadedMiner`].
#[derive(Default)]
#[must_use]
pub struct SingleThreadedMinerBuilder {
    local_pow: bool,
    tips_interval_secs: Option<u64>,
}

impl SingleThreadedMinerBuilder {
    /// Immediately return a default 0 nonce if false (remote proof-of-work).
    pub fn local_pow(mut self, local_pow: bool) -> Self {
        self.local_pow = local_pow;
        self
    }

    /// Aborts and returns a "cancelled" error after the interval elapses, if set.
    /// New parents (tips) should be fetched and proof-of-work re-run afterwards.
    pub fn tips_interval_secs(mut self, tips_interval_secs: u64) -> Self {
        self.tips_interval_secs = Some(tips_interval_secs);
        self
    }
}

impl NonceProviderBuilder for SingleThreadedMinerBuilder {
    type Provider = SingleThreadedMiner;

    fn finish(self) -> Self::Provider {
        SingleThreadedMiner {
            local_pow: self.local_pow,
            tips_interval_secs: self.tips_interval_secs,
        }
    }
}

impl NonceProvider for SingleThreadedMiner {
    type Builder = SingleThreadedMinerBuilder;
    type Error = crate::Error;

    fn nonce(&self, bytes: &[u8], target_score: u32) -> Result<u64, Self::Error> {
        // Remote proof-of-work will compute the block nonce.
        if !self.local_pow {
            return Ok(0);
        }

        let mut pow_digest = TritBuf::<T1B1Buf>::new();
        let target_zeros =
            (((bytes.len() + std::mem::size_of::<u64>()) as f64 * target_score as f64).ln() / LN_3).ceil() as usize;
        if target_zeros > HASH_LENGTH {
            return Err(crate::Error::Pow(
                bee_pow::providers::miner::Error::InvalidPowScore(target_score, target_zeros).to_string(),
            ));
        }

        let hash = Blake2b256::digest(bytes);
        b1t6::encode::<T1B1Buf>(&hash).iter().for_each(|t| pow_digest.push(t));

        let mut nonce = 0;
        let mut hasher = CurlPBatchHasher::<T1B1Buf>::new(HASH_LENGTH);
        let mut buffers = Vec::<TritBuf<T1B1Buf>>::with_capacity(BATCH_SIZE);
        for _ in 0..BATCH_SIZE {
            let mut buffer = TritBuf::<T1B1Buf>::zeros(HASH_LENGTH);
            buffer[..pow_digest.len()].copy_from(&pow_digest);
            buffers.push(buffer);
        }

        // Counter to reduce number of mining_start.elapsed() calls.
        let mut counter = 0;
        let mining_start = instant::Instant::now();
        loop {
            if let Some(tips_interval) = self.tips_interval_secs {
                if counter % POW_ROUNDS_BEFORE_INTERVAL_CHECK == 0
                    && mining_start.elapsed() > instant::Duration::from_secs(tips_interval)
                {
                    // Tips interval elapsed, cancel work and get new parents.
                    break;
                }
            }

            for (i, buffer) in buffers.iter_mut().enumerate() {
                let nonce_trits = b1t6::encode::<T1B1Buf>(&(nonce + i as u64).to_le_bytes());
                buffer[pow_digest.len()..pow_digest.len() + nonce_trits.len()].copy_from(&nonce_trits);
                hasher.add(buffer.clone());
            }
            for (i, hash) in hasher.hash().enumerate() {
                let trailing_zeros = hash.iter().rev().take_while(|t| *t == Btrit::Zero).count();
                if trailing_zeros >= target_zeros {
                    return Ok(nonce + i as u64);
                }
            }
            nonce += BATCH_SIZE as u64;
            counter += 1;
        }

        Err(crate::Error::Pow(
            bee_pow::providers::miner::Error::Cancelled.to_string(),
        ))
    }
}
