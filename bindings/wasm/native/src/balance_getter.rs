// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{client::Client, error::wasm_error};
use iota_client::Seed;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
#[derive(Clone)]
pub struct BalanceGetter {
  client: Client,
  seed: String,
  account_index: Option<usize>,
  gap_limit: Option<usize>,
  initial_address_index: Option<usize>,
}

#[wasm_bindgen]
impl BalanceGetter {
  pub fn new(client: Client, seed: String) -> Self {
    Self {
      client,
      seed,
      account_index: None,
      gap_limit: None,
      initial_address_index: None,
    }
  }

  /// Sets the account index
  #[wasm_bindgen(js_name = accountIndex)]
  pub fn account_index(&mut self, index: usize) -> Result<BalanceGetter, JsValue> {
    self.account_index.replace(index);
    Ok(self.clone())
  }

  /// Sets the address index from which to start looking for balance
  #[wasm_bindgen(js_name = initialAddressIndex)]
  pub fn initial_address_index(&mut self, initial_address_index: usize) -> Result<BalanceGetter, JsValue> {
    self.initial_address_index.replace(initial_address_index);
    Ok(self.clone())
  }

  /// Sets the gap limit to specify how many addresses will be checked each round.
  /// If gap_limit amount of addresses in a row have no balance the function will return.
  #[wasm_bindgen(js_name = gap_limit)]
  pub fn gap_limit(&mut self, gap_limit: usize) -> Result<BalanceGetter, JsValue> {
    self.gap_limit.replace(gap_limit);
    Ok(self.clone())
  }

  /// Get the balance.
  #[wasm_bindgen]
  pub fn get(&self) -> Result<Promise, JsValue> {
    let options = self.clone();
    let promise: Promise = future_to_promise(async move {
      let seed = Seed::from_bytes(&hex::decode(&options.seed).map_err(wasm_error)?);
      let mut balance_getter = options.client.client.get_balance(&seed);
      if let Some(account_index) = options.account_index {
        balance_getter = balance_getter.with_account_index(account_index);
      }
      if let Some(index) = options.initial_address_index {
        balance_getter = balance_getter.with_initial_address_index(index);
      }
      if let Some(gap_limit) = options.gap_limit {
        balance_getter = balance_getter.with_gap_limit(gap_limit);
      }

      balance_getter
        .finish()
        .await
        .map_err(wasm_error)
        .and_then(|balance| JsValue::from_serde(&balance).map_err(wasm_error))
    });
    Ok(promise)
  }
}
