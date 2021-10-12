// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate serde;

use iota_client::{bee_message::MessageId, bee_rest_api::types::dtos::MessageDto};
use wasm_bindgen::prelude::*;

#[macro_use]
mod macros;
pub mod address_getter;
pub mod balance_getter;
pub mod client;
pub mod client_builder;
mod error;
pub mod get_address;
pub mod message_builder;
pub mod message_getter;
pub mod unspent_address_getter;

pub use client_builder::ClientBuilder;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  Ok(())
}

/// Message wrapper to have the messageId together with the message
#[derive(Serialize)]
pub struct MessageWrapper {
  #[serde(rename = "messageId")]
  message_id: MessageId,
  message: MessageDto,
}
