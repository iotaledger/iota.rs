// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! # Python binding implementation for Client library.

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

/// The client library of python binding.
pub mod types;
use std::sync::Mutex;

use iota_client::message_interface::Message;
use once_cell::sync::OnceCell;
use pyo3::{prelude::*, wrap_pyfunction};
use tokio::runtime::Runtime;
use types::*;

pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}

#[pyfunction]
/// Create message handler for python-side usage.
pub fn create_message_handler(options: Option<String>) -> Result<ClientMessageHandler> {
    let message_handler =
        crate::block_on(async { iota_client::message_interface::create_message_handler(options).await })?;

    Ok(ClientMessageHandler {
        client_message_handler: message_handler,
    })
}

#[pyfunction]
/// Send message through handler.
pub fn send_message(handle: &ClientMessageHandler, message: String) -> Result<String> {
    let message = match serde_json::from_str::<Message>(&message) {
        Ok(message) => message,
        Err(e) => {
            panic!("Wrong message type! {:?}", e);
        }
    };
    let response = crate::block_on(async {
        iota_client::message_interface::send_message(&handle.client_message_handler, message).await
    });
    Ok(serde_json::to_string(&response)?)
}

/// IOTA Client implemented in Rust for Python binding.
#[pymodule]
fn iota_client(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_message_handler, m)?).unwrap();
    m.add_function(wrap_pyfunction!(send_message, m)?).unwrap();
    Ok(())
}
