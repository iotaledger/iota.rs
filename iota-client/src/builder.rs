// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


//! Builder of the Clinet Instnace
use crate::{client::*, error::*, types::*};
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

/// Builder to construct client instance with sensible default values
pub struct ClientBuilder {
    nodes: HashSet<Url>,
    node_sync_interval: Duration,
    node_sync_enabled: bool,
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
            nodes: HashSet::new(),
            node_sync_interval: Duration::from_millis(60000),
            node_sync_enabled: true,
            network: Network::Testnet,
            #[cfg(feature = "mqtt")]
            broker_options: Default::default(),
            local_pow: true,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            api_timeout: {
                let mut api_default_timeout: HashMap<Api, Duration> = HashMap::new();
                api_default_timeout.insert(Api::GetInfo, Duration::from_millis(2000));
                api_default_timeout.insert(Api::GetHealth, Duration::from_millis(2000));
                api_default_timeout.insert(Api::GetMilestone, Duration::from_millis(2000));
                api_default_timeout.insert(Api::GetTips, Duration::from_millis(2000));
                api_default_timeout.insert(Api::PostMessage, Duration::from_millis(2000));
                api_default_timeout.insert(Api::PostMessageWithRemotePow, Duration::from_millis(30000));
                api_default_timeout.insert(Api::GetOutput, Duration::from_millis(2000));
                api_default_timeout
            }
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

    /// Get node list from the node_pool_urls
    pub fn with_node_pool_urls(mut self, node_pool_urls: &str) -> Result<Self> {
        let text: String = reqwest::blocking::get(node_pool_urls).unwrap().text().map_err(|_| Error::NodePoolUrlsError)?;
        let nodes_details: Vec<NodeDetail> = serde_json::from_str(&text).unwrap();
        for node_detail in nodes_details {
            let url = Url::parse(&node_detail.node).map_err(|_| Error::UrlError)?;
            self.nodes.insert(url);
        }
        Ok(self)
    }

    /// Selects the type of network the added nodes belong to.
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
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
        self.local_pow = local;
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

        let local_pow = self.local_pow;
        let network = self.network;
        let nodes = self.nodes;
        let node_sync_interval = self.node_sync_interval;

        let (runtime, sync, sync_kill_sender) = if self.node_sync_enabled {
            let sync = Arc::new(RwLock::new(HashSet::new()));
            let sync_ = sync.clone();
            let (sync_kill_sender, sync_kill_receiver) = channel(1);
            let runtime = std::thread::spawn(move || {
                let runtime = Runtime::new().unwrap();
                runtime.block_on(Client::sync_nodes(&sync_, &nodes, local_pow, network.clone()));
                Client::start_sync_process(
                    &runtime,
                    sync_,
                    nodes,
                    node_sync_interval,
                    local_pow,
                    network,
                    sync_kill_receiver,
                );
                runtime
            })
            .join()
            .expect("failed to init node syncing process");
            (Some(runtime), sync, Some(sync_kill_sender))
        } else {
            (None, Arc::new(RwLock::new(nodes)), None)
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
            local_pow: self.local_pow,
            request_timeout: self.request_timeout,
            api_timeout: self.api_timeout,
        };

        Ok(client)
    }
}
