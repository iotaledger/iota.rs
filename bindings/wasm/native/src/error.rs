// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Copied from https://github.com/iotaledger/identity.rs/blob/dev/bindings/wasm/src/error.rs

use std::borrow::Cow;
use wasm_bindgen::JsValue;

/// Convert an error into an idiomatic [js_sys::Error].
pub fn wasm_error<'a, T>(error: T) -> JsValue
where
  T: Into<WasmError<'a>>,
{
  let wasm_err: WasmError = error.into();
  JsValue::from(wasm_err)
}

/// Convenience struct to convert internal errors to [js_sys::Error]. Uses [std::borrow::Cow]
/// internally to avoid unnecessary clones.
///
/// This is a workaround for orphan rules so we can implement [core::convert::From] on errors from
/// dependencies.
#[derive(Debug, Clone)]
pub struct WasmError<'a> {
  pub name: Cow<'a, str>,
  pub message: Cow<'a, str>,
}

impl<'a> WasmError<'a> {
  pub fn new(name: Cow<'a, str>, message: Cow<'a, str>) -> Self {
    Self { name, message }
  }
}

/// Convert [WasmError] into [js_sys::Error] for idiomatic error handling.
impl From<WasmError<'_>> for js_sys::Error {
  fn from(error: WasmError<'_>) -> Self {
    let js_error = js_sys::Error::new(&error.message);
    js_error.set_name(&error.name);
    js_error
  }
}

/// Convert [WasmError] into [wasm_bindgen::JsValue].
impl From<WasmError<'_>> for JsValue {
  fn from(error: WasmError<'_>) -> Self {
    JsValue::from(js_sys::Error::from(error))
  }
}

impl From<serde_json::Error> for WasmError<'_> {
  fn from(error: serde_json::Error) -> Self {
    Self {
      name: Cow::Borrowed("serde_json::Error"), // the exact error code is embedded in the message
      message: Cow::Owned(error.to_string()),
    }
  }
}

impl From<iota_client::Error> for WasmError<'_> {
  fn from(error: iota_client::Error) -> Self {
    Self {
      name: Cow::Borrowed("iota_client::Error"), // the exact error code is embedded in the message
      message: Cow::Owned(error.to_string()),
    }
  }
}

impl From<iota_client::bee_rest_api::types::error::Error> for WasmError<'_> {
  fn from(error: iota_client::bee_rest_api::types::error::Error) -> Self {
    Self {
      name: Cow::Borrowed("bee_rest_api::types::error::Error"), // the exact error code is embedded in the message
      message: Cow::Owned(error.to_string()),
    }
  }
}

impl From<hex::FromHexError> for WasmError<'_> {
  fn from(error: hex::FromHexError) -> Self {
    Self {
      name: Cow::Borrowed("hex::FromHexError"),
      message: Cow::Owned(error.to_string()),
    }
  }
}

impl From<iota_client::bee_message::Error> for WasmError<'_> {
  fn from(error: iota_client::bee_message::Error) -> Self {
    Self {
      name: Cow::Borrowed("bee_message::Error"),
      message: Cow::Owned(error.to_string()),
    }
  }
}
