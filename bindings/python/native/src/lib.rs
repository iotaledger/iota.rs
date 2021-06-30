// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! # Python binding implementation for Client library.

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

/// The client library of python binding.
pub mod client;
use client::Client;
use pyo3::prelude::*;

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use tokio::runtime::Runtime;

pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}

pub(crate) fn spawn_blocking<F, R>(f: F) -> tokio::task::JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().spawn_blocking(f)
}

/// A Python module implemented in Rust.
#[pymodule]
fn iota_client(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    Ok(())
}
