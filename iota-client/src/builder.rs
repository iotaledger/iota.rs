// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Builder of the Clinet Instnace

use crate::{
    client::{BrokerOptions, Client},
    error::*,
};

use reqwest::Url;
use tokio::{runtime::Runtime, sync::broadcast::channel};

use std::{
    collections::HashSet,
    num::NonZeroU64,
    sync::{Arc, RwLock},
};

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
    broker_options: BrokerOptions,
    local_pow: bool,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            node_sync_interval: NonZeroU64::new(60000).unwrap(),
            network: Network::Mainnet,
            broker_options: Default::default(),
            local_pow: true,
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
    pub fn broker_options(mut self, options: BrokerOptions) -> Self {
        self.broker_options = options;
        self
    }

    /// Whether the PoW should be local or remote
    pub fn local_pow(mut self, local: bool) -> Self {
        self.local_pow = local;
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
            mqtt_client: None,
            mqtt_topic_handlers: Default::default(),
            broker_options: self.broker_options,
            local_pow: self.local_pow,
        };

        Ok(client)
    }
}
