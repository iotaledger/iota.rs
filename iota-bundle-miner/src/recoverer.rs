// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::constant::MESSAGE_FRAGMENT_LENGTH;
use crate::error::{Error, Result};
use crate::{
    miner::{CrackabilityMinerEvent, Miner},
    success,
};
use bee_signing::ternary::wots::normalize;
use bee_ternary::{t3b1::T3B1Buf, T1B1Buf, TritBuf, Trits, T3B1};

/// CrackProbability estimates the probability that an attacker can successfully crack the given hashes.
pub fn get_crack_probability(security_level: usize, bundle_hashes: &[TritBuf<T1B1Buf>]) -> f64 {
    let max_hash = TritBuf::<T3B1Buf>::from_i8s(
        get_max_normalized_bundle_hash(bundle_hashes, security_level).as_i8_slice(),
    )
    .unwrap()
    .as_i8_slice()
    .to_vec();
    let mut p = 1.0_f64;
    for i in 0..security_level {
        p *= success(&max_hash[i * 27..i * 27 + 27].to_vec());
    }
    p
}

/// Builder for a recoverer.
#[derive(Default)]
pub struct RecovererBuilder {
    /// The security level of the input hashes.
    security_level: Option<usize>,
    /// The input bundle hashes for cracking.
    kown_bundle_hashes: Option<Vec<TritBuf<T1B1Buf>>>,
    /// The threshold of the recoverer.
    threshold: Option<f64>,
    /// The bundle miner used in recoverer.
    miner: Option<Miner>,
}

pub struct Recoverer {
    /// The security level of the input hashes.
    security_level: usize,
    /// The input bundle hashes for recovering.
    kown_bundle_hashes: Vec<TritBuf<T1B1Buf>>,
    /// The threshold of the recoverer.
    threshold: f64,
    /// The bundle miner used in recoverer.
    miner: Miner,
}

impl RecovererBuilder {
    /// Creates a new builder for a recoverer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the security level of the recoverer.
    pub fn with_security_level(mut self, security_level: usize) -> Self {
        self.security_level.replace(security_level);
        self
    }
    /// Sets the kown bundle hashes of the recoverer.
    pub fn with_kown_bundle_hashes(mut self, kown_bundle_hashes: Vec<TritBuf<T1B1Buf>>) -> Self {
        self.kown_bundle_hashes.replace(kown_bundle_hashes);
        self
    }
    /// Sets the threshold of the recoverer.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold.replace(threshold);
        self
    }
    /// Sets the bundle miner of the recoverer.
    pub fn miner(mut self, miner: Miner) -> Self {
        self.miner.replace(miner);
        self
    }
    /// Builds a recoverer.
    pub fn finish(self) -> Result<Recoverer> {
        Ok(Recoverer {
            security_level: match self.security_level {
                Some(level) => level,
                None => return Err(Error::RecovererSecurityLevelNotSet),
            },
            kown_bundle_hashes: match self.kown_bundle_hashes {
                Some(hashes) => hashes,
                None => return Err(Error::KownBundleHashesNotSet),
            },
            threshold: self.threshold.unwrap_or(0.0),
            miner: match self.miner {
                Some(miner) => miner,
                None => return Err(Error::MinerInRecovererNotSet),
            },
        })
    }
}

impl Recoverer {
    /// Start running mining workers
    pub fn recover(&mut self) -> CrackabilityMinerEvent {
        let target_crackability =
            get_crack_probability(self.security_level, &self.kown_bundle_hashes);
        self.miner
            .run(Some(target_crackability), Some(self.threshold))
            .unwrap()
    }
}

/// Get the maximum bundle hash by selecting the max trytes from all input bundle hashes
pub fn get_max_normalized_bundle_hash(
    bundle_hashes: &[TritBuf<T1B1Buf>],
    security_level: usize,
) -> TritBuf<T1B1Buf> {
    // Normalize the bundle hashes
    let mut normalized_hashes_i8_vecs = bundle_hashes
        .iter()
        .map(|t| {
            TritBuf::<T3B1Buf>::from_i8s(normalize(&t).unwrap().as_i8_slice())
                .unwrap()
                .as_i8_slice()
                .to_vec()
        })
        .collect::<Vec<Vec<i8>>>();

    // Get the max normalized bundle hash
    let mut max_vec_i8 = normalized_hashes_i8_vecs.pop().unwrap();
    while let Some(current_vec_i8) = normalized_hashes_i8_vecs.pop() {
        max_vec_i8 = get_the_max_tryte_values(max_vec_i8, current_vec_i8);
    }

    // Return the max normalized bundle hash in TritBuf::<T1B1Buf>
    unsafe {
        Trits::<T3B1>::from_raw_unchecked(
            &max_vec_i8[..MESSAGE_FRAGMENT_LENGTH * security_level],
            MESSAGE_FRAGMENT_LENGTH * security_level * 3,
        )
        .to_buf::<T3B1Buf>()
        .encode::<T1B1Buf>()
    }
}

/// Get max trytes values from two i8 vectors
pub fn get_the_max_tryte_values(vec_i8_first: Vec<i8>, vec_i8_second: Vec<i8>) -> Vec<i8> {
    vec_i8_first
        .iter()
        .zip(&vec_i8_second)
        .map(|(&x, &y)| x.max(y))
        .collect()
}
