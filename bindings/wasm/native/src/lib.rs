// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;

#[macro_use]
mod macros;
mod address_getter;
mod balance_getter;
mod client;
mod client_builder;
mod message_builder;
mod message_getter;
mod unspent_address_getter;
mod utils;

pub use client_builder::ClientBuilder;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  Ok(())
}
