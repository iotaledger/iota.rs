// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

pub mod message_handler;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Initializes the console error panic hook for better panic messages.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

/// The Wasm bindings do not support internal logging yet.
///
/// Calling this is a no-op, only included for compatibility with the Node.js bindings TypeScript definitions.
#[wasm_bindgen(js_name = initLogger)]
pub fn init_logger(_config: JsValue) {
    // TODO
}
