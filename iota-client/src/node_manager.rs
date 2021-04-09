// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests and quroum if enabled

use bee_rest_api::types::responses::InfoResponse as NodeInfo;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, Result};
use log::warn;
use regex::Regex;
#[cfg(feature = "wasm")]
use std::sync::RwLock;
#[cfg(not(feature = "wasm"))]
use tokio::sync::RwLock;
#[cfg(all(feature = "sync", not(feature = "async")))]
use ureq::{Agent, AgentBuilder};
use url::Url;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};

const NODE_SYNC_INTERVAL: Duration = Duration::from_secs(60);
const DEFAULT_QUORUM_SIZE: usize = 3;
const DEFAULT_QUORUM_THRESHOLD: usize = 66;

pub(crate) struct NodeManager {
    pub(crate) primary_node: Option<Url>,
    primary_pow_node: Option<Url>,
    pub(crate) nodes: HashSet<Url>,
    pub(crate) sync: bool,
    sync_interval: Duration,
    // #[cfg(feature = "wasm")]
    // pub(crate) synced_nodes: Arc<HashSet<Url>>,
    pub(crate) synced_nodes: Arc<RwLock<HashSet<Url>>>,
    quorum: bool,
    quorum_size: usize,
    quorum_threshold: usize,
    pub(crate) http_client: HttpClient,
}

impl std::fmt::Debug for NodeManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("NodeManager");
        d.field("primary_node", &self.primary_node);
        d.field("primary_pow_node", &self.primary_pow_node);
        d.field("nodes", &self.nodes);
        d.field("sync", &self.sync);
        d.field("sync_interval", &self.sync_interval);
        d.field("synced_nodes", &self.synced_nodes);
        d.field("quorum", &self.quorum);
        d.field("quorum_size", &self.quorum_size);
        d.field("quorum_threshold", &self.quorum_threshold).finish()
    }
}

impl NodeManager {
    pub(crate) fn builder() -> NodeManagerBuilder {
        NodeManagerBuilder::new()
    }
    pub(crate) async fn get_urls(&self, path: &str, query: Option<&str>, remote_pow: bool) -> Vec<Url> {
        let mut urls = Vec::new();
        if remote_pow {
            if let Some(mut pow_node) = self.primary_pow_node.clone() {
                pow_node.set_path(path);
                pow_node.set_query(query);
                urls.push(pow_node);
            }
        }
        if let Some(mut primary_node) = self.primary_node.clone() {
            primary_node.set_path(path);
            primary_node.set_query(query);
            urls.push(primary_node);
        }
        let nodes = if self.sync {
            #[cfg(not(feature = "wasm"))]
            {
                self.synced_nodes.read().await.clone()
            }
            #[cfg(feature = "wasm")]
            {
                self.nodes.clone()
            }
        } else {
            self.nodes.clone()
        };
        for mut url in nodes {
            url.set_path(path);
            url.set_query(query);
            urls.push(url);
        }
        urls
    }

