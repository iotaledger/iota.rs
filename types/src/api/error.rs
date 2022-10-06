// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

use crate::block::Error as BlockError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid field \"{0}\"")]
    InvalidField(&'static str),
    #[error("{0}")]
    Block(#[from] BlockError),
}
