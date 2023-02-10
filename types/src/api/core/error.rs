// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::Error as BlockError;

#[derive(Debug)]
pub enum Error {
    InvalidField(&'static str),
    Block(BlockError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidField(field) => write!(f, "invalid field \"{field}\""),
            Self::Block(error) => write!(f, "{error}"),
        }
    }
}

impl From<BlockError> for Error {
    fn from(error: BlockError) -> Self {
        Self::Block(error)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
