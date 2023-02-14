// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests with healthy nodes and quorum if enabled

pub mod builder;
pub(crate) mod http_client;
/// Structs for nodes
pub mod node;
pub(crate) mod syncing;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Duration,
};

use iota_types::api::core::response::InfoResponse;
use serde_json::Value;

use self::{http_client::HttpClient, node::Node};
use crate::{
    error::{Error, Result},
    node_manager::builder::NodeManagerBuilder,
};

// The node manager takes care of selecting node(s) for requests until a result is returned or if quorum is enabled it
// will send the requests for some endpoints to multiple nodes and compares the results.
#[derive(Clone)]
pub(crate) struct NodeManager {
    pub(crate) primary_node: Option<Node>,
    primary_pow_node: Option<Node>,
    pub(crate) nodes: HashSet<Node>,
    permanodes: Option<HashSet<Node>>,
    pub(crate) ignore_node_health: bool,
    node_sync_interval: Duration,
    pub(crate) healthy_nodes: Arc<RwLock<HashMap<Node, InfoResponse>>>,
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
        d.field("ignore_node_health", &self.ignore_node_health);
        d.field("node_sync_interval", &self.node_sync_interval);
        d.field("healthy_nodes", &self.healthy_nodes);
        d.field("quorum", &self.quorum);
        d.field("min_quorum_size", &self.min_quorum_size);
        d.field("quorum_threshold", &self.quorum_threshold).finish()
    }
}

impl NodeManager {
    pub(crate) fn builder() -> NodeManagerBuilder {
        NodeManagerBuilder::new()
    }

    fn get_nodes(
        &self,
        path: &str,
        query: Option<&str>,
        use_pow_nodes: bool,
        prefer_permanode: bool,
    ) -> Result<Vec<Node>> {
        let mut nodes_with_modified_url: Vec<Node> = Vec::new();

        if prefer_permanode || (path == "api/core/v2/blocks" && query.is_some()) {
            if let Some(permanodes) = self.permanodes.clone() {
                for permanode in permanodes {
                    if !nodes_with_modified_url.iter().any(|n| n.url == permanode.url) {
                        nodes_with_modified_url.push(permanode);
                    }
                }
            }
        }

        if use_pow_nodes {
            if let Some(pow_node) = self.primary_pow_node.clone() {
                if !nodes_with_modified_url.iter().any(|n| n.url == pow_node.url) {
                    nodes_with_modified_url.push(pow_node);
                }
            }
        }

        if let Some(primary_node) = self.primary_node.clone() {
            if !nodes_with_modified_url.iter().any(|n| n.url == primary_node.url) {
                nodes_with_modified_url.push(primary_node);
            }
        }

        // Add other nodes in random order, so they are not always used in the same order
        let nodes_random_order = if !self.ignore_node_health {
            #[cfg(not(target_family = "wasm"))]
            {
                self.healthy_nodes
                    .read()
                    .map_err(|_| crate::Error::PoisonError)?
                    .iter()
                    .filter_map(|(n, info)| {
                        // Only add nodes with pow feature enabled, when remote PoW is used
                        if use_pow_nodes {
                            let pow_feature = String::from("pow");

                            if info.features.contains(&pow_feature) {
                                Some(n.clone())
                            } else {
                                None
                            }
                        } else {
                            Some(n.clone())
                        }
                    })
                    .collect()
            }
            #[cfg(target_family = "wasm")]
            {
                self.nodes.clone()
            }
        } else {
            self.nodes.clone()
        };

        // Add remaining nodes in random order
        for node in nodes_random_order {
            if !nodes_with_modified_url.iter().any(|n| n.url == node.url) {
                nodes_with_modified_url.push(node);
            }
        }

        // remove disabled nodes
        nodes_with_modified_url.retain(|n| !n.disabled);

        if nodes_with_modified_url.is_empty() {
            return Err(crate::Error::HealthyNodePoolEmpty);
        }

        // Set path and query parameters
        for node in &mut nodes_with_modified_url {
            node.url.set_path(path);
            node.url.set_query(query);
            if let Some(auth) = &node.auth {
                if let Some((name, password)) = &auth.basic_auth_name_pwd {
                    node.url
                        .set_username(name)
                        .map_err(|_| crate::Error::UrlAuth("username"))?;
                    node.url
                        .set_password(Some(password))
                        .map_err(|_| crate::Error::UrlAuth("password"))?;
                }
            }
        }

        Ok(nodes_with_modified_url)
    }

