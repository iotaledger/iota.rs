// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The node manager that takes care of sending requests with synced nodes and quorum if enabled

use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    builder::NetworkInfo,
    constants::{DEFAULT_API_TIMEOUT, DEFAULT_MIN_QUORUM_SIZE, DEFAULT_QUORUM_THRESHOLD, NODE_SYNC_INTERVAL},
    error::{Error, Result},
    node_manager::{
        http_client::HttpClient,
        node::{Node, NodeAuth, NodeDetail, NodeDto},
        NodeManager,
    },
};

/// Node manager builder
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NodeManagerBuilder {
    /// Node which will be tried first for all requests
    #[serde(rename = "primaryNode")]
    pub primary_node: Option<NodeDto>,
    /// Node which will be tried first when using remote PoW, even before the primary_node
    #[serde(rename = "primaryPoWNode")]
    pub primary_pow_node: Option<NodeDto>,
    /// Nodes
    #[serde(default)]
    pub nodes: HashSet<NodeDto>,
    /// Permanodes
    pub permanodes: Option<HashSet<NodeDto>>,
    /// If node syncing is enabled
    #[serde(rename = "nodeSyncEnabled", default = "default_node_sync_enabled")]
    pub node_sync_enabled: bool,
    /// Interval in which nodes will be checked for their sync status and the [NetworkInfo] gets updated
    #[serde(rename = "nodeSyncInterval", default = "default_node_sync_interval")]
    pub node_sync_interval: Duration,
    /// If node quorum is enabled. Will compare the responses from multiple nodes and only returns the response if
    /// `quorum_threshold`% of the nodes return the same one
    #[serde(default)]
    pub quorum: bool,
    /// Minimum amount of nodes required for request when quorum is enabled
    #[serde(rename = "minQuorumSize", default = "default_min_quorum_size")]
    pub min_quorum_size: usize,
    /// % of nodes that have to return the same response so it gets accepted
    #[serde(rename = "quorumThreshold", default = "default_quorum_threshold")]
    pub quorum_threshold: usize,
}

fn default_node_sync_enabled() -> bool {
    true
}

fn default_node_sync_interval() -> Duration {
    NODE_SYNC_INTERVAL
}

fn default_min_quorum_size() -> usize {
    DEFAULT_MIN_QUORUM_SIZE
}

fn default_quorum_threshold() -> usize {
    DEFAULT_QUORUM_THRESHOLD
}

impl NodeManagerBuilder {
    pub(crate) fn new() -> Self {
        Default::default()
    }
    pub(crate) fn with_node(mut self, url: &str) -> Result<Self> {
        let url = validate_url(Url::parse(url)?)?;
        self.nodes.insert(NodeDto::Node(Node {
            url,
            auth: None,
            disabled: false,
        }));
        Ok(self)
    }
    pub(crate) fn with_primary_node(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name)
                    .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
                url.set_password(Some(password))
                    .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
            }
        }
        self.primary_node.replace(NodeDto::Node(Node {
            url,
            auth,
            disabled: false,
        }));
        Ok(self)
    }
    pub(crate) fn with_primary_pow_node(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name)
                    .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
                url.set_password(Some(password))
                    .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
            }
        }
        self.primary_pow_node.replace(NodeDto::Node(Node {
            url,
            auth,
            disabled: false,
        }));
        Ok(self)
    }
    pub(crate) fn with_permanode(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name)
                    .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
                url.set_password(Some(password))
                    .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
            }
        }
        match self.permanodes {
            Some(ref mut permanodes) => {
                permanodes.insert(NodeDto::Node(Node {
                    url,
                    auth,
                    disabled: false,
                }));
            }
            None => {
                let mut permanodes = HashSet::new();
                permanodes.insert(NodeDto::Node(Node {
                    url,
                    auth,
                    disabled: false,
                }));
                self.permanodes.replace(permanodes);
            }
        }
        Ok(self)
    }
    pub(crate) fn with_node_sync_disabled(mut self) -> Self {
        self.node_sync_enabled = false;
        self
    }
    pub(crate) fn with_node_auth(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        let mut url = validate_url(Url::parse(url)?)?;
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name)
                    .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
                url.set_password(Some(password))
                    .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
            }
        }
        self.nodes.insert(NodeDto::Node(Node {
            url,
            auth,
            disabled: false,
        }));
        Ok(self)
    }
    pub(crate) fn with_nodes(mut self, urls: &[&str]) -> Result<Self> {
        for url in urls {
            let url = validate_url(Url::parse(url)?)?;
            self.nodes.insert(NodeDto::Node(Node {
                url,
                auth: None,
                disabled: false,
            }));
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
                        auth: None,
                        disabled: false,
                    },
                    DEFAULT_API_TIMEOUT,
                )
                .await?
                .json()
                .await?;
            for node_detail in nodes_details {
                let url = validate_url(Url::parse(&node_detail.node)?)?;
                self.nodes.insert(NodeDto::Node(Node {
                    url,
                    auth: None,
                    disabled: false,
                }));
            }
        }
        Ok(self)
    }
    pub(crate) fn with_node_sync_interval(mut self, node_sync_interval: Duration) -> Self {
        self.node_sync_interval = node_sync_interval;
        self
    }
    pub(crate) fn with_quorum(mut self, quorum: bool) -> Self {
        self.quorum = quorum;
        self
    }
    pub(crate) fn with_min_quorum_size(mut self, min_quorum_size: usize) -> Self {
        self.min_quorum_size = min_quorum_size;
        self
    }
    pub(crate) fn with_quorum_threshold(mut self, threshold: usize) -> Self {
        self.quorum_threshold = threshold;
        self
    }
    pub(crate) async fn add_default_nodes(mut self, network_info: &NetworkInfo) -> Result<Self> {
        // todo update with new node pool
        // let default_testnet_node_pools = vec!["https://giftiota.com/nodes.json".to_string()];
        let default_testnet_nodes = vec![];
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
            primary_node: self.primary_node.map(|node| node.into()),
            primary_pow_node: self.primary_pow_node.map(|node| node.into()),
            nodes: self.nodes.into_iter().map(|node| node.into()).collect(),
            permanodes: self
                .permanodes
                .map(|nodes| nodes.into_iter().map(|node| node.into()).collect()),
            node_sync_enabled: self.node_sync_enabled,
            node_sync_interval: self.node_sync_interval,
            synced_nodes,
            quorum: self.quorum,
            min_quorum_size: self.min_quorum_size,
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
            node_sync_enabled: true,
            node_sync_interval: NODE_SYNC_INTERVAL,
            quorum: false,
            min_quorum_size: DEFAULT_MIN_QUORUM_SIZE,
            quorum_threshold: DEFAULT_QUORUM_THRESHOLD,
        }
    }
}

/// Validates if the url starts with http or https
pub fn validate_url(url: Url) -> Result<Url> {
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(Error::UrlValidationError(format!("Invalid scheme: {}", url.scheme())));
    }
    Ok(url)
}
