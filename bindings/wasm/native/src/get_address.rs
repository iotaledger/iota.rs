// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{client::Client, error::wasm_error};
use iota_client::node::OutputsOptions;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
#[derive(Clone)]
pub struct GetAddressBuilder {
  client: Client,
}

#[wasm_bindgen]
impl GetAddressBuilder {
  pub fn new(client: Client) -> Self {
    Self { client }
  }

  /// Consume the builder and get the balance of a given Bech32 encoded address.
  /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
  /// reasons. User should sweep the address to reduce the amount of outputs.
  #[wasm_bindgen]
  pub fn balance(&self, address: String) -> Result<Promise, JsValue> {
    #[derive(Serialize)]
    struct BalanceAddressResponseDto {
      pub address: String,
      pub balance: u64,
      #[serde(rename = "dustAllowed")]
      pub dust_allowed: bool,
      #[serde(rename = "ledgerIndex")]
      pub ledger_index: u32,
    }
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      let res = client
        .client
        .get_address()
        .balance(&address)
        .await
        .map_err(wasm_error)?;
      let balance = BalanceAddressResponseDto {
        address: client
          .client
          .hex_to_bech32(&res.address, None)
          .await
          .map_err(wasm_error)?,
        balance: res.balance,
        dust_allowed: res.dust_allowed,
        ledger_index: res.ledger_index,
      };
      JsValue::from_serde(&balance).map_err(wasm_error)
    });
    Ok(promise)
  }

  /// Consume the builder and get all outputs that use a given address.
  /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
  /// reasons. User should sweep the address to reduce the amount of outputs.
  #[wasm_bindgen]
  pub fn outputs(&self, address: String, options: JsValue) -> Result<Promise, JsValue> {
    let client = self.client.clone();
    let options: OutputsOptions = options.into_serde().unwrap_or_default();
    let promise: Promise = future_to_promise(async move {
      client
        .client
        .get_address()
        .outputs(&address, options)
        .await
        .map_err(wasm_error)
        .and_then(|outputs| JsValue::from_serde(&outputs).map_err(wasm_error))
    });
    Ok(promise)
  }
}
