// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests with healthy nodes and quorum if enabled

use std::time::Duration;

use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    error::{Error, Result},
    node_manager::node::Node,
};
pub(crate) struct Response(reqwest::Response);

impl Response {
    pub(crate) fn status(&self) -> u16 {
        self.0.status().as_u16()
    }

    pub(crate) async fn into_json<T: DeserializeOwned>(self) -> Result<T> {
        self.0.json().await.map_err(Into::into)
    }

    pub(crate) async fn into_text(self) -> Result<String> {
        self.0.text().await.map_err(Into::into)
    }

    pub(crate) async fn into_bytes(self) -> Result<Vec<u8>> {
        self.0.bytes().await.map(|b| b.to_vec()).map_err(Into::into)
    }
}

#[derive(Clone)]
pub(crate) struct HttpClient {
    client: reqwest::Client,
    user_agent: String,
}

impl HttpClient {
    pub(crate) fn new(user_agent: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            user_agent,
        }
    }

    async fn parse_response(response: reqwest::Response, url: &url::Url) -> Result<Response> {
        let status = response.status();
        if status.is_success() {
            Ok(Response(response))
        } else {
            Err(Error::ResponseError {
                code: status.as_u16(),
                text: response.text().await?,
                url: url.to_string(),
            })
        }
    }

    fn build_request(&self, request_builder: RequestBuilder, node: &Node, _timeout: Duration) -> RequestBuilder {
        let mut request_builder = request_builder.header(reqwest::header::USER_AGENT, &self.user_agent);

        if let Some(node_auth) = &node.auth {
            if let Some(jwt) = &node_auth.jwt {
                request_builder = request_builder.bearer_auth(jwt);
            }
        }
        #[cfg(not(target_family = "wasm"))]
        {
            request_builder = request_builder.timeout(_timeout);
        }
        request_builder
    }

    pub(crate) async fn get(&self, node: Node, timeout: Duration) -> Result<Response> {
        let mut request_builder = self.client.get(node.url.clone());
        request_builder = self.build_request(request_builder, &node, timeout);
        let start_time = instant::Instant::now();
        let resp = request_builder.send().await?;
        log::debug!(
            "GET: {:?} ms for {} {}",
            start_time.elapsed().as_millis(),
            resp.status(),
            node.url
        );
        Self::parse_response(resp, &node.url).await
    }

    // Get with header: "accept", "application/vnd.iota.serializer-v1"
    pub(crate) async fn get_bytes(&self, node: Node, timeout: Duration) -> Result<Response> {
        let mut request_builder = self.client.get(node.url.clone());
        request_builder = self.build_request(request_builder, &node, timeout);
        request_builder = request_builder.header("accept", "application/vnd.iota.serializer-v1");
        let resp = request_builder.send().await?;
        Self::parse_response(resp, &node.url).await
    }

    pub(crate) async fn post_json(&self, node: Node, timeout: Duration, json: Value) -> Result<Response> {
        let mut request_builder = self.client.post(node.url.clone());
        request_builder = self.build_request(request_builder, &node, timeout);
        Self::parse_response(request_builder.json(&json).send().await?, &node.url).await
    }

    pub(crate) async fn post_bytes(&self, node: Node, timeout: Duration, body: &[u8]) -> Result<Response> {
        let mut request_builder = self.client.post(node.url.clone());
        request_builder = self.build_request(request_builder, &node, timeout);
        request_builder = request_builder.header("Content-Type", "application/vnd.iota.serializer-v1");
        Self::parse_response(request_builder.body(body.to_vec()).send().await?, &node.url).await
    }
}
