// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{client::Client, error::wasm_error};
use iota_client::Seed;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
#[derive(Clone)]
pub struct UnspentAddressGetter {
  client: Client,
  seed: String,
  account_index: Option<usize>,
  initial_address_index: Option<usize>,
}

#[wasm_bindgen]
impl UnspentAddressGetter {
  pub fn new(client: Client, seed: String) -> Self {
    Self {
      client,
      seed,
      account_index: None,
      initial_address_index: None,
    }
  }

  /// Sets the account index
  #[wasm_bindgen(js_name = accountIndex)]
  pub fn account_index(&mut self, index: usize) -> Result<UnspentAddressGetter, JsValue> {
    self.account_index.replace(index);
    Ok(self.clone())
  }

  /// Sets the index of the address to start looking for balance
  #[wasm_bindgen(js_name = initialAddressIndex)]
  pub fn initial_address_index(&mut self, index: usize) -> Result<UnspentAddressGetter, JsValue> {
    self.initial_address_index.replace(index);
    Ok(self.clone())
  }

  /// Get an unspent address with its index.
  #[wasm_bindgen]
  pub fn get(&self) -> Result<Promise, JsValue> {
    let options = self.clone();
    let promise: Promise = future_to_promise(async move {
      let seed = Seed::from_bytes(&hex::decode(&options.seed).map_err(wasm_error)?);
      let mut address_gettter = options.client.client.get_unspent_address(&seed);
      if let Some(account_index) = options.account_index {
        address_gettter = address_gettter.with_account_index(account_index);
      }
      if let Some(index) = options.initial_address_index {
        address_gettter = address_gettter.with_initial_address_index(index);
      }
      address_gettter
        .get()
        .await
        .map_err(wasm_error)
        .and_then(|address| JsValue::from_serde(&address).map_err(wasm_error))
    });
    Ok(promise)
  }
}
