// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests and quroum if enabled

use bee_rest_api::types::{body::SuccessBody, responses::InfoResponse};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    builder::NetworkInfo,
    error::{Error, Result},
};
use log::warn;
use regex::Regex;
use std::sync::RwLock;
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

/// Node struct
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Node {
    /// node url
    pub url: Url,
    /// node jwt
    pub jwt: Option<String>,
}

// Nodemanger, takes care of selecting node(s) for requests until a result is returned or if quorum
// is enabled it will send the requests for some endpoints to multiple nodes and compares the results
#[derive(Clone)]
pub(crate) struct NodeManager {
    pub(crate) primary_node: Option<Node>,
    primary_pow_node: Option<Node>,
    pub(crate) nodes: HashSet<Node>,
    permanodes: Option<HashSet<Node>>,
    pub(crate) sync: bool,
    sync_interval: Duration,
    pub(crate) synced_nodes: Arc<RwLock<HashSet<Node>>>,
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
        d.field("permanodes", &self.permanodes);
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
    pub(crate) async fn get_nodes(
        &self,
        path: &str,
        query: Option<&str>,
        use_primary_pow_node: bool,
    ) -> Result<Vec<Node>> {
        let mut nodes_with_modified_url = Vec::new();

        // Endpoints for which only permanodes will be used if provided
        let permanode_regexes = lazy_static!(
            [
              Regex::new(r"messages/([A-Fa-f0-9]{64})").expect("regex failed"),
              Regex::new(r"messages/([A-Fa-f0-9]{64})/metadata").expect("regex failed"),
              Regex::new(r"messages/([A-Fa-f0-9]{64})/children").expect("regex failed"),
              Regex::new(r"outputs/([A-Fa-f0-9]{64})(\d{4})").expect("regex failed"),
              // BIP-173 compliant bech32 address
              Regex::new("addresses/[\x21-\x7E]{1,30}1[A-Za-z0-9]").expect("regex failed"),
              Regex::new("addresses/[\x21-\x7E]{1,30}1[A-Za-z0-9]+/outputs").expect("regex failed"),
              // ED25519 address hex
              Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})").expect("regex failed"),
              Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})/outputs").expect("regex failed"),
              Regex::new(r"transactions/([A-Fa-f0-9]{64})/included-message").expect("regex failed"),
              Regex::new(r"milestones/[0-9]").expect("regex failed"),
            ].to_vec() => Vec<Regex>
        );
        if permanode_regexes.iter().any(|re| re.is_match(path)) || (path == "api/v1/messages" && query.is_some()) {
            if let Some(permanodes) = self.permanodes.clone() {
                // remove api/v1/ since permanodes can have custom keyspaces
                // https://editor.swagger.io/?url=https://raw.githubusercontent.com/iotaledger/chronicle.rs/main/docs/api.yaml
                let path = &path["api/v1/".len()..];
                for mut permanode in permanodes {
                    permanode.url.set_path(&format!("{}{}", permanode.url.path(), path));
                    permanode.url.set_query(query);
                    nodes_with_modified_url.push(permanode);
                }
            }
        }

        if use_primary_pow_node {
            if let Some(mut pow_node) = self.primary_pow_node.clone() {
                pow_node.url.set_path(path);
                pow_node.url.set_query(query);
                nodes_with_modified_url.push(pow_node);
            }
        }
        if let Some(mut primary_node) = self.primary_node.clone() {
            primary_node.url.set_path(path);
            primary_node.url.set_query(query);
            nodes_with_modified_url.push(primary_node);
        }
        let nodes = if self.sync {
            #[cfg(not(feature = "wasm"))]
            {
                self.synced_nodes
                    .read()
                    .map_err(|_| crate::Error::NodeReadError)?
                    .clone()
            }
            #[cfg(feature = "wasm")]
            {
                self.nodes.clone()
            }
        } else {
            self.nodes.clone()
        };
        for mut node in nodes {
            node.url.set_path(path);
            node.url.set_query(query);
            nodes_with_modified_url.push(node);
        }
        Ok(nodes_with_modified_url)
    }

    pub(crate) async fn get_request<T: serde::de::DeserializeOwned + std::fmt::Debug + serde::Serialize>(
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
              // BIP-173 compliant bech32 address
              Regex::new("addresses/[\x21-\x7E]{1,30}1[A-Za-z0-9]").expect("regex failed"),
              Regex::new("addresses/[\x21-\x7E]{1,30}1[A-Za-z0-9]+/outputs").expect("regex failed"),
              // ED25519 address hex
              Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})").expect("regex failed"),
              Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})/outputs").expect("regex failed"),
              Regex::new(r"transactions/([A-Fa-f0-9]{64})/included-message").expect("regex failed"),
            ].to_vec() => Vec<Regex>
        );
        let mut result: HashMap<String, usize> = HashMap::new();
        // submit message with local PoW should use primary pow node
        // Get node urls and set path
        let nodes = self.get_nodes(path, query, false).await?;
        if self.quorum && quorum_regexes.iter().any(|re| re.is_match(path)) && nodes.len() < self.quorum_size {
            return Err(Error::QuorumPoolSizeError(nodes.len(), self.quorum_size));
        }

        // Track amount of results for quorum
        let mut result_counter = 0;
        let mut error = None;
        // Send requests parallel for quorum
        #[cfg(feature = "wasm")]
        let wasm = true;
        #[cfg(not(feature = "wasm"))]
        let wasm = false;
        if !wasm && self.quorum && quorum_regexes.iter().any(|re| re.is_match(path)) && query.is_none() {
            #[cfg(not(feature = "wasm"))]
            {
                let mut tasks = Vec::new();
                let nodes_ = nodes.clone();
                for (index, node) in nodes_.into_iter().enumerate() {
                    if index < self.quorum_size {
                        let client_ = self.http_client.clone();
                        tasks.push(async move { tokio::spawn(async move { client_.get(node, timeout).await }).await });
                    }
                }
                for res in futures::future::try_join_all(tasks).await? {
                    match res {
                        Ok(res) => {
                            if let Ok(res_text) = res.text().await {
                                let counters = result.entry(res_text.to_string()).or_insert(0);
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
            }
        } else {
            // Send requests
            for node in nodes {
                match self.http_client.get(node.clone(), timeout).await {
                    Ok(res) => {
                        let status = res.status();
                        if let Ok(res_text) = res.text().await {
                            match status {
                                200 => {
                                    // Handle nodeinfo extra because we also want to return the url
                                    if path == "api/v1/info" {
                                        if let Ok(nodeinfo) =
                                            serde_json::from_str::<SuccessBody<InfoResponse>>(&res_text)
                                        {
                                            let wrapper = crate::client::NodeInfoWrapper {
                                                nodeinfo: nodeinfo.data,
                                                url: format!(
                                                    "{}://{}",
                                                    node.url.scheme(),
                                                    node.url.host_str().unwrap_or("")
                                                ),
                                            };
                                            let serde_res = serde_json::to_string(&wrapper)?;
                                            return Ok(serde_json::from_str(&serde_res)?);
                                        }
                                    }

                                    match serde_json::from_str::<T>(&res_text) {
                                        Ok(result_data) => {
                                            let counters =
                                                result.entry(serde_json::to_string(&result_data)?).or_insert(0);
                                            *counters += 1;
                                            result_counter += 1;
                                            // Without quorum it's enough if we got one response
                                            if !self.quorum
                                            || result_counter >= self.quorum_size
                                            || !quorum_regexes.iter().any(|re| re.is_match(path))
                                            // with query we ignore quorum because the nodes can store a different amount of history
                                            || query.is_some()
                                            {
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            error.replace(e.into());
                                        }
                                    }
                                }
                                _ => {
                                    error.replace(crate::Error::NodeError(res_text));
                                }
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
            .ok_or_else(|| error.unwrap_or_else(|| Error::NodeError("Couldn't get a result from any node".into())))?;

        // Return if quorum is false or check if quorum was reached
        if !self.quorum
            || res.1 as f64 >= self.quorum_size as f64 * (self.quorum_threshold as f64 / 100.0)
            || !quorum_regexes.iter().any(|re| re.is_match(path))
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
        // Get node urls and set path
        let nodes = self.get_nodes(path, query, false).await?;
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.get(node, timeout).await {
                Ok(res) => {
                    let status = res.status();
                    if let Ok(res_text) = res.text().await {
                        // Without quorum it's enough if we got one response
                        match status {
                            200 => return Ok(res_text),
                            _ => error.replace(crate::Error::NodeError(res_text)),
                        };
                    }
                }
                Err(e) => {
                    error.replace(crate::Error::NodeError(e.to_string()));
                }
            }
        }
        Err(error.unwrap_or_else(|| Error::NodeError("Couldn't get a result from any node".into())))
    }
    pub(crate) async fn post_request_bytes<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        timeout: Duration,
        body: &[u8],
        local_pow: bool,
    ) -> Result<T> {
        // primary_pow_node should only be used when remote PoW is used
        let nodes = self.get_nodes(path, None, !local_pow).await?;
        if nodes.is_empty() {
            return Err(Error::NodeError("No available nodes with remote PoW".into()));
        }
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.post_bytes(node, timeout, body).await {
                Ok(res) => {
                    let status = res.status();
                    if let Ok(res_text) = res.text().await {
                        match status {
                            200 | 201 => match serde_json::from_str(&res_text) {
                                Ok(res) => return Ok(res),
                                Err(e) => error.replace(e.into()),
                            },
                            _ => error.replace(crate::Error::NodeError(res_text)),
                        };
                    }
                }
                Err(e) => {
                    error.replace(crate::Error::NodeError(e.to_string()));
                }
            }
        }
        Err(error.unwrap_or_else(|| Error::NodeError("Couldn't get a result from any node".into())))
    }

    pub(crate) async fn post_request_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        timeout: Duration,
        json: Value,
        local_pow: bool,
    ) -> Result<T> {
        // primary_pow_node should only be used when remote PoW is used
        let nodes = self.get_nodes(path, None, !local_pow).await?;
        if nodes.is_empty() {
            return Err(Error::NodeError("No available nodes with remote PoW".into()));
        }
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.post_json(node, timeout, json.clone()).await {
                Ok(res) => {
                    let status = res.status();
                    if let Ok(res_text) = res.text().await {
                        match status {
                            200 | 201 => match serde_json::from_str(&res_text) {
                                Ok(res) => return Ok(res),
                                Err(e) => error.replace(e.into()),
                            },
                            _ => error.replace(crate::Error::NodeError(res_text)),
                        };
                    }
                }
                Err(e) => {
                    error.replace(crate::Error::NodeError(e.to_string()));
                }
            }
        }
        Err(error.unwrap_or_else(|| Error::NodeError("Couldn't get a result from any node".into())))
    }
}

#[derive(Clone)]
pub(crate) struct NodeManagerBuilder {
    pub(crate) primary_node: Option<Node>,
    primary_pow_node: Option<Node>,
    pub(crate) nodes: HashSet<Node>,
    pub(crate) permanodes: Option<HashSet<Node>>,
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
        self.nodes.insert(Node { url, jwt: None });
        Ok(self)
    }
    pub(crate) fn with_primary_node(
        mut self,
        url: &str,
        jwt: Option<String>,
        basic_auth_name_pwd: Option<(&str, &str)>,
    ) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some((name, password)) = basic_auth_name_pwd {
            url.set_username(name)
                .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
            url.set_password(Some(password))
                .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
        }
        self.primary_node.replace(Node { url, jwt });
        Ok(self)
    }
    pub(crate) fn with_primary_pow_node(
        mut self,
        url: &str,
        jwt: Option<String>,
        basic_auth_name_pwd: Option<(&str, &str)>,
    ) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some((name, password)) = basic_auth_name_pwd {
            url.set_username(name)
                .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
            url.set_password(Some(password))
                .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
        }
        self.primary_pow_node.replace(Node { url, jwt });
        Ok(self)
    }
    pub(crate) fn with_permanode(
        mut self,
        url: &str,
        jwt: Option<String>,
        basic_auth_name_pwd: Option<(&str, &str)>,
    ) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some((name, password)) = basic_auth_name_pwd {
            url.set_username(name)
                .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
            url.set_password(Some(password))
                .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
        }
        match self.permanodes {
            Some(ref mut permanodes) => {
                permanodes.insert(Node { url, jwt });
            }
            None => {
                let mut permanodes = HashSet::new();
                permanodes.insert(Node { url, jwt });
                self.permanodes.replace(permanodes);
            }
        }
        Ok(self)
    }
    pub(crate) fn with_node_sync_disabled(mut self) -> Self {
        self.sync = false;
        self
    }
    pub(crate) fn with_node_auth(
        mut self,
        url: &str,
        jwt: Option<String>,
        basic_auth_name_pwd: Option<(&str, &str)>,
    ) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some((name, password)) = basic_auth_name_pwd {
            url.set_username(name)
                .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
            url.set_password(Some(password))
                .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
        }
        self.nodes.insert(Node { url, jwt });
        Ok(self)
    }
    pub(crate) fn with_nodes(mut self, urls: &[&str]) -> Result<Self> {
        for url in urls {
            let url = validate_url(Url::parse(url)?)?;
            self.nodes.insert(Node { url, jwt: None });
        }
        Ok(self)
    }
    /// Get node list from the node_pool_urls
    pub(crate) async fn with_node_pool_urls(mut self, node_pool_urls: &[String]) -> Result<Self> {
        for pool_url in node_pool_urls {
            let http_client = crate::node_manager::HttpClient::new();
            let nodes_details: Vec<NodeDetail> = http_client
                .get(
                    Node {
                        url: validate_url(Url::parse(pool_url)?)?,
                        jwt: None,
                    },
                    crate::builder::GET_API_TIMEOUT,
                )
                .await?
                .json()
                .await?;
            for node_detail in nodes_details {
                let url = validate_url(Url::parse(&node_detail.node)?)?;
                self.nodes.insert(Node { url, jwt: None });
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
    pub(crate) async fn add_default_nodes(mut self, network_info: &NetworkInfo) -> Result<Self> {
        // todo update with new node pool
        // let default_testnet_node_pools = vec!["https://giftiota.com/nodes.json".to_string()];
        let default_testnet_nodes = vec![
            "https://api.lb-0.h.chrysalis-devnet.iota.cafe/",
            "https://api.lb-1.h.chrysalis-devnet.iota.cafe/",
        ];
        if self.nodes.is_empty() && self.primary_node.is_none() {
            match network_info.network {
                Some(ref network) => match network.to_lowercase().as_str() {
                    "testnet" | "devnet" | "test" | "dev" => {
                        self = self.with_nodes(&default_testnet_nodes[..])?;
                        // self = self.with_node_pool_urls(&default_testnet_node_pools[..]).await?;
                    }
                    _ => return Err(Error::SyncedNodePoolEmpty),
                },
                _ => {
                    self = self.with_nodes(&default_testnet_nodes[..])?;
                    // self = self.with_node_pool_urls(&default_testnet_node_pools[..]).await?;
                }
            }
        }
        Ok(self)
    }
    pub(crate) fn build(self, synced_nodes: Arc<RwLock<HashSet<Node>>>) -> NodeManager {
        NodeManager {
            primary_node: self.primary_node,
            primary_pow_node: self.primary_pow_node,
            nodes: self.nodes,
            permanodes: self.permanodes,
            sync: self.sync,
            sync_interval: self.sync_interval,
            synced_nodes,
            quorum: self.quorum,
            quorum_size: self.quorum_size,
            quorum_threshold: self.quorum_threshold,
            http_client: HttpClient::new(),
        }
    }
}

impl Default for NodeManagerBuilder {
    fn default() -> Self {
        Self {
            primary_node: None,
            primary_pow_node: None,
            nodes: HashSet::new(),
            permanodes: None,
            sync: true,
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

    async fn parse_response(response: reqwest::Response) -> Result<Response> {
        let status = response.status();
        if status.is_success() {
            Ok(Response(response))
        } else {
            Err(Error::ResponseError(status.as_u16(), response.text().await?))
        }
    }

    pub(crate) async fn get(&self, node: Node, _timeout: Duration) -> Result<Response> {
        #[cfg(feature = "wasm")]
        let start_time = instant::Instant::now();
        #[cfg(not(feature = "wasm"))]
        let start_time = std::time::Instant::now();
        let mut request_builder = self.client.get(node.url.clone());
        if let Some(jwt) = node.jwt {
            request_builder = request_builder.bearer_auth(jwt);
        }
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(_timeout);
        }
        let resp = request_builder.send().await?;
        let response = Self::parse_response(resp).await;
        log::debug!(
            "GET request took {:?} ms for {}",
            start_time.elapsed().as_millis(),
            node.url
        );
        response
    }

    pub(crate) async fn post_bytes(&self, node: Node, _timeout: Duration, body: &[u8]) -> Result<Response> {
        let mut request_builder = self.client.post(node.url);
        if let Some(jwt) = node.jwt {
            request_builder = request_builder.bearer_auth(jwt);
        }
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(_timeout);
        }
        request_builder = request_builder.header("Content-Type", "application/octet-stream");
        Self::parse_response(request_builder.body(body.to_vec()).send().await?).await
    }

    pub(crate) async fn post_json(&self, node: Node, _timeout: Duration, json: Value) -> Result<Response> {
        let mut request_builder = self.client.post(node.url);
        if let Some(jwt) = node.jwt {
            request_builder = request_builder.bearer_auth(jwt);
        }
        #[cfg(not(feature = "wasm"))]
        {
            request_builder = request_builder.timeout(_timeout);
        }
        Self::parse_response(request_builder.json(&json).send().await?).await
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
        if let Some(jwt) = node.jwt {
            request_builder = request_builder.set("Authorization", &format!("Bearer {}", jwt));
        }
        Ok(request_builder.call()?.into())
    }

    pub(crate) async fn post_bytes(&self, node: Node, timeout: Duration, body: &[u8]) -> Result<Response> {
        let mut request_builder = Self::get_ureq_agent(timeout).post(node.url.as_str());
        if let Some(jwt) = node.jwt {
            request_builder = request_builder.set("Authorization", &format!("Bearer {}", jwt));
        }
        request_builder = request_builder.set("Content-Type", "application/octet-stream");
        Ok(request_builder.send_bytes(body)?.into())
    }

    pub(crate) async fn post_json(&self, node: Node, timeout: Duration, json: Value) -> Result<Response> {
        let mut request_builder = Self::get_ureq_agent(timeout).post(node.url.as_str());
        if let Some(jwt) = node.jwt {
            request_builder = request_builder.set("Authorization", &format!("Bearer {}", jwt));
        }
        Ok(request_builder.send_json(json)?.into())
    }

    fn get_ureq_agent(timeout: Duration) -> Agent {
        AgentBuilder::new().timeout_read(timeout).timeout_write(timeout).build()
    }
}

/// Validates if the url starts with http or https
pub fn validate_url(url: Url) -> Result<Url> {
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
