// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-bundle-miner crate.

/// Type alias of `Result` in iota-bundle-miner
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the iota bundle miner crate.
pub enum Error {
    /// IO error.
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// Counters MutexGuard error.
    #[error("Counters MutexGuard error")]
    CounterPoisonError,
    /// Crackability MutexGuard error.
    #[error("Crackability MutexGuard error")]
    CrackabilityPoisonError,
    /// The kown bundle hashes are not set.
    #[error("The kown bundle hashes should be set")]
    KownBundleHashesNotSet,
    /// The essences from unsigned bundle are not set.
    #[error("The essences from unsigned bundle should be set")]
    EssencesFromUnsignedBundleNotSet,
    /// The ternary conversion error.
    #[error("Ternary conversion error")]
    InvalidConversion,
    /// The essences to mine are empty.
    #[error("The essences to mine are empty")]
    EmptyEssenceToMine,
    /// The normalized hashes are empty.
    #[error("The normalized hashes are empty")]
    EmptyNormalizedHashes,
    /// The security level in recoverer needs to be set.
    #[error(
        "The recoverer security level should be set to calculate the target crack probability"
    )]
    RecovererSecurityLevelNotSet,
    /// The miner should be set for recoverer.
    #[error("The miner of the recoverer should be set")]
    MinerInRecovererNotSet,
}

impl std::convert::From<()> for Error {
    fn from(_: ()) -> Self {
        Error::InvalidConversion
    }
}
