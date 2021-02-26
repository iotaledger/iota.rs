// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod error;
pub mod full_node_api;
pub mod high_level_api;
pub mod mqtt;
pub mod types;
use iota::{Api, BrokerOptions as RustBrokerOptions, Client as RustClient};
use pyo3::prelude::*;
use std::{collections::HashMap, time::Duration};
use types::{
    AddressBalancePair, BalanceForAddressResponse, BrokerOptions, InfoResponse, Input, Message,
    MessageMetadataResponse, MilestoneDto, MilestoneUTXOChanges, Output, OutputResponse, PeerDto, UTXOInput,
    BECH32_HRP,
};

/// Client builder
#[pyclass]
pub struct Client {
    pub client: RustClient,
}

/// An instance of the client using IOTA node URI.
#[pymethods]
impl Client {
    #[new]
    #[allow(clippy::too_many_arguments)]
    /// The constructor of the client instance.
    fn new(
        network: Option<&str>,
        node: Option<&str>,
        nodes: Option<Vec<&str>>,
        node_sync_interval: Option<u64>,
        node_sync_disabled: Option<bool>,
        node_pool_urls: Option<Vec<String>>,
        request_timeout: Option<u64>,
        api_timeout: Option<HashMap<&str, u64>>,
        local_pow: Option<bool>,
        tips_interval: Option<u64>,
        mqtt_broker_options: Option<BrokerOptions>,
    ) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut client = RustClient::builder();
        if let Some(network) = network {
            client = client.with_network(network);
        }
        if let Some(node) = node {
            client = client.with_node(node).unwrap();
        }
        if let Some(nodes) = nodes {
            client = client.with_nodes(&nodes).unwrap();
        }
        if let Some(node_sync_interval) = node_sync_interval {
            client = client.with_node_sync_interval(Duration::from_millis(node_sync_interval));
        }
        if let Some(node_sync_disabled) = node_sync_disabled {
            if node_sync_disabled {
                client = client.with_node_sync_disabled();
            }
        }
        if let Some(node_pool_urls) = node_pool_urls {
            client = rt.block_on(async { client.with_node_pool_urls(&node_pool_urls).await.unwrap() });
        }
        if let Some(timeout) = request_timeout {
            client = client.with_request_timeout(Duration::from_millis(timeout));
        }
        if let Some(api_timeout) = api_timeout {
            for (api, timeout) in api_timeout {
                match api {
                    "GetHealth" => client = client.with_api_timeout(Api::GetHealth, Duration::from_millis(timeout)),
                    "GetInfo" => client = client.with_api_timeout(Api::GetInfo, Duration::from_millis(timeout)),
                    "GetPeers" => client = client.with_api_timeout(Api::GetPeers, Duration::from_millis(timeout)),
                    "GetTips" => client = client.with_api_timeout(Api::GetTips, Duration::from_millis(timeout)),
                    "PostMessage" => client = client.with_api_timeout(Api::PostMessage, Duration::from_millis(timeout)),
                    "GetOutput" => client = client.with_api_timeout(Api::GetOutput, Duration::from_millis(timeout)),
                    "GetMilestone" => {
                        client = client.with_api_timeout(Api::GetMilestone, Duration::from_millis(timeout))
                    }
                    _ => (),
                }
            }
        }
        if let Some(local_pow) = local_pow {
            client = client.with_local_pow(local_pow);
        }
        if let Some(tips_interval) = tips_interval {
            client = client.with_tips_interval(tips_interval);
        }
        if let Some(broker_options) = mqtt_broker_options {
            let rust_broker_options = RustBrokerOptions::new()
                .automatic_disconnect(broker_options.automatic_disconnect)
                .timeout(Duration::from_millis(broker_options.timeout))
                .use_websockets(broker_options.use_ws);
            client = client.with_mqtt_broker_options(rust_broker_options);
        }
        let client = rt.block_on(async { client.finish().await.unwrap() });

        // Update the BECH32_HRP
        // Note: This unsafe code is actually safe, because the BECH32_HRP will be only initialized when we
        //       create the client object.
        let bech32_hrp = rt.block_on(async { client.get_bech32_hrp().await.unwrap() });
        unsafe {
            BECH32_HRP = Box::leak(bech32_hrp.into_boxed_str());
        }
        Client { client }
    }
}