    pub(crate) async fn get_request<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: Option<&str>,
        timeout: Duration,
    ) -> Result<T> {
        // Endpoints for which quorum will be used if enabled
        let quorum_regexes = lazy_static!(
            [
              Regex::new(r"messages/([A-Fa-f0-9]{64})/metadata").expect("regex failed"),
              Regex::new(r"outputs/([A-Fa-f0-9]{64})(\d{4})").expect("regex failed"),
              // bech32 address
              Regex::new("addresses/(iota|atoi|iot|toi)1[A-Za-z0-9]").expect("regex failed"),
              Regex::new("addresses/(iota|atoi|iot|toi)1[A-Za-z0-9]+/outputs").expect("regex failed"),
              // ED25519 address hex
              Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})").expect("regex failed"),
              Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})/outputs").expect("regex failed"),
              Regex::new(r"transactions/([A-Fa-f0-9]{64})/included-message").expect("regex failed"),
            ].to_vec() => Vec<Regex>
        );
        let mut result: HashMap<String, usize> = HashMap::new();
        // submit message with local PoW should use primary pow node
        // Get urls and set path
        let urls = self.get_urls(path, query, false).await;
        if self.quorum && quorum_regexes.iter().any(|re| re.is_match(&path)) && urls.len() < self.quorum_size {
            return Err(Error::QuorumPoolSizeError(urls.len(), self.quorum_size));
        }

        // Track amount of results for quorum
        let mut result_counter = 0;
        let mut error = None;
        // Send requests parallel for quorum
        #[cfg(feature = "wasm")]
        let wasm = true;
        #[cfg(not(feature = "wasm"))]
        let wasm = false;
        #[cfg(not(feature = "wasm"))]
        if !wasm && self.quorum && quorum_regexes.iter().any(|re| re.is_match(&path)) && query.is_none() {
            let mut tasks = Vec::new();
            let urls_ = urls.clone();
            for (index, url) in urls_.into_iter().enumerate() {
                if index < self.quorum_size {
                    let client_ = self.http_client.clone();
                    tasks.push(
                        async move { tokio::spawn(async move { client_.get(url.as_str(), timeout).await }).await },
                    );
                }
            }
            for res in futures::future::try_join_all(tasks)
                .await
                .expect("failed to sync address")
            {
                match res {
                    Ok(res) => {
                        if let Ok(res_text) = res.text().await {
                            let counters = result.entry(res_text).or_insert(0);
                            *counters += 1;
                            result_counter += 1;
                        } else {
                            warn!("Couldn't convert noderesult to text");
                        }
                    }
                    Err(err) => {
                        error.replace(err);
                    }
                }
            }
        } else {
            // Send requests
            for url in urls {
                match self.http_client.get(url.as_str(), timeout).await {
                    Ok(res) => {
                        if let Ok(res_text) = res.text().await {
                            // Handle nodeinfo extra because we also want to return the url
                            if path == "api/v1/info" {
                                #[derive(Debug, Serialize, Deserialize)]
                                struct ResponseWrapper {
                                    data: NodeInfo,
                                }
                                let wrapper = crate::client::NodeInfoWrapper {
                                    nodeinfo: serde_json::from_str::<ResponseWrapper>(&res_text)?.data,
                                    url: format!("{}://{}", url.scheme(), url.host_str().unwrap_or("")),
                                };
                                let serde_res = serde_json::to_string(&wrapper)?;
                                return Ok(serde_json::from_str(&serde_res)?);
                            }
                            let counters = result.entry(res_text).or_insert(0);
                            *counters += 1;
                            result_counter += 1;
                            // Without quorum it's enough if we got one response
                            if !self.quorum
                            || result_counter >= self.quorum_size
                            || !quorum_regexes.iter().any(|re| re.is_match(&path)) 
                            // with query we ignore quorum because the nodes can store a different amount of history
                            || query.is_some()
                            {
                                break;
                            }
                        } else {
                            warn!("Couldn't convert noderesult to text");
                        }
                    }
                    Err(err) => {
                        error.replace(err);
                    }
                }
            }
        }

        let res = result
            .into_iter()
            .max_by_key(|v| v.1)
            .ok_or_else(|| error.unwrap_or(Error::NodeError))?;

        // todo if quorum then only for: balance, outputs(only unspent?), message metadata
        // Return if quorum is false or check if quorum was reached
        if !self.quorum
            || res.1 as f64 >= self.quorum_size as f64 * (self.quorum_threshold as f64 / 100.0)
            || !quorum_regexes.iter().any(|re| re.is_match(&path))
            // with query we ignore quorum because the nodes can store a different amount of history
            || query.is_some()
        {
            Ok(serde_json::from_str(&res.0)?)
        } else {
            Err(Error::QuorumThresholdError(res.1, self.quorum_size))
        }
    }
    // Only used for api/v1/messages/{messageID}/raw, that's why we don't need the quorum stuff
    pub(crate) async fn get_request_text(&self, path: &str, query: Option<&str>, timeout: Duration) -> Result<String> {
        // Get urls and set path
        let urls = self.get_urls(path, query, false).await;
        // Send requests
        for url in urls {
            if let Ok(res) = self.http_client.get(url.as_str(), timeout).await {
                if let Ok(res_text) = res.text().await {
                    // Without quorum it's enough if we got one response
                    return Ok(res_text);
                } else {
                    warn!("Couldn't convert noderesult to text");
                }
            }
        }
        Err(Error::NodeError)
    }
    pub(crate) async fn post_request_bytes<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        timeout: Duration,
        body: &[u8],
        remote_pow: bool,
    ) -> Result<T> {
        let urls = self.get_urls(path, None, remote_pow).await;
        // Send requests
        for url in urls {
            if let Ok(res) = self.http_client.post_bytes(url.as_str(), timeout, body).await {
                if let Ok(res_text) = res.text().await {
                    return Ok(serde_json::from_str(&res_text)?);
                } else {
                    warn!("Couldn't convert noderesult to text");
                }
            }
        }
        Err(Error::NodeError)
    }

    pub(crate) async fn post_request_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        timeout: Duration,
        json: Value,
        remote_pow: bool,
    ) -> Result<T> {
        let urls = self.get_urls(path, None, remote_pow).await;
        // Send requests
        for url in urls {
            if let Ok(res) = self.http_client.post_json(url.as_str(), timeout, json.clone()).await {
                if let Ok(res_text) = res.text().await {
                    return Ok(serde_json::from_str(&res_text)?);
                } else {
                    warn!("Couldn't convert noderesult to text");
                }
            }
        }
        Err(Error::NodeError)
    }
}

