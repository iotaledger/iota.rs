// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug)]
pub enum Error {
    /// Invalid participations error
    InvalidParticipations,
    /// IO error
    Io(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidParticipations => write!(f, "invalid participations"),
            Self::Io(error) => write!(f, "{error}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
