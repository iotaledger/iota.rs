// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Builder of the Client Instance
use crate::{client::*, error::*};

#[cfg(not(feature = "wasm"))]
use tokio::{
    runtime::Runtime,
    sync::{broadcast::channel, RwLock},
};

#[cfg(not(feature = "wasm"))]
use std::collections::HashSet;
#[cfg(feature = "wasm")]
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc, time::Duration};

const DEFAULT_REMOTE_POW_TIMEOUT: Duration = Duration::from_secs(50);
pub(crate) const GET_API_TIMEOUT: Duration = Duration::from_secs(10);
#[cfg(not(feature = "wasm"))]
const NODE_SYNC_INTERVAL: Duration = Duration::from_secs(60);
// Interval in seconds when new tips will be requested during PoW
const TIPS_INTERVAL: u64 = 15;
const DEFAULT_MIN_POW: f64 = 4000f64;
const DEFAULT_BECH32_HRP: &str = "iota";

/// Struct containing network and PoW related information
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NetworkInfo {
    /// Network
    pub network: Option<String>,
    /// Network ID
    #[serde(rename = "networkId")]
    pub network_id: Option<u64>,
    /// Bech32 HRP
    #[serde(rename = "bech32HRP")]
    pub bech32_hrp: String,
    /// Mininum proof of work score
    #[serde(rename = "minPoWScore")]
    pub min_pow_score: f64,
    /// Local proof of work
    #[serde(rename = "localPow")]
    pub local_pow: bool,
    /// Tips request interval during PoW in seconds
    #[serde(rename = "tipsInterval")]
    pub tips_interval: u64,
}

/// Builder to construct client instance with sensible default values
pub struct ClientBuilder {
    node_manager_builder: crate::node_manager::NodeManagerBuilder,
    #[cfg(not(feature = "wasm"))]
    node_sync_interval: Duration,
    #[cfg(not(feature = "wasm"))]
    node_sync_enabled: bool,
    #[cfg(feature = "mqtt")]
    broker_options: BrokerOptions,
    pub(crate) network_info: NetworkInfo,
    request_timeout: Duration,
    api_timeout: HashMap<Api, Duration>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            node_manager_builder: crate::node_manager::NodeManager::builder(),
            #[cfg(not(feature = "wasm"))]
            node_sync_interval: NODE_SYNC_INTERVAL,
            #[cfg(not(feature = "wasm"))]
            node_sync_enabled: true,
            #[cfg(feature = "mqtt")]
            broker_options: Default::default(),
            network_info: NetworkInfo {
                network: None,
                network_id: None,
                min_pow_score: DEFAULT_MIN_POW,
                #[cfg(not(feature = "wasm"))]
                local_pow: true,
                #[cfg(feature = "wasm")]
                local_pow: false,
                bech32_hrp: DEFAULT_BECH32_HRP.into(),
                tips_interval: TIPS_INTERVAL,
            },
            request_timeout: DEFAULT_REMOTE_POW_TIMEOUT,
            api_timeout: Default::default(),
        }
    }
}

