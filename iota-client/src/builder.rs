// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Builder of the client instance

use crate::{client::*, error::*};

use reqwest::Url;
use tokio::{runtime::Runtime, sync::broadcast::channel};

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Duration,
};

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Network of the Iota nodes belong to
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Network {
    /// Mainnet
    Mainnet,
    /// Any network that is not the mainnet
    Testnet,
}

/// Struct containing network and PoW related information
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NetworkInfo {
    /// Network of the Iota nodes belong to
    pub network: Network,
    /// Network ID
    pub network_id: String,
    /// Mininum proof of work score
    pub min_pow_score: f64,
    /// Local proof of work
    pub local_pow: bool,
}

/// Builder to construct client instance with sensible default values
pub struct ClientBuilder {
    nodes: HashSet<Url>,
    node_sync_interval: Duration,
    node_sync_enabled: bool,
    #[cfg(feature = "mqtt")]
    broker_options: BrokerOptions,
    network_info: NetworkInfo,
    request_timeout: Duration,
    api_timeout: HashMap<Api, Duration>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            nodes: HashSet::new(),
            node_sync_interval: Duration::from_millis(60000),
            node_sync_enabled: true,
            #[cfg(feature = "mqtt")]
            broker_options: Default::default(),
            network_info: NetworkInfo {
                network: Network::Testnet,
                network_id: "alphanet1".into(),
                min_pow_score: 4000f64,
                local_pow: true,
            },
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
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
        let url = Url::parse(url).map_err(|_| Error::UrlError)?;
        self.nodes.insert(url);
        Ok(self)
    }

    /// Adds a list of IOTA nodes by their URLs.
    pub fn with_nodes(mut self, urls: &[&str]) -> Result<Self> {
        for url in urls {
            let url = Url::parse(url).map_err(|_| Error::UrlError)?;
            self.nodes.insert(url);
        }
        Ok(self)
    }

    /// Set the node sync interval
    pub fn with_node_sync_interval(mut self, node_sync_interval: Duration) -> Self {
        self.node_sync_interval = node_sync_interval;
        self
    }

    /// Disables the node syncing process.
    /// Every node will be considered healthy and ready to use.
    pub fn with_node_sync_disabled(mut self) -> Self {
        self.node_sync_enabled = false;
        self
    }

    // TODO node pool

    /// Selects the type of network the added nodes belong to.
    pub fn with_network(mut self, network: Network) -> Self {
        self.network_info.network = network;
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

    /// Sets the request timeout.
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
    pub fn finish(self) -> Result<Client> {
        if self.nodes.is_empty() {
            return Err(Error::MissingParameter(String::from("Iota node")));
        }

        let network_info = Arc::new(RwLock::new(self.network_info));
        let nodes = self.nodes;
        let node_sync_interval = self.node_sync_interval;

        let (runtime, sync, sync_kill_sender, network_info) = if self.node_sync_enabled {
            let sync = Arc::new(RwLock::new(HashSet::new()));
            let sync_ = sync.clone();
            let network_info_ = network_info.clone();
            let (sync_kill_sender, sync_kill_receiver) = channel(1);
            let runtime = std::thread::spawn(move || {
                let runtime = Runtime::new().unwrap();
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
            (None, Arc::new(RwLock::new(nodes)), None, network_info)
        };

        let client = Client {
            runtime,
            sync,
            sync_kill_sender: sync_kill_sender.map(Arc::new),
            client: reqwest::Client::new(),
            #[cfg(feature = "mqtt")]
            mqtt_client: None,
            #[cfg(feature = "mqtt")]
            mqtt_topic_handlers: Default::default(),
            #[cfg(feature = "mqtt")]
            broker_options: self.broker_options,
            network_info,
            request_timeout: self.request_timeout,
            api_timeout: self.api_timeout,
        };

        Ok(client)
    }
}
