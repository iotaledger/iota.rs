// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

/// Errors occurring when computing nonces.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
    /// The worker has been cancelled.
    #[error("the worker has been cancelled")]
    Cancelled,
}
