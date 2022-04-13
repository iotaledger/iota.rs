// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests with synced nodes and quorum if enabled

use std::time::Duration;

use serde::de::DeserializeOwned;
use serde_json::Value;
#[cfg(all(feature = "sync", not(feature = "async")))]
use ureq::{Agent, AgentBuilder};

use crate::{
    error::{Error, Result},
    node_manager::node::Node,
};

#[cfg(all(feature = "sync", not(feature = "async")))]
pub(crate) struct Response(ureq::Response);

#[cfg(all(feature = "sync", not(feature = "async")))]
impl From<ureq::Response> for Response {
    fn from(response: ureq::Response) -> Self {
        Self(response)
    }
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl Response {
    pub(crate) fn status(&self) -> u16 {
        self.0.status()
    }

    pub(crate) async fn json<T: DeserializeOwned>(self) -> Result<T> {
        self.0.into_json().map_err(Into::into)
    }

    pub(crate) async fn text(self) -> Result<String> {
        self.0.into_string().map_err(Into::into)
    }
}

#[cfg(any(feature = "async", feature = "wasm"))]
pub(crate) struct Response(reqwest::Response);

#[cfg(any(feature = "async", feature = "wasm"))]
impl Response {
    pub(crate) fn status(&self) -> u16 {
        self.0.status().as_u16()
    }

    pub(crate) async fn json<T: DeserializeOwned>(self) -> Result<T> {
        self.0.json().await.map_err(Into::into)
    }

    pub(crate) async fn text(self) -> Result<String> {
        self.0.text().await.map_err(Into::into)
    }
}

#[cfg(any(feature = "async", feature = "wasm"))]
#[derive(Clone)]
pub(crate) struct HttpClient {
    client: reqwest::Client,
}

#[cfg(all(feature = "sync", not(feature = "async")))]
#[derive(Clone)]
pub(crate) struct HttpClient;

#[cfg(any(feature = "async", feature = "wasm"))]
impl HttpClient {
    pub(crate) fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn parse_response(response: reqwest::Response, url: &url::Url) -> Result<Response> {
        let status = response.status();
        if status.is_success() {
            Ok(Response(response))
        } else {
            Err(Error::ResponseError(
                status.as_u16(),
                response.text().await?,
                url.to_string(),
            ))
        }
    }

    pub(crate) async fn get(&self, node: Node, _timeout: Duration) -> Result<Response> {
        let mut request_builder = self.client.get(node.url.clone());
        if let Some(node_auth) = node.auth {
            if let Some(jwt) = node_auth.jwt {
                request_builder = request_builder.bearer_auth(jwt);
            }
        }
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(_timeout);
        }
        let resp = request_builder.send().await?;
        Self::parse_response(resp, &node.url).await
    }

    pub(crate) async fn post_bytes(&self, node: Node, _timeout: Duration, body: &[u8]) -> Result<Response> {
        let mut request_builder = self.client.post(node.url.clone());
        if let Some(node_auth) = node.auth {
            if let Some(jwt) = node_auth.jwt {
                request_builder = request_builder.bearer_auth(jwt);
            }
        }
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(_timeout);
        }
        request_builder = request_builder.header("Content-Type", "application/octet-stream");
        Self::parse_response(request_builder.body(body.to_vec()).send().await?, &node.url).await
    }

    pub(crate) async fn post_json(&self, node: Node, _timeout: Duration, json: Value) -> Result<Response> {
        let mut request_builder = self.client.post(node.url.clone());
        if let Some(node_auth) = node.auth {
            if let Some(jwt) = node_auth.jwt {
                request_builder = request_builder.bearer_auth(jwt);
            }
        }
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(_timeout);
        }
        Self::parse_response(request_builder.json(&json).send().await?, &node.url).await
    }
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl HttpClient {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn clone(&self) -> Self {
        Self {}
    }

    pub(crate) async fn get(&self, node: Node, timeout: Duration) -> Result<Response> {
        let mut request_builder = Self::get_ureq_agent(timeout).get(node.url.as_str());
        if let Some(node_auth) = node.auth {
            if let Some(jwt) = node_auth.jwt {
                request_builder = request_builder.set("Authorization", &format!("Bearer {}", jwt));
            }
        }
        Ok(request_builder.call()?.into())
    }

    pub(crate) async fn post_bytes(&self, node: Node, timeout: Duration, body: &[u8]) -> Result<Response> {
        let mut request_builder = Self::get_ureq_agent(timeout).post(node.url.as_str());
        if let Some(node_auth) = node.auth {
            if let Some(jwt) = node_auth.jwt {
                request_builder = request_builder.set("Authorization", &format!("Bearer {}", jwt));
            }
        }
        request_builder = request_builder.set("Content-Type", "application/octet-stream");
        Ok(request_builder.send_bytes(body)?.into())
    }

    pub(crate) async fn post_json(&self, node: Node, timeout: Duration, json: Value) -> Result<Response> {
        let mut request_builder = Self::get_ureq_agent(timeout).post(node.url.as_str());
        if let Some(node_auth) = node.auth {
            if let Some(jwt) = node_auth.jwt {
                request_builder = request_builder.set("Authorization", &format!("Bearer {}", jwt));
            }
        }
        Ok(request_builder.send_json(json)?.into())
    }

    fn get_ureq_agent(timeout: Duration) -> Agent {
        AgentBuilder::new().timeout_read(timeout).timeout_write(timeout).build()
    }
}