impl ClientBuilder {
    /// Creates an IOTA client builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds an IOTA node by its URL.
    pub fn with_node(mut self, url: &str) -> Result<Self> {
        self.node_manager_builder = self.node_manager_builder.with_node(url)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL to be used as primary node, with optional jwt and or basic authentication
    pub fn with_primary_node(
        mut self,
        url: &str,
        jwt: Option<String>,
        basic_auth_name_pwd: Option<(&str, &str)>,
    ) -> Result<Self> {
        self.node_manager_builder = self
            .node_manager_builder
            .with_primary_node(url, jwt, basic_auth_name_pwd)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL to be used as primary PoW node (for remote PoW), with optional jwt and or basic
    /// authentication
    pub fn with_primary_pow_node(
        mut self,
        url: &str,
        jwt: Option<String>,
        basic_auth_name_pwd: Option<(&str, &str)>,
    ) -> Result<Self> {
        self.node_manager_builder = self
            .node_manager_builder
            .with_primary_pow_node(url, jwt, basic_auth_name_pwd)?;
        Ok(self)
    }

    /// Adds an IOTA node by its URL with optional jwt and or basic authentication
    pub fn with_node_auth(
        mut self,
        url: &str,
        jwt: Option<String>,
        basic_auth_name_pwd: Option<(&str, &str)>,
    ) -> Result<Self> {
        self.node_manager_builder = self
            .node_manager_builder
            .with_node_auth(url, jwt, basic_auth_name_pwd)?;
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
    pub fn with_quorum_size(mut self, quorum_size: usize) -> Self {
        self.node_manager_builder = self.node_manager_builder.with_quorum_size(quorum_size);
        self
    }

    /// Set quorum_threshold
    pub fn with_quorum_threshold(mut self, threshold: usize) -> Self {
        let threshold = if threshold > 100 { 100 } else { threshold };
        self.node_manager_builder = self.node_manager_builder.with_quorum_threshold(threshold);
        self
    }

    /// Selects the type of network to get default nodes for it, only "testnet" is supported at the moment.
    /// Nodes that don't belong to this network are ignored. Default nodes are only used when no other nodes are
    /// provided.
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

    /// Sets after how many seconds new tips will be requested during PoW
    pub fn with_tips_interval(mut self, tips_interval: u64) -> Self {
        self.network_info.tips_interval = tips_interval;
        self
    }

    /// Sets the default request timeout.
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Sets the request timeout for a specific API usage.
    pub fn with_api_timeout(mut self, api: Api, timeout: Duration) -> Self {
        self.api_timeout.insert(api, timeout);
        self
    }

    /// Build the Client instance.
    pub async fn finish(mut self) -> Result<Client> {
        let network_info = Arc::new(RwLock::new(self.network_info));
        let nodes = self.node_manager_builder.nodes.clone();
        #[cfg(not(feature = "wasm"))]
        let node_sync_interval = self.node_sync_interval;

        #[cfg(feature = "wasm")]
        let (sync, network_info) = (Arc::new(std::sync::RwLock::new(nodes.clone())), network_info);
        #[cfg(not(feature = "wasm"))]
        let (runtime, sync, sync_kill_sender, network_info) = if self.node_sync_enabled {
            let sync = Arc::new(RwLock::new(HashSet::new()));
            let sync_ = sync.clone();
            let network_info_ = network_info.clone();
            let (sync_kill_sender, sync_kill_receiver) = channel(1);
            let nodes = nodes.clone();
            let runtime = std::thread::spawn(move || {
                let runtime = Runtime::new().expect("Failed to create Tokio runtime");
                runtime.block_on(Client::sync_nodes(&sync_, &nodes, &network_info_));
                Client::start_sync_process(
                    &runtime,
                    sync_,
                    nodes,
                    node_sync_interval,
                    network_info_,
                    sync_kill_receiver,
                );
                runtime
            })
            .join()
            .expect("failed to init node syncing process");
            (Some(runtime), sync, Some(sync_kill_sender), network_info)
        } else {
            (None, Arc::new(RwLock::new(nodes.clone())), None, network_info)
        };

        let mut api_timeout = HashMap::new();
        api_timeout.insert(
            Api::GetInfo,
            self.api_timeout.remove(&Api::GetInfo).unwrap_or(GET_API_TIMEOUT),
        );
        api_timeout.insert(
            Api::GetPeers,
            self.api_timeout.remove(&Api::GetPeers).unwrap_or(GET_API_TIMEOUT),
        );
        api_timeout.insert(
            Api::GetHealth,
            self.api_timeout.remove(&Api::GetHealth).unwrap_or(GET_API_TIMEOUT),
        );
        api_timeout.insert(
            Api::GetMilestone,
            self.api_timeout.remove(&Api::GetMilestone).unwrap_or(GET_API_TIMEOUT),
        );
        api_timeout.insert(
            Api::GetBalance,
            self.api_timeout.remove(&Api::GetBalance).unwrap_or(GET_API_TIMEOUT),
        );
        api_timeout.insert(
            Api::GetMessage,
            self.api_timeout.remove(&Api::GetMessage).unwrap_or(GET_API_TIMEOUT),
        );
        api_timeout.insert(
            Api::GetTips,
            self.api_timeout.remove(&Api::GetTips).unwrap_or(GET_API_TIMEOUT),
        );
        api_timeout.insert(
            Api::PostMessage,
            self.api_timeout.remove(&Api::PostMessage).unwrap_or(GET_API_TIMEOUT),
        );
        api_timeout.insert(
            Api::PostMessageWithRemotePow,
            self.api_timeout
                .remove(&Api::PostMessageWithRemotePow)
                .unwrap_or(DEFAULT_REMOTE_POW_TIMEOUT),
        );
        api_timeout.insert(
            Api::GetOutput,
            self.api_timeout.remove(&Api::GetOutput).unwrap_or(GET_API_TIMEOUT),
        );

        #[cfg(feature = "mqtt")]
        let (mqtt_event_tx, mqtt_event_rx) = tokio::sync::watch::channel(MqttEvent::Connected);
        #[cfg(not(feature = "wasm"))]
        let network_info_ = network_info.read().await.clone();
        #[cfg(feature = "wasm")]
        let network_info_ = network_info.read().expect("Can't read network info").clone();
        let client = Client {
            node_manager: self.node_manager_builder.build(network_info_, sync.clone()).await?,
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
            request_timeout: self.request_timeout,
            api_timeout,
        };
        Ok(client)
    }
}
