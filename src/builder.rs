// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Builder of the Client Instance
use crate::{
    client::*,
    constants::{
        DEFAULT_API_TIMEOUT, DEFAULT_BECH32_HRP, DEFAULT_MIN_POW, DEFAULT_REMOTE_POW_API_TIMEOUT, DEFAULT_TIPS_INTERVAL,
    },
    error::*,
    node_manager::{
        builder::validate_url,
        node::{Node, NodeAuth},
    },
};

use log::LevelFilter;

use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[cfg(feature = "mqtt")]
use crate::node_api::mqtt::{BrokerOptions, MqttEvent};

#[cfg(not(feature = "wasm"))]
use {
    std::collections::HashSet,
    tokio::{runtime::Runtime, sync::broadcast::channel},
};

/// Struct containing network and PoW related information
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NetworkInfo {
    /// Network
    pub network: Option<String>,
    /// Network ID
    #[serde(rename = "networkId")]
    pub network_id: Option<u64>,
    /// Protocol version
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Option<u8>,
    /// Bech32 HRP
    #[serde(rename = "bech32HRP", default = "default_bech32_hrp")]
    pub bech32_hrp: String,
    /// Mininum proof of work score
    #[serde(rename = "minPoWScore", default = "default_min_pow_score")]
    pub min_pow_score: f64,
    /// Local proof of work
    #[serde(rename = "localPow", default = "default_local_pow")]
    pub local_pow: bool,
    /// Fallback to local proof of work if the node doesn't support remote PoW
    #[serde(rename = "fallbackToLocalPow")]
    pub fallback_to_local_pow: bool,
    /// Tips request interval during PoW in seconds
    #[serde(rename = "tipsInterval", default = "default_tips_interval")]
    pub tips_interval: u64,
}

fn default_bech32_hrp() -> String {
    DEFAULT_BECH32_HRP.into()
}
fn default_min_pow_score() -> f64 {
    4000.0
}

fn default_local_pow() -> bool {
    #[cfg(not(feature = "wasm"))]
    {
        true
    }
    #[cfg(feature = "wasm")]
    {
        false
    }
}

fn default_tips_interval() -> u64 {
    DEFAULT_TIPS_INTERVAL
}

/// Builder to construct client instance with sensible default values
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientBuilder {
    #[serde(flatten, rename = "nodeManagerBuilder")]
    node_manager_builder: crate::node_manager::builder::NodeManagerBuilder,
    #[cfg(feature = "mqtt")]
    #[serde(flatten, rename = "brokerOptions")]
    broker_options: BrokerOptions,
    #[serde(flatten, rename = "networkInfo", default)]
    pub(crate) network_info: NetworkInfo,
    #[serde(rename = "apiTimeout", default = "default_api_timeout")]
    api_timeout: Duration,
    #[serde(rename = "remotePowTimeout", default = "default_remote_pow_timeout")]
    remote_pow_timeout: Duration,
    #[serde(default)]
    offline: bool,
    #[serde(rename = "powWorkerCount", default)]
    pow_worker_count: Option<usize>,
}

fn default_api_timeout() -> Duration {
    DEFAULT_API_TIMEOUT
}

fn default_remote_pow_timeout() -> Duration {
    DEFAULT_REMOTE_POW_API_TIMEOUT
}

impl Default for NetworkInfo {
    fn default() -> Self {
        Self {
            network: None,
            network_id: None,
            // todo default None and get from nodeinfo
            protocol_version: Some(2),
            min_pow_score: DEFAULT_MIN_POW,
            local_pow: default_local_pow(),
            fallback_to_local_pow: true,
            bech32_hrp: DEFAULT_BECH32_HRP.into(),
            tips_interval: DEFAULT_TIPS_INTERVAL,
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            node_manager_builder: crate::node_manager::NodeManager::builder(),
            #[cfg(feature = "mqtt")]
            broker_options: Default::default(),
            network_info: NetworkInfo::default(),
            api_timeout: DEFAULT_API_TIMEOUT,
            remote_pow_timeout: DEFAULT_REMOTE_POW_API_TIMEOUT,
            offline: false,
            pow_worker_count: None,
        }
    }
}

impl ClientBuilder {
    /// Creates an IOTA client builder.
    pub fn new() -> Self {
        Default::default()
    }

