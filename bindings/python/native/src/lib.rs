// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod client;
use client::Client;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn iota_client(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    Ok(())
}
