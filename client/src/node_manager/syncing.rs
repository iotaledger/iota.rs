// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_family = "wasm"))]
use {
    crate::NetworkInfo,
    iota_types::{api::core::response::InfoResponse, block::protocol::ProtocolParameters},
    std::collections::HashMap,
    std::{
        collections::HashSet,
        sync::{Arc, RwLock},
        time::Duration,
    },
    tokio::{runtime::Runtime, time::sleep},
};

use super::Node;
use crate::{Client, Error, Result};

impl Client {
    /// Get a node candidate from the healthy node pool.
    pub fn get_node(&self) -> Result<Node> {
        if let Some(primary_node) = &self.node_manager.primary_node {
            return Ok(primary_node.clone());
        }

        let pool = self.node_manager.nodes.clone();

        pool.into_iter().next().ok_or(Error::HealthyNodePoolEmpty)
    }

    /// returns the unhealthy nodes.
    #[cfg(not(target_family = "wasm"))]
    pub fn unhealthy_nodes(&self) -> HashSet<&Node> {
        self.node_manager
            .healthy_nodes
            .read()
            .map_or(HashSet::new(), |healthy_nodes| {
                self.node_manager
                    .nodes
                    .iter()
                    .filter(|node| !healthy_nodes.contains_key(node))
                    .collect()
            })
    }

    /// Sync the node lists per node_sync_interval milliseconds
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn start_sync_process(
        runtime: &Runtime,
        sync: Arc<RwLock<HashMap<Node, InfoResponse>>>,
        nodes: HashSet<Node>,
        node_sync_interval: Duration,
        network_info: Arc<RwLock<NetworkInfo>>,
        ignore_node_health: bool,
    ) -> tokio::task::JoinHandle<()> {
        runtime.spawn(async move {
            loop {
                // Delay first since the first `sync_nodes` call is made by the builder to ensure the node list is
                // filled before the client is used.
                sleep(node_sync_interval).await;
                if let Err(e) = Self::sync_nodes(&sync, &nodes, &network_info, ignore_node_health).await {
                    log::warn!("Syncing nodes failed: {e}");
                }
            }
        })
    }

    #[cfg(not(target_family = "wasm"))]
    pub(crate) async fn sync_nodes(
        sync: &Arc<RwLock<HashMap<Node, InfoResponse>>>,
        nodes: &HashSet<Node>,
        network_info: &Arc<RwLock<NetworkInfo>>,
        ignore_node_health: bool,
    ) -> Result<()> {
        log::debug!("sync_nodes");
        let mut healthy_nodes = HashMap::new();
        let mut network_nodes: HashMap<String, Vec<(InfoResponse, Node)>> = HashMap::new();

        for node in nodes {
            // Put the healthy node url into the network_nodes
            if let Ok(info) = Self::get_node_info(node.url.as_ref(), node.auth.clone()).await {
                if info.status.is_healthy || ignore_node_health {
                    match network_nodes.get_mut(&info.protocol.network_name) {
                        Some(network_node_entry) => {
                            network_node_entry.push((info, node.clone()));
                        }
                        None => {
                            network_nodes.insert(info.protocol.network_name.clone(), vec![(info, node.clone())]);
                        }
                    }
                } else {
                    log::debug!("{} is not healthy: {:?}", node.url, info);
                }
            } else {
                log::error!("Couldn't get the node info from {}", node.url);
            }
        }

        // Get network_id with the most nodes
        let mut most_nodes = ("network_id", 0);
        for (network_id, node) in &network_nodes {
            if node.len() > most_nodes.1 {
                most_nodes.0 = network_id;
                most_nodes.1 = node.len();
            }
        }

        if let Some(nodes) = network_nodes.get(most_nodes.0) {
            if let Some((info, _node_url)) = nodes.first() {
                let mut network_info = network_info.write().map_err(|_| crate::Error::PoisonError)?;

                network_info.latest_milestone_timestamp = info.status.latest_milestone.timestamp;
                network_info.protocol_parameters = ProtocolParameters::try_from(info.protocol.clone())?;
            }

            for (info, node_url) in nodes {
                healthy_nodes.insert(node_url.clone(), info.clone());
            }
        }

        // Update the sync list.
        *sync.write().map_err(|_| crate::Error::PoisonError)? = healthy_nodes;

        Ok(())
    }
}