    #[allow(unused_assignments)]
    /// Set the fields from a client JSON config
    pub fn from_json(mut self, client_config: &str) -> Result<Self> {
        self = serde_json::from_str(client_config)?;
        // validate URLs
        if let Some(node_dto) = &self.node_manager_builder.primary_node {
            let node: Node = node_dto.into();
            validate_url(node.url)?;
        }
        if let Some(node_dto) = &self.node_manager_builder.primary_pow_node {
            let node: Node = node_dto.into();
            validate_url(node.url)?;
        }
        for node_dto in &self.node_manager_builder.nodes {
            let node: Node = node_dto.into();
            validate_url(node.url)?;
        }
        if let Some(permanodes) = &self.node_manager_builder.permanodes {
            for node_dto in permanodes {
                let node: Node = node_dto.into();
                validate_url(node.url)?;
            }
        }
        Ok(self)
    }

    /// Export the client builder as JSON string
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Adds an IOTA node by its URL.
    pub fn with_node(mut self, url: &str) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_node(url)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL to be used as primary node, with optional jwt and or basic authentication
    pub fn with_primary_node(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_primary_node(url, auth)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL to be used as primary PoW node (for remote PoW), with optional jwt and or basic
    /// authentication
    pub fn with_primary_pow_node(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_primary_pow_node(url, auth)?;
        Ok(self)
    }

    /// Adds a permanode by its URL, with optional jwt and or basic authentication
    pub fn with_permanode(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_permanode(url, auth)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL with optional jwt and or basic authentication
    pub fn with_node_auth(mut self, url: &str, auth: Option<NodeAuth>) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_node_auth(url, auth)?;
        Ok(self)
    }

    /// Adds a list of IOTA nodes by their URLs.
    pub fn with_nodes(mut self, urls: &[&str]) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_nodes(urls)?;
        Ok(self)
    }

