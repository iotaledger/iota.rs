// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Invalid participations error
    #[error("invalid participations")]
    InvalidParticipations,
    /// IO error
    #[error("`{0}`")]
    Io(#[from] std::io::Error),
}
