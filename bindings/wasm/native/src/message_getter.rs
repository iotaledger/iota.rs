// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{client::Client, error::wasm_error, MessageWrapper};
use iota_client::{bee_message::MessageId, bee_rest_api::types::dtos::MessageDto};
use js_sys::Promise;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
#[derive(Clone)]
pub struct MessageGetter {
  client: Client,
}

#[wasm_bindgen]
impl MessageGetter {
  pub fn new(client: Client) -> Self {
    Self { client }
  }

  /// Get message ids with an index.
  #[wasm_bindgen]
  pub fn index(&self, index: Vec<u8>) -> Result<Promise, JsValue> {
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .client
        .get_message()
        .index(index)
        .await
        .map_err(wasm_error)
        .and_then(|message_ids| JsValue::from_serde(&message_ids).map_err(wasm_error))
    });
    Ok(promise)
  }

  /// Get a message with the message id.
  #[wasm_bindgen]
  pub fn data(&self, message_id: String) -> Result<Promise, JsValue> {
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .client
        .get_message()
        .data(&MessageId::from_str(&message_id).map_err(wasm_error)?)
        .await
        .map_err(wasm_error)
        .and_then(|message| {
          let message_id = message.id().0;
          JsValue::from_serde(&MessageWrapper {
            message_id,
            message: MessageDto::from(&message),
          })
          .map_err(wasm_error)
        })
    });
    Ok(promise)
  }

  /// Get the raw message with the message id.
  #[wasm_bindgen]
  pub fn raw(&self, message_id: String) -> Result<Promise, JsValue> {
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .client
        .get_message()
        .raw(&MessageId::from_str(&message_id).map_err(wasm_error)?)
        .await
        .map_err(wasm_error)
        .and_then(|message| JsValue::from_serde(&message).map_err(wasm_error))
    });
    Ok(promise)
  }

  /// Get the childrens of a message with the message id.
  #[wasm_bindgen]
  pub fn children(&self, message_id: String) -> Result<Promise, JsValue> {
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .client
        .get_message()
        .children(&MessageId::from_str(&message_id).map_err(wasm_error)?)
        .await
        .map_err(wasm_error)
        .and_then(|message| JsValue::from_serde(&message).map_err(wasm_error))
    });
    Ok(promise)
  }

  /// Get the metadata of a message with the message id.
  #[wasm_bindgen]
  pub fn metadata(&self, message_id: String) -> Result<Promise, JsValue> {
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .client
        .get_message()
        .metadata(&MessageId::from_str(&message_id).map_err(wasm_error)?)
        .await
        .map_err(wasm_error)
        .and_then(|message| JsValue::from_serde(&message).map_err(wasm_error))
    });
    Ok(promise)
  }
}
