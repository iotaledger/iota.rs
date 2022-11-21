// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::{From, Infallible};

use pyo3::{exceptions, prelude::*};

/// The `Result` structure to wrap the error type for python binding.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// The Error type.
pub struct Error {
    /// The error exposed to python.
    pub error: PyErr,
}

impl From<iota_client::error::Error> for Error {
    fn from(err: iota_client::error::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<Error> for PyErr {
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

impl From<iota_client::block::Error> for Error {
    fn from(err: iota_client::block::Error) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}

impl From<Infallible> for Error {
    fn from(err: Infallible) -> Self {
        Error {
            error: PyErr::new::<exceptions::PyValueError, _>(err.to_string()),
        }
    }
}
