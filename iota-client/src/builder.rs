//! Builder of the Clinet Instnace

use crate::client::{BrokerOptions, Client};
use crate::error::*;

use reqwest::Url;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::num::NonZeroU64;
use std::sync::{atomic::AtomicBool, Arc, Mutex, RwLock};

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
    quorum_size: u8,
    quorum_threshold: u8,
    broker_options: BrokerOptions,
}

impl ClientBuilder {
    /// Create an Iota client builder
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_sync_interval: NonZeroU64::new(60000).unwrap(),
            network: Network::Mainnet,
            quorum_size: 3,
            quorum_threshold: 50,
            broker_options: Default::default(),
        }
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

    /// Quorum size defines how many of nodes will be queried at the same time to check for quorum
    pub fn quorum_size(mut self, size: u8) -> Self {
        self.quorum_size = size;
        self
    }

    /// The quorum threshold defines the minimum amount of nodes from the quorum pool that need to agree if we want to
    /// consider the result true. The default is 50 meaning at least 50% of the nodes need to agree. (so at least 2 out
    /// of 3 nodes when the quorum size is 3).
    pub fn quorum_threshold(mut self, threshold: u8) -> Self {
        self.quorum_threshold = threshold;
        self
    }

    /// Sets the MQTT broker options.
    pub fn broker_options(mut self, options: BrokerOptions) -> Self {
        self.broker_options = options;
        self
    }

    /// Build the Client instance.
    pub fn build(self) -> Result<Client> {
        if self.nodes.is_empty() {
            return Err(Error::MissingParameter(String::from("Iota node")));
        }

        let mwm = match self.network {
            Network::Mainnet => 14,
            Network::Comnet => 10,
            Network::Devnet => 9,
        };

        let quorum_size = match self.nodes.len() {
            1 => 1,
            _ => self.quorum_size,
        };

        let quorum_threshold = match self.quorum_threshold {
            100..=255 => 100,
            x => x,
        };

        let mut client = Client {
            nodes: self.nodes.clone(),
            pool: Arc::new(RwLock::new(HashSet::from_iter(self.nodes.into_iter()))),
            sync: Arc::new(Mutex::new(Vec::new())),
            stop_sync: Arc::new(AtomicBool::new(false)),
            sync_handle: None,
            node_sync_interval: self.node_sync_interval,
            client: reqwest::Client::new(),
            mwm,
            quorum_size,
            quorum_threshold,
            mqtt_client: None,
            mqtt_topic_handlers: Default::default(),
            broker_options: self.broker_options,
        };

        // let mut sync = client.clone();
        // tokio::block_on(async { sync.sync() });

        // tokio::spawn(async {
        //     loop {
        //         tokio::time::delay_for(std::time::Duration::from_secs(180)).await;
        //         sync.sync();
        //     }
        // });
        client.start_sync_process();
        Ok(client)
    }
}
