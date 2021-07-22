// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::Client as IotaClient;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::message_builder::MessageBuilder;
use crate::message_getter::MessageGetter;
use crate::utils::err;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Client {
  pub(crate) client: Rc<IotaClient>,
}

#[wasm_bindgen]
impl Client {
  /// Get the nodeinfo.
  #[wasm_bindgen(js_name = getInfo)]
  pub fn get_info(&self) -> Result<Promise, JsValue> {
    let client: Rc<IotaClient> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .get_info()
        .await
        .map_err(err)
        .and_then(|receipt| JsValue::from_serde(&receipt).map_err(err))
    });

    Ok(promise)
  }

  /// Send a message to the Tangle.
  pub fn message(&self) -> MessageBuilder {
    MessageBuilder::new(self.clone())
  }
  /// Get a message from the Tangle.
  #[wasm_bindgen(js_name = getMessage)]
  pub fn get_message(&self) -> MessageGetter {
    MessageGetter::new(self.clone())
  }
}
