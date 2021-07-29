// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::address_getter::AddressGetter;
use crate::balance_getter::BalanceGetter;
use crate::message_builder::MessageBuilder;
use crate::message_getter::MessageGetter;
use crate::unspent_address_getter::UnspentAddressGetter;
use crate::utils::err;
use iota_client::bee_message::parents::Parents;
use iota_client::bee_message::MessageBuilder as RustMessageBuilder;
use iota_client::bee_message::MessageId;
use iota_client::bee_rest_api::types::dtos::PayloadDto;
use iota_client::common::packable::Packable;
use iota_client::Client as RustClient;
use iota_client::ClientMiner;
use iota_client::Seed;
use std::{convert::TryInto, str::FromStr};
// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

/// Struct for PostMessage
#[derive(Serialize, Deserialize)]
pub struct MessageDto {
  pub parents: Option<Vec<String>>,
  pub payload: PayloadDto,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Client {
  pub(crate) client: Rc<RustClient>,
}

#[wasm_bindgen]
impl Client {
  /// Send a message to the Tangle.
  pub fn message(&self) -> MessageBuilder {
    MessageBuilder::new(self.clone())
  }
  /// Get a message from the Tangle.
  #[wasm_bindgen(js_name = getMessage)]
  pub fn get_message(&self) -> MessageGetter {
    MessageGetter::new(self.clone())
  }
  /// Generate addresses.
  #[wasm_bindgen(js_name = getAddresses)]
  pub fn get_addresses(&self, seed: String) -> AddressGetter {
    AddressGetter::new(self.clone(), seed)
  }
  /// Get an unspent address.
  #[wasm_bindgen(js_name = getUnspentAddress)]
  pub fn get_unspent_address(&self, seed: String) -> UnspentAddressGetter {
    UnspentAddressGetter::new(self.clone(), seed)
  }
  /// Get the account balance.
  #[wasm_bindgen(js_name = getBalance)]
  pub fn get_balance(&self, seed: String) -> BalanceGetter {
    BalanceGetter::new(self.clone(), seed)
  }

  /// Get the nodeinfo.
  #[wasm_bindgen(js_name = getInfo)]
  pub fn get_info(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_info()
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Get the node health.
  #[wasm_bindgen(js_name = getHealth)]
  pub fn get_health(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_health()
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Get tips.
  #[wasm_bindgen(js_name = getTips)]
  pub fn get_tips(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_tips()
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Get peers.
  #[wasm_bindgen(js_name = getPeers)]
  pub fn get_peers(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_peers()
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Post message.
  #[wasm_bindgen(js_name = postMessage)]
  pub fn post_message(&self, message: JsValue) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    let message: MessageDto = serde_json::from_value(message.into_serde().map_err(err)?).map_err(err)?;
    Ok(future_to_promise(async move {
      let mut parent_msg_ids = match message.parents.as_ref() {
        Some(parents) => {
          let mut parent_ids = Vec::new();
          for msg_id in parents {
            parent_ids.push(MessageId::from_str(&msg_id).map_err(err)?)
          }
          parent_ids
        }
        None => client.get_tips().await.map_err(err)?,
      };
      parent_msg_ids.sort_unstable_by_key(|a| a.pack_new());
      parent_msg_ids.dedup();
      let network_id = client.get_network_id().await.map_err(err)?;
      let nonce_provider = client.get_pow_provider().await;
      let min_pow_score = client.get_min_pow_score().await.map_err(err)?;
      let message = RustMessageBuilder::<ClientMiner>::new()
        .with_network_id(network_id)
        .with_parents(Parents::new(parent_msg_ids).map_err(err)?)
        .with_nonce_provider(nonce_provider, min_pow_score)
        .with_payload((&message.payload).try_into().map_err(err)?)
        .finish()
        .map_err(err)?;
      client
        .post_message(&message)
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Retries (promotes or reattaches) a message for provided message id. Message should only be
  /// retried only if they are valid and haven't been confirmed for a while.
  #[wasm_bindgen]
  pub fn retry(&self, message_id: String) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .retry(&MessageId::from_str(&message_id).map_err(err)?)
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Only works in browser because of the timeouts
  /// Retries (promotes or reattaches) a message for provided message id until it's included (referenced by a
  /// milestone). Default interval is 5 seconds and max attempts is 10. Returns reattached messages
  #[wasm_bindgen(js_name = retryUntilIncluded)]
  pub fn retry_until_included(
    &self,
    message_id: String,
    interval: Option<u64>,
    max_attempts: Option<u64>,
  ) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .retry_until_included(&MessageId::from_str(&message_id).map_err(err)?, interval, max_attempts)
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
  /// confirmed for a while.
  #[wasm_bindgen]
  pub fn reattach(&self, message_id: String) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .reattach(&MessageId::from_str(&message_id).map_err(err)?)
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
  /// method should error out and should not allow unnecessary promotions.
  #[wasm_bindgen]
  pub fn promote(&self, message_id: String) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .promote(&MessageId::from_str(&message_id).map_err(err)?)
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Only works in browser because of the timeouts
  /// Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
  /// Returns the address to which the funds got consolidated, if any were available
  #[wasm_bindgen(js_name = consolidateFunds)]
  pub fn consolidate_funds(
    &self,
    seed: String,
    account_index: usize,
    start_index: usize,
    end_index: usize,
  ) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    let seed = Seed::from_bytes(&hex::decode(&seed).map_err(err)?);
    Ok(future_to_promise(async move {
      client
        .consolidate_funds(&seed, account_index, start_index..end_index)
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }

  /// Returns a parsed hex String from bech32.
  #[wasm_bindgen(js_name = bech32ToHex)]
  pub fn bech32_to_hex(&self, address: &str) -> Result<String, JsValue> {
    RustClient::bech32_to_hex(address).map_err(err)
  }

  /// Returns a parsed bech32 String from hex.
  #[wasm_bindgen(js_name = hexToBech32)]
  pub fn hex_to_bech32(&self, address: String, bech32: Option<String>) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .hex_to_bech32(&address, bech32.as_deref())
        .await
        .map_err(err)
        .and_then(|res| JsValue::from_serde(&res).map_err(err))
    }))
  }
}
