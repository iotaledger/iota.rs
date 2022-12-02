// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Contains nonce providers for Proof of Work.

pub mod miner;
#[cfg(target_family = "wasm")]
pub mod wasm_miner;

use thiserror::Error;

// Precomputed natural logarithm of 3 for performance reasons.
// See https://oeis.org/A002391.
const LN_3: f64 = 1.098_612_288_668_109;

/// Errors occurring when computing nonces with the `Miner` nonce provider.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
    /// The worker has been cancelled.
    #[error("the worker has been cancelled")]
    Cancelled,
    /// Invalid proof of work score.
    #[error("invalid proof of work score {0}, requiring {1} trailing zeros")]
    InvalidPowScore(u32, usize),
}
