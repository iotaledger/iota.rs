// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// The error type exposed to users
pub mod error;
/// Full node API binding
pub mod full_node_api;
/// High-level API binding
pub mod high_level_api;
/// MQTT binding
pub mod mqtt;
/// Type casting between Rust and Python
pub mod types;
use iota_client::{Api, BrokerOptions as RustBrokerOptions, Client as RustClient};
use pyo3::prelude::*;
use std::{collections::HashMap, time::Duration};
use types::{
    AddressBalancePair, AddressDto, AddressOutputsOptions, BalanceAddressResponse, BrokerOptions, Message,
    MessageMetadataResponse, MilestoneDto, MilestoneUTXOChanges, NodeInfoWrapper, Output, OutputDto, OutputResponse,
    Payload, PeerDto, PreparedTransactionData, ReceiptDto, TreasuryResponse, UtxoInput, BECH32_HRP,
};

/// Client builder
#[pyclass]
pub struct Client {
    /// The Client structure in native rust
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
        primary_node_jwt_name_password: Option<Vec<&str>>,
        primary_pow_node_jwt_name_password: Option<Vec<&str>>,
        nodes_name_password: Option<Vec<Vec<&str>>>,
        permanodes_name_password: Option<Vec<Vec<&str>>>,
        node_sync_interval: Option<u64>,
        node_sync_disabled: Option<bool>,
        offline: Option<bool>,
        node_pool_urls: Option<Vec<String>>,
        quorum: Option<bool>,
        quorum_size: Option<usize>,
        quorum_threshold: Option<usize>,
        request_timeout: Option<u64>,
        api_timeout: Option<HashMap<&str, u64>>,
        local_pow: Option<bool>,
        tips_interval: Option<u64>,
        mqtt_broker_options: Option<BrokerOptions>,
    ) -> Self {
        let mut client = RustClient::builder();
        if let Some(network) = network {
            client = client.with_network(network);
        }
        if let Some(input) = primary_node_jwt_name_password {
            let (url, jwt, basic_auth_name_pwd) = get_url_jwt_auth(input);
            if let Some(url) = url {
                client = client.with_primary_node(url, jwt, basic_auth_name_pwd).unwrap();
            }
        }
        if let Some(input) = primary_pow_node_jwt_name_password {
            let (url, jwt, basic_auth_name_pwd) = get_url_jwt_auth(input);
            if let Some(url) = url {
                client = client.with_primary_pow_node(url, jwt, basic_auth_name_pwd).unwrap();
            }
        }
        if let Some(nodes_name_password) = nodes_name_password {
            for input in nodes_name_password {
                let (url, jwt, basic_auth_name_pwd) = get_url_jwt_auth(input);
                if let Some(url) = url {
                    client = client.with_node_auth(url, jwt, basic_auth_name_pwd).unwrap();
                }
            }
        }
        if let Some(permanodes_name_password) = permanodes_name_password {
            for input in permanodes_name_password {
                let (url, jwt, basic_auth_name_pwd) = get_url_jwt_auth(input);
                if let Some(url) = url {
                    client = client.with_permanode(url, jwt, basic_auth_name_pwd).unwrap();
                }
            }
        }
        if let Some(node_sync_interval) = node_sync_interval {
            client = client.with_node_sync_interval(Duration::from_millis(node_sync_interval));
        }
        if let Some(node_sync_disabled) = node_sync_disabled {
            if node_sync_disabled {
                client = client.with_node_sync_disabled();
            }
        }
        if let Some(offline) = offline {
            if offline {
                client = client.with_offline_mode();
            }
        }
        if let Some(node_pool_urls) = node_pool_urls {
            client = crate::block_on(async { client.with_node_pool_urls(&node_pool_urls).await.unwrap() });
        }
        if let Some(enabled) = quorum {
            client = client.with_quorum(enabled);
        }
        if let Some(quorum_size) = quorum_size {
            client = client.with_quorum_size(quorum_size);
        }
        if let Some(quorum_threshold) = quorum_threshold {
            client = client.with_quorum_threshold(quorum_threshold);
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
                .timeout(Duration::from_secs(broker_options.timeout))
                .use_ws(broker_options.use_ws)
                .port(broker_options.port)
                .max_reconnection_attempts(broker_options.max_reconnection_attempts);
            client = client.with_mqtt_broker_options(rust_broker_options);
        }
        let client = crate::block_on(async { client.finish().await.unwrap() });

        // If not in the offline mode
        if offline != Some(true) {
            // Update the BECH32_HRP
            // Note: This unsafe code is actually safe, because the BECH32_HRP will be only initialized when we
            //       create the client object.
            let bech32_hrp = crate::block_on(async { client.get_bech32_hrp().await.unwrap() });
            // Note that mutable static is unsafe and requires unsafe function or block
            unsafe {
                BECH32_HRP = Box::leak(bech32_hrp.into_boxed_str());
            }
        }
        Client { client }
    }
}

// helper function to get the provided options for a node
fn get_url_jwt_auth(input: Vec<&str>) -> (Option<&str>, Option<String>, Option<(&str, &str)>) {
    if input.len() == 1 {
        // node url alone
        (Some(input[0]), None, None)
    } else if input.len() == 2 {
        // node url with jwt
        (Some(input[0]), Some(input[1].to_string()), None)
    } else if input.len() == 3 {
        // node url with basic auth
        (Some(input[0]), None, Some((input[1], input[2])))
    } else if input.len() >= 4 {
        // node url with jwt and basic auth
        (Some(input[0]), Some(input[1].to_string()), Some((input[2], input[3])))
    } else {
        (None, None, None)
    }
}