    /// Set the node sync interval
    pub fn with_node_sync_interval(mut self, node_sync_interval: Duration) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_node_sync_interval(node_sync_interval);
        self
    }

    /// Disables the node syncing process.
    /// Every node will be considered healthy and ready to use.
    pub fn with_node_sync_disabled(mut self) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_node_sync_disabled();
        self
    }

    /// Allows creating the client without nodes for offline address generation or signing
    pub fn with_offline_mode(mut self) -> Self {
        self.offline = true;
        self
    }

    /// Get node list from the node_pool_urls
    pub async fn with_node_pool_urls(mut self, node_pool_urls: &[String]) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_node_pool_urls(node_pool_urls).await?;
        Ok(self)
    }

    /// Set if quroum should be used or not
    pub fn with_quorum(mut self, quorum: bool) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_quorum(quorum);
        self
    }

    /// Set amount of nodes which should be used for quorum
    pub fn with_min_quorum_size(mut self, min_quorum_size: usize) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_min_quorum_size(min_quorum_size);
        self
    }

    /// Set quorum_threshold
    pub fn with_quorum_threshold(mut self, threshold: usize) -> Self {
        let threshold = if threshold > 100 { 100 } else { threshold };
        self.node_manager_builder = self.node_manager_builder.with_quorum_threshold(threshold);
        self
    }

    /// Selects the type of network to get default nodes for it, only "testnet" is supported at the moment.
    /// Nodes that don't belong to this network are ignored. The &str must match a part or all of the networkId returned
    /// in the nodeinfo from a node. For example, if the networkId is `"private-tangle"`, `"tangle"` can be used.
    /// Default nodes are only used when no other nodes are provided.
    pub fn with_network(mut self, network: &str) -> Self {
        self.network_info.network.replace(network.into());
        self
    }

    /// Sets the MQTT broker options.
    #[cfg(feature = "mqtt")]
    pub fn with_mqtt_broker_options(mut self, options: BrokerOptions) -> Self {
        self.broker_options = options;
        self
    }

    /// Sets whether the PoW should be done locally or remotely.
    pub fn with_local_pow(mut self, local: bool) -> Self {
        self.network_info.local_pow = local;
        self
    }

    /// Sets the amount of workers that should be used for PoW, default is num_cpus::get().
    pub fn with_pow_worker_count(mut self, worker_count: usize) -> Self {
        self.pow_worker_count.replace(worker_count);
        self
    }

    /// Sets whether the PoW should be done locally in case a node doesn't support remote PoW.
    pub fn with_fallback_to_local_pow(mut self, fallback_to_local_pow: bool) -> Self {
        self.network_info.fallback_to_local_pow = fallback_to_local_pow;
        self
    }

    /// Sets after how many seconds new tips will be requested during PoW
    pub fn with_tips_interval(mut self, tips_interval: u64) -> Self {
        self.network_info.tips_interval = tips_interval;
        self
    }

    /// Sets the default request timeout.
    pub fn with_api_timeout(mut self, timeout: Duration) -> Self {
        self.api_timeout = timeout;
        self
    }

    /// Sets the request timeout for API usage.
    pub fn with_remote_pow_timeout(mut self, timeout: Duration) -> Self {
        self.remote_pow_timeout = timeout;
        self
    }

    /// Enables the default logger which writes debug logs to "iota.rs.log"
    pub fn with_default_logger(self) -> Result<Self> {
        crate::init_logger("iota.rs.log", LevelFilter::Debug)?;
        Ok(self)
    }

    /// Write logs to a file
    pub fn with_logger(self, filename: &str, level: LevelFilter) -> Result<Self> {
        crate::init_logger(filename, level)?;
        Ok(self)
    }

    /// Build the Client instance.
    pub async fn finish(mut self) -> Result<Client> {
        // Add default nodes
        if !self.offline {
            self.node_manager_builder = self.node_manager_builder.add_default_nodes(&self.network_info).await?;
            // Return error if we don't have a node
            if self.node_manager_builder.nodes.is_empty() && self.node_manager_builder.primary_node.is_none() {
                return Err(Error::MissingParameter("Node"));
            }
        }
        let network_info = Arc::new(RwLock::new(self.network_info));
        let nodes = self
            .node_manager_builder
            .nodes
            .iter()
            .map(|node| node.clone().into())
            .collect();
        #[cfg(feature = "wasm")]
        let (sync, network_info) = (Arc::new(RwLock::new(nodes)), network_info);
        #[cfg(not(feature = "wasm"))]
        let (runtime, sync, sync_kill_sender, network_info) = if self.node_manager_builder.node_sync_enabled {
            let sync = Arc::new(RwLock::new(HashSet::new()));
            let sync_ = sync.clone();
            let network_info_ = network_info.clone();
            let (sync_kill_sender, sync_kill_receiver) = channel(1);
            let runtime = std::thread::spawn(move || {
                let runtime = Runtime::new().expect("Failed to create Tokio runtime");
                runtime.block_on(Client::sync_nodes(&sync_, &nodes, &network_info_));
                Client::start_sync_process(
                    &runtime,
                    sync_,
                    nodes,
                    self.node_manager_builder.node_sync_interval,
                    network_info_,
                    sync_kill_receiver,
                );
                runtime
            })
            .join()
            .expect("failed to init node syncing process");
            (Some(Arc::new(runtime)), sync, Some(sync_kill_sender), network_info)
        } else {
            (None, Arc::new(RwLock::new(nodes)), None, network_info)
        };

        #[cfg(feature = "mqtt")]
        let (mqtt_event_tx, mqtt_event_rx) = tokio::sync::watch::channel(MqttEvent::Connected);
        let client = Client {
            node_manager: self.node_manager_builder.build(sync),
            #[cfg(not(feature = "wasm"))]
            runtime,
            #[cfg(not(feature = "wasm"))]
            sync_kill_sender: sync_kill_sender.map(Arc::new),
            #[cfg(feature = "mqtt")]
            mqtt_client: None,
            #[cfg(feature = "mqtt")]
            mqtt_topic_handlers: Default::default(),
            #[cfg(feature = "mqtt")]
            broker_options: self.broker_options,
            #[cfg(feature = "mqtt")]
            mqtt_event_channel: (Arc::new(mqtt_event_tx), mqtt_event_rx),
            network_info,
            api_timeout: self.api_timeout,
            remote_pow_timeout: self.remote_pow_timeout,
            pow_worker_count: self.pow_worker_count,
        };
        Ok(client)
    }
}
