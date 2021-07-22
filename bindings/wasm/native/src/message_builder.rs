// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::Client;
use crate::utils::err;
use iota_client::bee_message::address::Address;
use iota_client::bee_message::input::UtxoInput;
use iota_client::bee_message::MessageId;
use iota_client::bee_rest_api::types::dtos::MessageDto;
// use iota_client::Seed;
use js_sys::Promise;
use std::ops::Range;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[derive(Clone)]
pub struct MessageBuilderHelper {
  pub index: Option<Vec<u8>>,
  pub data: Option<Vec<u8>>,
  pub parents: Option<Vec<MessageId>>,
  pub seed: Option<String>,
  pub account_index: Option<usize>,
  pub initial_address_index: Option<usize>,
  pub inputs: Vec<UtxoInput>,
  pub input_range: Option<Range<usize>>,
  pub outputs: Vec<(Address, u64)>,
  pub dust_allowance_outputs: Vec<(Address, u64)>,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct MessageBuilder {
  builder: MessageBuilderHelper,
  client: Client,
}

#[wasm_bindgen]
impl MessageBuilder {
  // #[wasm_bindgen(constructor)]
  pub fn new(client: Client) -> Self {
    Self {
      builder: MessageBuilderHelper {
        index: None,
        data: None,
        parents: None,
        seed: None,
        account_index: None,
        initial_address_index: None,
        inputs: Vec::new(),
        input_range: None,
        outputs: Vec::new(),
        dust_allowance_outputs: Vec::new(),
      },
      client,
    }
  }

  #[wasm_bindgen]
  pub fn index(&mut self, index: &str) -> Result<MessageBuilder, JsValue> {
    self.builder.index.replace(index.into());
    // is there a way we can do it without the clone?
    Ok(self.clone())
  }

  // #[wasm_bindgen]
  // pub fn data(&mut self, data: Vec<u8>) -> Result<MessageBuilder, JsValue> {
  //   self.try_with_mut(|builder| builder.data.replace(data))?;
  //   // is there a way we can do it without the clone?
  //   Ok(self.clone())
  // }

  // #[wasm_bindgen(js_name = accountIndex)]
  // pub fn account_index(&mut self, account_index: usize) -> Result<MessageBuilder, JsValue> {
  //   self.try_with_mut(|builder| builder.with_account_index(account_index).map_err(err))?;
  //   Ok(self.clone())
  // }

  // pub(crate) fn take_builder(&mut self) -> Result<MessageBuilderHelper, JsValue> {
  //   self.builder.take().ok_or_else(|| "Message Builder Consumed".into())
  // }

  // fn with_mut(&mut self, f: impl Fn(MessageBuilderHelper) -> MessageBuilderHelper) -> Result<(), JsValue> {
  //   self.builder = Some(f(self.take_builder()?));
  //   Ok(())
  // }

  // fn try_with_mut(
  //   &mut self,
  //   f: impl Fn(MessageBuilderHelper) -> Result<MessageBuilderHelper, JsValue>,
  // ) -> Result<(), JsValue> {
  //   self.builder = Some(f(self.take_builder()?)?);
  //   Ok(())
  // }

  /// Build the client.
  #[wasm_bindgen]
  pub fn submit(&self) -> Result<Promise, JsValue> {
    let input_data = self.builder.clone();
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      let mut sender = client.client.message();
      // if let Some(seed) = self.builder.unwrap().seed {
      //     sender = sender.with_seed(Seed::from_bytes(&hex::decode(&seed).expect("invalid seed hex")));
      // }
      if let Some(index) = input_data.index {
        sender = sender.with_index(index);
      }
      if let Some(data) = input_data.data {
        sender = sender.with_data(data.clone());
      }
      // if let Some(parents) = parents {
      //     sender = sender.with_parents(parents.clone())?;
      // }
      // if let Some(account_index) = account_index {
      //     sender = sender.with_account_index(*account_index);
      // }
      // if let Some(initial_address_index) = initial_address_index {
      //     sender = sender.with_initial_address_index(*initial_address_index);
      // }
      // for input in inputs {
      //     sender = sender.with_input(input.clone());
      // }
      // if let Some(input_range) = input_range {
      //     sender = sender.with_input_range(input_range.clone());
      // }
      // let bech32_hrp = client.get_bech32_hrp().await.map_err(err)?;
      // for output in outputs {
      //     sender = sender.with_output(&output.0.clone().to_bech32(&bech32_hrp), output.1).map_err(err)?;
      // }
      // for output in dust_allowance_outputs {
      //     sender =
      //         sender.with_dust_allowance_output(&output.0.clone().to_bech32(&bech32_hrp), output.1).map_err(err)?;
      // }
      sender.finish().await.map_err(err).and_then(|message| {
        #[derive(Serialize)]
        struct MessageWrapper {
          #[serde(rename = "messageId")]
          message_id: MessageId,
          message: MessageDto,
        }
        let message_id = message.id().0;
        JsValue::from_serde(&MessageWrapper {
          message_id,
          message: MessageDto::from(&message),
        })
        .map_err(err)
      })
    });

    Ok(promise)
  }
}
