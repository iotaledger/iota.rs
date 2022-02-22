// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests with synced nodes and quorum if enabled

use crate::{
    error::{Error, Result},
    node_manager::builder::NodeManagerBuilder,
};

use bee_rest_api::types::responses::InfoResponse;

use log::warn;
use regex::Regex;
use serde_json::Value;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Duration,
};

pub mod builder;
pub(crate) mod http_client;
use http_client::HttpClient;
pub(crate) mod node;
use node::Node;

#[cfg(all(feature = "sync", not(feature = "async")))]
use ureq::{Agent, AgentBuilder};

// Nodemanger, takes care of selecting node(s) for requests until a result is returned or if quorum
// is enabled it will send the requests for some endpoints to multiple nodes and compares the results
#[derive(Clone)]
pub(crate) struct NodeManager {
    pub(crate) primary_node: Option<Node>,
    primary_pow_node: Option<Node>,
    pub(crate) nodes: HashSet<Node>,
    permanodes: Option<HashSet<Node>>,
    pub(crate) node_sync_enabled: bool,
    node_sync_interval: Duration,
    pub(crate) synced_nodes: Arc<RwLock<HashSet<Node>>>,
    quorum: bool,
    min_quorum_size: usize,
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
        d.field("node_sync_enabled", &self.node_sync_enabled);
        d.field("node_sync_interval", &self.node_sync_interval);
        d.field("synced_nodes", &self.synced_nodes);
        d.field("quorum", &self.quorum);
        d.field("min_quorum_size", &self.min_quorum_size);
        d.field("quorum_threshold", &self.quorum_threshold).finish()
    }
}

impl NodeManager {
    pub(crate) fn builder() -> NodeManagerBuilder {
        NodeManagerBuilder::new()
    }
    pub(crate) async fn get_nodes(&self, path: &str, query: Option<&str>, local_pow: bool) -> Result<Vec<Node>> {
        let mut nodes_with_modified_url = Vec::new();

        // Endpoints for which only permanodes will be used if provided
        let permanode_regexes = lazy_static!(
            [
              Regex::new(r"messages/([A-Fa-f0-9]{64})").expect("regex failed"),
              Regex::new(r"messages/([A-Fa-f0-9]{64})/metadata").expect("regex failed"),
              Regex::new(r"messages/([A-Fa-f0-9]{64})/children").expect("regex failed"),
              Regex::new(r"outputs/([A-Fa-f0-9]{64})(\d{4})").expect("regex failed"),
              // bech32 address
              Regex::new("addresses/(iota|atoi|iot|toi)1[A-Za-z0-9]").expect("regex failed"),
              Regex::new("addresses/(iota|atoi|iot|toi)1[A-Za-z0-9]+/outputs").expect("regex failed"),
              // ED25519 address hex
              Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})").expect("regex failed"),
              Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})/outputs").expect("regex failed"),
              Regex::new(r"transactions/([A-Fa-f0-9]{64})/included-message").expect("regex failed"),
              Regex::new(r"milestones/[0-9]").expect("regex failed"),
            ].to_vec() => Vec<Regex>
        );
        if permanode_regexes.iter().any(|re| re.is_match(path)) || (path == "api/v2/messages" && query.is_some()) {
            if let Some(permanodes) = self.permanodes.clone() {
                // remove api/v2/ since permanodes can have custom keyspaces
                // https://editor.swagger.io/?url=https://raw.githubusercontent.com/iotaledger/chronicle.rs/main/docs/api.yaml
                let path = &path["api/v2/".len()..];
                for mut permanode in permanodes {
                    permanode.url.set_path(&format!("{}{}", permanode.url.path(), path));
                    permanode.url.set_query(query);
                    nodes_with_modified_url.push(permanode);
                }
            }
        }

        if local_pow {
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
        let nodes = if self.node_sync_enabled {
            #[cfg(not(feature = "wasm"))]
            {
                self.synced_nodes.read().map_err(|_| crate::Error::PoisonError)?.clone()
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
        // remove disabled nodes
        nodes_with_modified_url.retain(|n| !n.disabled);
        if nodes_with_modified_url.is_empty() {
            return Err(crate::Error::SyncedNodePoolEmpty);
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
        // Get node urls and set path
        let nodes = self.get_nodes(path, query, false).await?;
        if self.quorum && quorum_regexes.iter().any(|re| re.is_match(path)) && nodes.len() < self.min_quorum_size {
            return Err(Error::QuorumPoolSizeError(nodes.len(), self.min_quorum_size));
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
                    if index < self.min_quorum_size {
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
                                    if path == "api/v2/info" {
                                        if let Ok(nodeinfo) = serde_json::from_str::<InfoResponse>(&res_text) {
                                            let wrapper = crate::client::NodeInfoWrapper {
                                                nodeinfo,
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
                                            || result_counter >= self.min_quorum_size
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
            || res.1 as f64 >= self.min_quorum_size as f64 * (self.quorum_threshold as f64 / 100.0)
            || !quorum_regexes.iter().any(|re| re.is_match(path))
            // with query we ignore quorum because the nodes can store a different amount of history
            || query.is_some()
        {
            Ok(serde_json::from_str(&res.0)?)
        } else {
            Err(Error::QuorumThresholdError(res.1, self.min_quorum_size))
        }
    }
    // Only used for api/v2/messages/{messageID}/raw, that's why we don't need the quorum stuff
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
        let nodes = self.get_nodes(path, None, local_pow).await?;
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
        let nodes = self.get_nodes(path, None, local_pow).await?;
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
