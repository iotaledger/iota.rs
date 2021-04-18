// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::{Error, Result};
use crate::helper::get_max_normalized_bundle_hash;
use crate::{
    miner::{CrackabilityMinerEvent, Miner},
    success,
};
use bee_ternary::{t3b1::T3B1Buf, T1B1Buf, TritBuf};

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
    known_bundle_hashes: Option<Vec<TritBuf<T1B1Buf>>>,
    /// The threshold of the recoverer.
    threshold: Option<f64>,
    /// The bundle miner used in recoverer.
    miner: Option<Miner>,
}

pub struct Recoverer {
    /// The security level of the input hashes.
    security_level: usize,
    /// The input bundle hashes for recovering.
    known_bundle_hashes: Vec<TritBuf<T1B1Buf>>,
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
    pub fn with_known_bundle_hashes(mut self, known_bundle_hashes: Vec<TritBuf<T1B1Buf>>) -> Self {
        self.known_bundle_hashes.replace(known_bundle_hashes);
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
            known_bundle_hashes: match self.known_bundle_hashes {
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
    pub async fn recover(&mut self) -> CrackabilityMinerEvent {
        let target_crackability =
            get_crack_probability(self.security_level, &self.known_bundle_hashes);
        self.miner
            .run(Some(target_crackability), Some(self.threshold))
            .await
            .unwrap()
    }
}
