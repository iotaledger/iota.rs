// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Builder of the Clinet Instnace

use crate::{
    client::*,
    error::*,
};

use reqwest::Url;
use tokio::{runtime::Runtime, sync::broadcast::channel};

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
    sync::{Arc, RwLock},
    time::Duration,
};

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Network of the Iota nodes belong to
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Network {
    /// Mainnet
    Mainnet,
    /// Devnet
    Devnet,
    /// Comnet
    Comnet,
}

/// Builder to construct client instance with sensible default values
pub struct ClientBuilder {
    nodes: Vec<Url>,
    node_sync_interval: NonZeroU64,
    network: Network,
    #[cfg(feature = "mqtt")]
    broker_options: BrokerOptions,
    local_pow: bool,
    request_timeout: Duration,
    api_timeout: HashMap<Api, Duration>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            node_sync_interval: NonZeroU64::new(60000).unwrap(),
            network: Network::Mainnet,
            #[cfg(feature = "mqtt")]
            broker_options: Default::default(),
            local_pow: true,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            api_timeout: Default::default(),
        }
    }
}

impl ClientBuilder {
    /// Create an Iota client builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a Iota node
    pub fn node(mut self, url: &str) -> Result<Self> {
        let url = Url::parse(url).map_err(|_| Error::UrlError)?;
        self.nodes.push(url);
        Ok(self)
    }

    /// Set the node sync interval
    pub fn node_sync_interval(mut self, node_sync_interval: NonZeroU64) -> Result<Self> {
        self.node_sync_interval = node_sync_interval;
        Ok(self)
    }

    /// Add a list of Iota nodes
    pub fn nodes(mut self, urls: &[&str]) -> Result<Self> {
        for url in urls {
            let url = Url::parse(url).map_err(|_| Error::UrlError)?;
            self.nodes.push(url);
        }
        Ok(self)
    }

    // TODO node pool

    /// Network of the Iota nodes belong to
    pub fn network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Sets the MQTT broker options.
    #[cfg(feature = "mqtt")]
    pub fn broker_options(mut self, options: BrokerOptions) -> Self {
        self.broker_options = options;
        self
    }

    /// Whether the PoW should be local or remote
    pub fn local_pow(mut self, local: bool) -> Self {
        self.local_pow = local;
        self
    }

    /// Sets the request timeout.
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Sets the request timeout for a specific API usage.
    pub fn api_timeout(mut self, api: Api, timeout: Duration) -> Self {
        self.api_timeout.insert(api, timeout);
        self
    }

    /// Build the Client instance.
    pub fn build(self) -> Result<Client> {
        if self.nodes.is_empty() {
            return Err(Error::MissingParameter(String::from("Iota node")));
        }

        let nodes = self.nodes;
        let node_sync_interval = self.node_sync_interval;

        let sync = Arc::new(RwLock::new(HashSet::new()));
        let sync_ = sync.clone();

        let (sync_kill_sender, sync_kill_receiver) = channel(1);

        let runtime = std::thread::spawn(move || {
            let mut runtime = Runtime::new().unwrap();
            runtime.block_on(Client::sync_nodes(&sync_, &nodes));
            Client::start_sync_process(&runtime, sync_, nodes, node_sync_interval, sync_kill_receiver);
            runtime
        })
        .join()
        .expect("failed to init node syncing process");

        let client = Client {
            runtime: Some(runtime),
            sync,
            sync_kill_sender: Arc::new(sync_kill_sender),
            client: reqwest::Client::new(),
            #[cfg(feature = "mqtt")]
            mqtt_client: None,
            #[cfg(feature = "mqtt")]
            mqtt_topic_handlers: Default::default(),
            #[cfg(feature = "mqtt")]
            broker_options: self.broker_options,
            local_pow: self.local_pow,
            request_timeout: self.request_timeout,
            api_timeout: self.api_timeout,
        };

        Ok(client)
    }
}
