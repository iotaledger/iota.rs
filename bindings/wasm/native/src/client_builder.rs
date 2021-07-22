// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::Client;
use crate::utils::err;
use futures::executor;
use iota_client::ClientBuilder as RustClientBuilder;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;

fn to_duration(seconds: u32) -> Duration {
  Duration::from_secs(u64::from(seconds))
}

fn to_basic_auth<'a>(username: &'a Option<String>, password: &'a Option<String>) -> Option<(&'a str, &'a str)> {
  username.as_deref().zip(password.as_deref())
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ClientBuilder {
  pub(crate) builder: Option<RustClientBuilder>,
}

#[wasm_bindgen]
impl ClientBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      builder: Some(RustClientBuilder::new()),
    }
  }

  #[wasm_bindgen]
  pub fn node(&mut self, url: &str) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder: RustClientBuilder| builder.with_node(url).map_err(err))?;
    // is there a way we can do it without the clone?
    Ok(self.clone())
  }

  #[wasm_bindgen(js_name = primaryNode)]
  pub fn primary_node(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder| {
      builder
        .with_primary_node(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(err)
    })?;
    Ok(self.clone())
  }

  #[wasm_bindgen(js_name = primaryPowNode)]
  pub fn primary_pow_node(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder| {
      builder
        .with_primary_pow_node(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(err)
    })?;
    Ok(self.clone())
  }

  #[wasm_bindgen]
  pub fn permanode(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder| {
      builder
        .with_permanode(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(err)
    })?;
    Ok(self.clone())
  }

  #[wasm_bindgen(js_name = nodeAuth)]
  pub fn node_auth(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder| {
      builder
        .with_node_auth(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(err)
    })?;
    Ok(self.clone())
  }

  #[wasm_bindgen(js_name = nodeSyncInterval)]
  pub fn node_sync_interval(&mut self, value: u32) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_node_sync_interval(to_duration(value)))?;
    Ok(self.clone())
  }

  #[wasm_bindgen(js_name = nodeSyncDisabled)]
  pub fn node_sync_disabled(&mut self) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_node_sync_disabled())?;
    Ok(self.clone())
  }

  #[wasm_bindgen(js_name = quorum)]
  pub fn quorum(&mut self, value: bool) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.with_quorum(value))
  }

  #[wasm_bindgen(js_name = quorumSize)]
  pub fn quorum_size(&mut self, value: usize) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.with_quorum_size(value))
  }

  #[wasm_bindgen(js_name = quorumThreshold)]
  pub fn quorum_threshold(&mut self, value: usize) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.with_quorum_threshold(value))
  }

  // We currently don't support local PoW with wasm
  // #[wasm_bindgen(js_name = localPoW)]
  // pub fn local_pow(&mut self, value: bool) -> Result<(), JsValue> {
  //   self.with_mut(|builder| builder.with_local_pow(value))
  // }

  #[wasm_bindgen(js_name = tipsInterval)]
  pub fn tips_interval(&mut self, value: u32) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.with_tips_interval(u64::from(value)))
  }

  #[wasm_bindgen(js_name = requestTimeout)]
  pub fn request_timeout(&mut self, value: u32) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.with_request_timeout(to_duration(value)))
  }

  pub(crate) fn take_builder(&mut self) -> Result<RustClientBuilder, JsValue> {
    self.builder.take().ok_or_else(|| "Client Builder Consumed".into())
  }

  fn with_mut(&mut self, f: impl Fn(RustClientBuilder) -> RustClientBuilder) -> Result<(), JsValue> {
    self.builder = Some(f(self.take_builder()?));
    Ok(())
  }

  fn try_with_mut(
    &mut self,
    f: impl Fn(RustClientBuilder) -> Result<RustClientBuilder, JsValue>,
  ) -> Result<(), JsValue> {
    self.builder = Some(f(self.take_builder()?)?);
    Ok(())
  }

  /// Build the client.
  #[wasm_bindgen]
  pub fn build(&mut self) -> Result<Client, JsValue> {
    let future = self.take_builder()?.finish();
    let output = executor::block_on(future).map_err(err)?;

    Ok(Client {
      client: Rc::new(output),
    })
  }
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self::new()
  }
}