pub(crate) struct NodeManagerBuilder {
    primary_node: Option<Url>,
    primary_pow_node: Option<Url>,
    nodes: HashSet<Url>,
    sync: bool,
    sync_interval: Duration,
    quorum: bool,
    quorum_size: usize,
    quorum_threshold: usize,
}

impl NodeManagerBuilder {
    pub(crate) fn new() -> Self {
        Default::default()
    }
    pub(crate) fn with_node(mut self, url: &str) -> Result<Self> {
        let url = validate_url(Url::parse(url)?)?;
        self.nodes.insert(url);
        Ok(self)
    }
    pub(crate) fn with_primary_node(mut self, url: &str, auth_name_passw: Option<(&str, &str)>) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some((name, password)) = auth_name_passw {
            url.set_username(name)
                .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
            url.set_password(Some(password))
                .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
        }
        self.primary_node.replace(url);
        Ok(self)
    }
    pub(crate) fn with_primary_pow_node(mut self, url: &str, auth_name_passw: Option<(&str, &str)>) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some((name, password)) = auth_name_passw {
            url.set_username(name)
                .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
            url.set_password(Some(password))
                .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
        }
        self.primary_pow_node.replace(url);
        Ok(self)
    }
    pub(crate) fn with_node_sync_disabled(mut self) -> Self {
        self.sync = false;
        self
    }
    pub(crate) fn with_node_auth(mut self, url: &str, name: &str, password: &str) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        url.set_username(name)
            .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
        url.set_password(Some(password))
            .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
        self.nodes.insert(url);
        Ok(self)
    }
    pub(crate) fn with_nodes(mut self, urls: &[&str]) -> Result<Self> {
        for url in urls {
            let url = validate_url(Url::parse(url)?)?;
            self.nodes.insert(url);
        }
        Ok(self)
    }
    /// Get node list from the node_pool_urls
    pub(crate) async fn with_node_pool_urls(mut self, node_pool_urls: &[String]) -> Result<Self> {
        for pool_url in node_pool_urls {
            let nodes_details: Vec<NodeDetail> = crate::node_manager::HttpClient::new()
                .get(&pool_url, crate::builder::GET_API_TIMEOUT)
                .await?
                .json()
                .await?;
            for node_detail in nodes_details {
                let url = validate_url(Url::parse(&node_detail.node)?)?;
                self.nodes.insert(url);
            }
        }
        Ok(self)
    }
    pub(crate) fn with_node_sync_interval(mut self, node_sync_interval: Duration) -> Self {
        self.sync_interval = node_sync_interval;
        self
    }
    pub(crate) fn with_quorum(mut self, quorum: bool) -> Self {
        self.quorum = quorum;
        self
    }
    pub(crate) fn with_quorum_size(mut self, quorum_size: usize) -> Self {
        self.quorum_size = quorum_size;
        self
    }
    pub(crate) fn with_quorum_threshold(mut self, threshold: usize) -> Self {
        self.quorum_threshold = threshold;
        self
    }
    pub(crate) async fn build(
        mut self,
        network_info: crate::builder::NetworkInfo,
        synced_nodes: Arc<RwLock<HashSet<Url>>>,
    ) -> Result<NodeManager> {
        let default_testnet_node_pools = vec!["https://giftiota.com/nodes.json".to_string()];
        if self.nodes.is_empty() && self.primary_node.is_none() {
            match network_info.network {
                Some(ref network) => match network.to_lowercase().as_str() {
                    "testnet" | "devnet" | "test" | "dev" => {
                        self = self.with_node_pool_urls(&default_testnet_node_pools[..]).await?;
                    }
                    _ => return Err(Error::SyncedNodePoolEmpty),
                },
                _ => {
                    self = self.with_node_pool_urls(&default_testnet_node_pools[..]).await?;
                }
            }
        }

        // Return error if we don't have a node
        if self.nodes.is_empty() && self.primary_node.is_none() {
            return Err(Error::MissingParameter("Node"));
        }

        let node_manager = NodeManager {
            primary_node: self.primary_node,
            primary_pow_node: self.primary_pow_node,
            nodes: self.nodes,
            sync: self.sync,
            sync_interval: self.sync_interval,
            synced_nodes,
            quorum: self.quorum,
            quorum_size: self.quorum_size,
            quorum_threshold: self.quorum_threshold,
            http_client: HttpClient::new(),
        };
        Ok(node_manager)
    }
}

