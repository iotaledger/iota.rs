// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

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
