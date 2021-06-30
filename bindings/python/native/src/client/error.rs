// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{bee_message, error::Error as RustError};
use pyo3::{exceptions, prelude::*};
use std::convert::From;

/// The `Result` structure to wrap the error type for python binding.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// The Error type.
pub struct Error {
    /// The error exposed to python.
    pub error: PyErr,
}

impl std::convert::From<RustError> for Error {
    fn from(err: RustError) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl std::convert::From<Error> for PyErr {
    fn from(err: Error) -> Self {
        err.error
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyIOError, _>(err.to_string()),
        }
    }
}

impl From<bee_message::Error> for Error {
    fn from(err: bee_message::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyTypeError, _>(err.to_string()),
        }
    }
}
