// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{client::Client, error::wasm_error, MessageWrapper};
use iota_client::{
  bee_message::{address::Address, input::UtxoInput, MessageId},
  bee_rest_api::types::dtos::MessageDto,
  Seed,
};
use js_sys::Promise;
use std::{ops::Range, str::FromStr};
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

  /// Set indexation to the builder
  #[wasm_bindgen]
  pub fn index(&mut self, index: Vec<u8>) -> Result<MessageBuilder, JsValue> {
    self.builder.index.replace(index);
    // is there a way we can do it without the clone?
    Ok(self.clone())
  }

  /// Set data to the builder
  #[wasm_bindgen]
  pub fn data(&mut self, data: Vec<u8>) -> Result<MessageBuilder, JsValue> {
    self.builder.data.replace(data);
    Ok(self.clone())
  }

  /// Sets the seed.
  #[wasm_bindgen]
  pub fn seed(&mut self, seed: &str) -> Result<MessageBuilder, JsValue> {
    self.builder.seed.replace(seed.into());
    Ok(self.clone())
  }

  /// Sets the account index.
  #[wasm_bindgen(js_name = accountIndex)]
  pub fn account_index(&mut self, account_index: usize) -> Result<MessageBuilder, JsValue> {
    self.builder.account_index.replace(account_index);
    Ok(self.clone())
  }

  /// Sets the index of the address to start looking for balance.
  #[wasm_bindgen(js_name = initialAddressIndex)]
  pub fn initial_address_index(&mut self, initial_address_index: usize) -> Result<MessageBuilder, JsValue> {
    self.builder.initial_address_index.replace(initial_address_index);
    Ok(self.clone())
  }

  /// Set 1-8 custom parent message ids
  #[wasm_bindgen]
  pub fn parents(&mut self, parents: JsValue) -> Result<MessageBuilder, JsValue> {
    let parents: Vec<String> = parents.into_serde().map_err(wasm_error)?;
    let parents = parents
      .iter()
      .map(|message_id| MessageId::from_str(message_id))
      .collect::<Result<Vec<MessageId>, iota_client::bee_message::Error>>()
      .map_err(wasm_error)?;
    self.builder.parents.replace(parents);
    Ok(self.clone())
  }

  /// Set a custom input(transaction output)
  #[wasm_bindgen]
  pub fn input(&mut self, output_id: &str) -> Result<MessageBuilder, JsValue> {
    self
      .builder
      .inputs
      .push(UtxoInput::from_str(output_id).map_err(wasm_error)?);
    Ok(self.clone())
  }

  /// Set a custom range in which to search for addresses for custom provided inputs. Default: 0..100
  #[wasm_bindgen(js_name = inputRange)]
  pub fn input_range(&mut self, start: usize, end: usize) -> Result<MessageBuilder, JsValue> {
    self.builder.input_range.replace(start..end);
    Ok(self.clone())
  }

  /// Set a transfer to the builder
  #[wasm_bindgen]
  pub fn output(&mut self, address: &str, amount: u64) -> Result<MessageBuilder, JsValue> {
    self
      .builder
      .outputs
      .push((Address::try_from_bech32(address).map_err(wasm_error)?, amount));
    Ok(self.clone())
  }

  /// Set a dust allowance transfer to the builder, address needs to be Bech32 encoded
  #[wasm_bindgen(js_name = dustAllowanceOutput)]
  pub fn dust_allowance_output(&mut self, address: &str, amount: u64) -> Result<MessageBuilder, JsValue> {
    self
      .builder
      .dust_allowance_outputs
      .push((Address::try_from_bech32(address).map_err(wasm_error)?, amount));
    Ok(self.clone())
  }

  /// Prepare a transaction
  #[wasm_bindgen(js_name = prepareTransaction)]
  pub fn prepare_transaction(&self) -> Result<Promise, JsValue> {
    let input_data = self.builder.clone();
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      let mut sender = client.client.message();
      if let Some(index) = input_data.index {
        sender = sender.with_index(index);
      }
      if let Some(data) = input_data.data {
        sender = sender.with_data(data);
      }
      if let Some(parents) = input_data.parents {
        sender = sender.with_parents(parents).map_err(wasm_error)?;
      }
      if let Some(account_index) = input_data.account_index {
        sender = sender.with_account_index(account_index);
      }
      if let Some(initial_address_index) = input_data.initial_address_index {
        sender = sender.with_initial_address_index(initial_address_index);
      }
      for input in input_data.inputs {
        sender = sender.with_input(input.clone());
      }
      if let Some(input_range) = input_data.input_range {
        sender = sender.with_input_range(input_range);
      }
      let bech32_hrp = client.client.get_bech32_hrp().await.map_err(wasm_error)?;
      for output in input_data.outputs {
        sender = sender
          .with_output(&output.0.clone().to_bech32(&bech32_hrp), output.1)
          .map_err(wasm_error)?;
      }
      for output in input_data.dust_allowance_outputs {
        sender = sender
          .with_dust_allowance_output(&output.0.clone().to_bech32(&bech32_hrp), output.1)
          .map_err(wasm_error)?;
      }
      let sender_future = if let Some(seed) = input_data.seed {
        let seed = Seed::from_bytes(&hex::decode(&seed).map_err(wasm_error)?);
        sender.with_seed(&seed).prepare_transaction().await
      } else {
        sender.prepare_transaction().await
      };
      sender_future
        .map_err(wasm_error)
        .and_then(|transaction| JsValue::from_serde(&transaction).map_err(wasm_error))
    });

    Ok(promise)
  }

  /// Sign a transaction
  #[wasm_bindgen(js_name = signTransaction)]
  pub fn sign_transaction(
    &self,
    prepared_transaction_data: JsValue,
    seed: String,
    input_range_start: Option<usize>,
    input_range_end: Option<usize>,
  ) -> Result<Promise, JsValue> {
    let client = self.client.clone();
    let range = input_range_start.unwrap_or(0)..input_range_end.unwrap_or(100);
    let promise: Promise = future_to_promise(async move {
      let seed = Seed::from_bytes(&hex::decode(&seed).map_err(wasm_error)?);
      client
        .client
        .message()
        .sign_transaction(
          prepared_transaction_data.into_serde().map_err(wasm_error)?,
          Some(&seed),
          Some(range),
        )
        .await
        .map_err(wasm_error)
        .and_then(|transaction| JsValue::from_serde(&transaction).map_err(wasm_error))
    });

    Ok(promise)
  }

  /// Create a message with a provided payload
  #[wasm_bindgen(js_name = finishMessage)]
  pub fn finish_message(&self, payload: JsValue) -> Result<Promise, JsValue> {
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .client
        .message()
        .finish_message(Some(payload.into_serde().map_err(wasm_error)?))
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

  /// Build and sumbit the message.
  #[wasm_bindgen]
  pub fn submit(&self) -> Result<Promise, JsValue> {
    let input_data = self.builder.clone();
    let client = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      let mut sender = client.client.message();
      if let Some(index) = input_data.index {
        sender = sender.with_index(index);
      }
      if let Some(data) = input_data.data {
        sender = sender.with_data(data);
      }
      if let Some(parents) = input_data.parents {
        sender = sender.with_parents(parents).map_err(wasm_error)?;
      }
      if let Some(account_index) = input_data.account_index {
        sender = sender.with_account_index(account_index);
      }
      if let Some(initial_address_index) = input_data.initial_address_index {
        sender = sender.with_initial_address_index(initial_address_index);
      }
      for input in input_data.inputs {
        sender = sender.with_input(input.clone());
      }
      if let Some(input_range) = input_data.input_range {
        sender = sender.with_input_range(input_range);
      }
      let bech32_hrp = client.client.get_bech32_hrp().await.map_err(wasm_error)?;
      for output in input_data.outputs {
        sender = sender
          .with_output(&output.0.clone().to_bech32(&bech32_hrp), output.1)
          .map_err(wasm_error)?;
      }
      for output in input_data.dust_allowance_outputs {
        sender = sender
          .with_dust_allowance_output(&output.0.clone().to_bech32(&bech32_hrp), output.1)
          .map_err(wasm_error)?;
      }
      let sender_future = if let Some(seed) = input_data.seed {
        let seed = Seed::from_bytes(&hex::decode(&seed).map_err(wasm_error)?);
        sender.with_seed(&seed).finish().await
      } else {
        sender.finish().await
      };
      sender_future.map_err(wasm_error).and_then(|message| {
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
}
