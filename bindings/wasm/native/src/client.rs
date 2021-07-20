// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use futures::executor;
use iota_client::Client as IotaClient;
use iota_client::bee_message::MessageId;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::client_builder::Config;
use crate::utils::err;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Client {
  pub(crate) client: Rc<IotaClient>,
}

#[wasm_bindgen]
impl Client {
  /// Creates a new `Client` with a node.
  #[wasm_bindgen(js_name = withNode)]
  pub fn with_node(url: &str) -> Result<Client, JsValue>  {
    let builder_future = IotaClient::builder().with_node(url).map_err(err)?.finish();
    let new_client = executor::block_on(builder_future).map_err(err)?;
    Ok(Self {
      client: Rc::new(new_client),
    })
  }

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
  pub fn message(&self) -> Result<Promise, JsValue> {
    let client: Rc<IotaClient> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .message()
        .finish()
        .await
        .map_err(err)
        .and_then(|receipt| JsValue::from_serde(&receipt).map_err(err))
    });

    Ok(promise)
  }

  /// Get a message with a message id.
  #[wasm_bindgen(js_name = getMessage)]
  pub fn get_message(&self, message_id: &str) -> Result<Promise, JsValue> {
    let message: MessageId = MessageId::from_str(message_id).map_err(err)?;
    let client: Rc<IotaClient> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .get_message().raw(&message)
        .await
        .map_err(err)
        .and_then(|receipt| JsValue::from_serde(&receipt).map_err(err))
    });

    Ok(promise)
  }

}