    pub(crate) async fn get_request<T: serde::de::DeserializeOwned + std::fmt::Debug + serde::Serialize>(
        &self,
        path: &str,
        query: Option<&str>,
        timeout: Duration,
        need_quorum: bool,
        prefer_permanode: bool,
    ) -> Result<T> {
        let mut result: HashMap<String, usize> = HashMap::new();
        // primary_pow_node should only be used for post request with remote PoW
        // Get node urls and set path
        let nodes = self.get_nodes(path, query, false, prefer_permanode)?;
        if self.quorum && need_quorum && nodes.len() < self.min_quorum_size {
            return Err(Error::QuorumPoolSizeError {
                available_nodes: nodes.len(),
                minimum_threshold: self.min_quorum_size,
            });
        }

        // Track amount of results for quorum
        let mut result_counter = 0;
        let mut error = None;
        // Send requests parallel for quorum
        #[cfg(target_family = "wasm")]
        let wasm = true;
        #[cfg(not(target_family = "wasm"))]
        let wasm = false;
        if !wasm && self.quorum && need_quorum && query.is_none() {
            #[cfg(not(target_family = "wasm"))]
            {
                let mut tasks = Vec::new();
                for (index, node) in nodes.into_iter().enumerate() {
                    if index < self.min_quorum_size {
                        let client_ = self.http_client.clone();
                        tasks.push(async move { tokio::spawn(async move { client_.get(node, timeout).await }).await });
                    }
                }
                for res in futures::future::try_join_all(tasks).await? {
                    match res {
                        Ok(res) => (res.into_text().await).map_or_else(
                            |_| {
                                log::warn!("couldn't convert node response to text");
                            },
                            |res_text| {
                                let counters = result.entry(res_text).or_insert(0);
                                *counters += 1;
                                result_counter += 1;
                            },
                        ),
                        Err(Error::ResponseError { code: 404, url, .. }) => {
                            error.replace(crate::Error::NotFound(url));
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
                        match res.status() {
                            200 => {
                                // Handle node_info extra because we also want to return the url
                                if path == "api/core/v2/info" {
                                    let node_info: InfoResponse = res.into_json().await?;
                                    let wrapper = crate::node_api::core::routes::NodeInfoWrapper {
                                        node_info,
                                        url: format!("{}://{}", node.url.scheme(), node.url.host_str().unwrap_or("")),
                                    };
                                    let serde_res = serde_json::to_string(&wrapper)?;
                                    return Ok(serde_json::from_str(&serde_res)?);
                                }

                                match res.into_json::<T>().await {
                                    Ok(result_data) => {
                                        let counters = result.entry(serde_json::to_string(&result_data)?).or_insert(0);
                                        *counters += 1;
                                        result_counter += 1;
                                        // Without quorum it's enough if we got one response
                                        if !self.quorum
                                            || result_counter >= self.min_quorum_size
                                            || !need_quorum
                                            // with query we ignore quorum because the nodes can store a different amount of history
                                            || query.is_some()
                                        {
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        error.replace(e);
                                    }
                                }
                            }

                            _ => {
                                error.replace(crate::Error::Node(
                                    res.into_text()
                                        .await
                                        .unwrap_or_else(|_| "couldn't convert node response into text".to_string()),
                                ));
                            }
                        }
                    }
                    Err(Error::ResponseError { code: 404, url, .. }) => {
                        error.replace(crate::Error::NotFound(url));
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
            .ok_or_else(|| error.unwrap_or_else(|| Error::Node("couldn't get a result from any node".into())))?;

        // Return if quorum is false or check if quorum was reached
        if !self.quorum
            || res.1 as f64 >= self.min_quorum_size as f64 * (self.quorum_threshold as f64 / 100.0)
            || !need_quorum
            // with query we ignore quorum because the nodes can store a different amount of history
            || query.is_some()
        {
            Ok(serde_json::from_str(&res.0)?)
        } else {
            Err(Error::QuorumThresholdError {
                quorum_size: res.1,
                minimum_threshold: self.min_quorum_size,
            })
        }
    }

    // Only used for api/core/v2/blocks/{blockID}, that's why we don't need the quorum stuff
    pub(crate) async fn get_request_bytes(
        &self,
        path: &str,
        query: Option<&str>,
        timeout: Duration,
    ) -> Result<Vec<u8>> {
        // primary_pow_node should only be used for post request with remote Pow
        // Get node urls and set path
        let nodes = self.get_nodes(path, query, false, false)?;
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.get_bytes(node, timeout).await {
                Ok(res) => {
                    let status = res.status();
                    if let Ok(res_text) = res.into_bytes().await {
                        // Without quorum it's enough if we got one response
                        match status {
                            200 => return Ok(res_text),
                            _ => error.replace(crate::Error::Node(
                                String::from_utf8(res_text)
                                    .map_err(|_| Error::Node("non UTF8 node response".into()))?,
                            )),
                        };
                    }
                }
                Err(Error::ResponseError { code: 404, url, .. }) => {
                    error.replace(crate::Error::NotFound(url));
                }
                Err(err) => {
                    error.replace(err);
                }
            }
        }
        Err(error.unwrap_or_else(|| Error::Node("couldn't get a result from any node".into())))
    }

    pub(crate) async fn post_request_bytes<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        timeout: Duration,
        body: &[u8],
        local_pow: bool,
    ) -> Result<T> {
        // primary_pow_node should only be used for post request with remote PoW
        let nodes = self.get_nodes(path, None, !local_pow, false)?;
        if nodes.is_empty() {
            return Err(Error::Node("no available nodes with remote Pow".into()));
        }
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.post_bytes(node, timeout, body).await {
                Ok(res) => {
                    match res.status() {
                        200 | 201 => match res.into_json::<T>().await {
                            Ok(res) => return Ok(res),
                            Err(e) => error.replace(e),
                        },
                        _ => error.replace(crate::Error::Node(
                            res.into_text()
                                .await
                                .unwrap_or_else(|_| "couldn't convert node response into text".to_string()),
                        )),
                    };
                }
                Err(e) => {
                    error.replace(crate::Error::Node(e.to_string()));
                }
            }
        }
        Err(error.unwrap_or_else(|| Error::Node("couldn't get a result from any node".into())))
    }

    pub(crate) async fn post_request_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        timeout: Duration,
        json: Value,
        local_pow: bool,
    ) -> Result<T> {
        // primary_pow_node should only be used for post request with remote PoW
        let nodes = self.get_nodes(path, None, !local_pow, false)?;
        if nodes.is_empty() {
            return Err(Error::Node("no available nodes with remote Pow".into()));
        }
        let mut error = None;
        // Send requests
        for node in nodes {
            match self.http_client.post_json(node, timeout, json.clone()).await {
                Ok(res) => {
                    match res.status() {
                        200 | 201 => match res.into_json::<T>().await {
                            Ok(res) => return Ok(res),
                            Err(e) => error.replace(e),
                        },
                        _ => error.replace(crate::Error::Node(
                            res.into_text()
                                .await
                                .unwrap_or_else(|_| "couldn't convert node response into text".to_string()),
                        )),
                    };
                }
                Err(e) => {
                    error.replace(crate::Error::Node(e.to_string()));
                }
            }
        }
        Err(error.unwrap_or_else(|| Error::Node("couldn't get a result from any node".into())))
    }
}
