//! Builder of the Clinet Instnace

use crate::client::Client;
use crate::error::*;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::{Arc, RwLock};

use reqwest::Url;

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
    network: Network,
    quorum_size: u8,
    quorum_threshold: u8,
}

impl ClientBuilder {
    /// Create an Iota client builder
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            network: Network::Mainnet,
            quorum_size: 3,
            quorum_threshold: 50,
        }
    }

    /// Add a Iota node
    pub fn node(mut self, url: &str) -> Result<Self> {
        let url = Url::parse(url).map_err(|_| Error::UrlError)?;
        self.nodes.push(url);
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

    /// Build the Client instance.
    pub fn build(self) -> Result<Client> {
        // if self.nodes.len() == 0 {
        //     return Err(Error::MissingNode);
        // }

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

        let client = Client {
            pool: Arc::new(RwLock::new(HashSet::from_iter(self.nodes.into_iter()))),
            sync: Arc::new(RwLock::new(Vec::new())),
            client: reqwest::Client::new(),
            mwm,
            quorum_size,
            quorum_threshold,
        };

        //let mut sync = client.clone();
        //tokio::block_on(async { sync.sync() });

        tokio::spawn(async {
            loop {
                tokio::time::delay_for(std::time::Duration::from_secs(180)).await;
                //sync.sync();
            }
        });

        Ok(client)
    }
}