impl Default for NodeManagerBuilder {
    fn default() -> Self {
        Self {
            primary_node: None,
            primary_pow_node: None,
            nodes: HashSet::new(),
            sync: false,
            sync_interval: NODE_SYNC_INTERVAL,
            quorum: false,
            quorum_size: DEFAULT_QUORUM_SIZE,
            quorum_threshold: DEFAULT_QUORUM_THRESHOLD,
        }
    }
}

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

#[cfg(feature = "async")]
pub(crate) struct Response(reqwest::Response);

#[cfg(feature = "async")]
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

#[cfg(feature = "async")]
#[derive(Clone)]
pub(crate) struct HttpClient {
    client: reqwest::Client,
}

#[cfg(all(feature = "sync", not(feature = "async")))]
pub(crate) struct HttpClient;

#[cfg(feature = "async")]
impl HttpClient {
    pub(crate) fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn parse_response(response: reqwest::Response) -> Result<Response> {
        let status = response.status();
        if status.is_success() {
            Ok(Response(response))
        } else {
            Err(Error::ResponseError(status.as_u16(), response.text().await?))
        }
    }

    pub(crate) async fn get(&self, url: &str, timeout: Duration) -> Result<Response> {
        let mut request_builder = self.client.get(url);
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(timeout)
        }
        let response = request_builder.send().await?;
        Self::parse_response(response).await
    }

    pub(crate) async fn post_bytes(&self, url: &str, timeout: Duration, body: &[u8]) -> Result<Response> {
        let mut request_builder = self.client.post(url);
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(timeout)
        }
        let response = request_builder.body(body.to_vec()).send().await?;
        Self::parse_response(response).await
    }

    pub(crate) async fn post_json(&self, url: &str, timeout: Duration, json: Value) -> Result<Response> {
        let mut request_builder = self.client.post(url);
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(timeout)
        }
        let response = request_builder.json(&json).send().await?;
        Self::parse_response(response).await
    }
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl HttpClient {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn get(&self, url: &str, timeout: Duration) -> Result<Response> {
        Ok(Self::get_ureq_agent(timeout).get(url).call()?.into())
    }

    pub(crate) async fn post_bytes(&self, url: &str, timeout: Duration, body: &[u8]) -> Result<Response> {
        Ok(Self::get_ureq_agent(timeout).post(url).send_bytes(body)?.into())
    }

    pub(crate) async fn post_json(&self, url: &str, timeout: Duration, json: Value) -> Result<Response> {
        Ok(Self::get_ureq_agent(timeout).post(url).send_json(json)?.into())
    }

    fn get_ureq_agent(timeout: Duration) -> Agent {
        AgentBuilder::new().timeout_read(timeout).timeout_write(timeout).build()
    }
}

fn validate_url(url: Url) -> Result<Url> {
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(Error::UrlValidationError(format!("Invalid scheme: {}", url.scheme())));
    }
    Ok(url)
}

/// JSON struct for NodeDetail from the node_pool_urls
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDetail {
    /// Iota node url
    pub node: String,
    /// Network id
    pub network_id: String,
    /// Implementation name
    pub implementation: String,
    /// Enabled PoW
    pub pow: bool,
}
