// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{client::Client, error::wasm_error};
use iota_client::Seed;
use js_sys::Promise;
use std::ops::Range;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
#[derive(Clone)]
pub struct AddressGetter {
  client: Client,
  seed: String,
  account_index: Option<usize>,
  range: Option<Range<usize>>,
  bech32_hrp: Option<String>,
  include_internal: bool,
}

#[wasm_bindgen]
impl AddressGetter {
  pub fn new(client: Client, seed: String) -> Self {
    Self {
      client,
      seed,
      account_index: None,
      range: None,
      bech32_hrp: None,
      include_internal: false,
    }
  }

  /// Set the account index
  #[wasm_bindgen(js_name = accountIndex)]
  pub fn account_index(&mut self, index: usize) -> Result<AddressGetter, JsValue> {
    self.account_index.replace(index);
    Ok(self.clone())
  }

  /// Set the address range
  #[wasm_bindgen]
  pub fn range(&mut self, start: usize, end: usize) -> Result<AddressGetter, JsValue> {
    self.range.replace(start..end);
    Ok(self.clone())
  }

  /// Set the bech32 hrp
  #[wasm_bindgen(js_name = bech32Hrp)]
  pub fn bech32_hrp(&mut self, bech32_hrp: String) -> Result<AddressGetter, JsValue> {
    self.bech32_hrp.replace(bech32_hrp);
    Ok(self.clone())
  }

  /// Include internal addresses
  #[wasm_bindgen(js_name = includeInternal)]
  pub fn include_internal(&mut self) -> Result<AddressGetter, JsValue> {
    self.include_internal = true;
    Ok(self.clone())
  }

  /// Get the addresses.
  #[wasm_bindgen]
  pub fn get(&self) -> Result<Promise, JsValue> {
    let options = self.clone();
    let promise: Promise = future_to_promise(async move {
      let seed = Seed::from_bytes(&hex::decode(&options.seed).map_err(wasm_error)?);
      let mut address_gettter = options.client.client.get_addresses(&seed);
      if let Some(account_index) = options.account_index {
        address_gettter = address_gettter.with_account_index(account_index);
      }
      if let Some(range) = options.range {
        address_gettter = address_gettter.with_range(range);
      }
      if let Some(bech32_hrp) = options.bech32_hrp {
        address_gettter = address_gettter.with_bech32_hrp(bech32_hrp);
      }
      if options.include_internal {
        address_gettter
          .get_all()
          .await
          .map_err(wasm_error)
          .and_then(|addresses| JsValue::from_serde(&addresses).map_err(wasm_error))
      } else {
        address_gettter
          .finish()
          .await
          .map_err(wasm_error)
          .and_then(|addresses| JsValue::from_serde(&addresses).map_err(wasm_error))
      }
    });
    Ok(promise)
  }
}
