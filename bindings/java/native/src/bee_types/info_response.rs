// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use iota_client::{
    bee_rest_api::types::responses::InfoResponse as RustInfoResponse, client::NodeInfoWrapper as RustNodeInfoWrapper,
};

#[derive(PartialEq, Getters, CopyGetters)]
pub struct NodeInfoWrapper {
    #[getset(get = "pub")]
    url: String,
    nodeinfo: InfoResponse,
}

impl NodeInfoWrapper {
    pub fn nodeinfo(&self) -> InfoResponse {
        self.nodeinfo.clone()
    }
}

impl core::fmt::Display for NodeInfoWrapper {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "url: {}, nodeinfo: {}", self.url, self.nodeinfo)
    }
}

impl core::fmt::Debug for NodeInfoWrapper {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "NodeInfoWrapper({self})")
    }
}

impl From<RustNodeInfoWrapper> for NodeInfoWrapper {
    fn from(info: RustNodeInfoWrapper) -> Self {
        Self {
            url: info.url,
            nodeinfo: InfoResponse {
                name: info.nodeinfo.name,
                version: info.nodeinfo.version,
                is_healthy: info.nodeinfo.is_healthy,
                network_id: info.nodeinfo.network_id,
                bech32_hrp: info.nodeinfo.bech32_hrp,
                min_pow_score: info.nodeinfo.min_pow_score,
                messages_per_second: info.nodeinfo.messages_per_second,
                referenced_messages_per_second: info.nodeinfo.referenced_messages_per_second,
                referenced_rate: info.nodeinfo.referenced_rate,
                latest_milestone_timestamp: info.nodeinfo.latest_milestone_timestamp,
                latest_milestone_index: info.nodeinfo.latest_milestone_index,
                confirmed_milestone_index: info.nodeinfo.confirmed_milestone_index,
                pruning_index: info.nodeinfo.pruning_index,
                features: info.nodeinfo.features,
            },
        }
    }
}

#[derive(Clone, PartialEq, Getters, CopyGetters)]
pub struct InfoResponse {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    version: String,
    #[getset(get_copy = "pub")]
    is_healthy: bool,
    #[getset(get = "pub")]
    network_id: String,
    #[getset(get = "pub")]
    bech32_hrp: String,
    #[getset(get_copy = "pub")]
    min_pow_score: f64,
    #[getset(get_copy = "pub")]
    messages_per_second: f64,
    #[getset(get_copy = "pub")]
    referenced_messages_per_second: f64,
    #[getset(get_copy = "pub")]
    referenced_rate: f64,
    #[getset(get_copy = "pub")]
    latest_milestone_timestamp: u64,
    #[getset(get_copy = "pub")]
    latest_milestone_index: u32,
    #[getset(get_copy = "pub")]
    confirmed_milestone_index: u32,
    #[getset(get_copy = "pub")]
    pruning_index: u32,
    features: Vec<String>,
}

impl InfoResponse {
    pub fn features(&self) -> Vec<String> {
        self.features.to_vec()
    }
}

impl core::fmt::Display for InfoResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "name={}, version={}, is_healthy={}, network_id={}, bech32_hrp={}, min_pow_score={}
            , messages_per_second={}, referenced_messages_per_second={}, referenced_rate={}
            , latest_milestone_timestamp={}, latest_milestone_index={}, confirmed_milestone_index={}
            , pruning_index={}, features=({:?})",
            self.name,
            self.version,
            self.is_healthy,
            self.network_id,
            self.bech32_hrp,
            self.min_pow_score,
            self.messages_per_second,
            self.referenced_messages_per_second,
            self.referenced_rate,
            self.latest_milestone_timestamp,
            self.latest_milestone_index,
            self.confirmed_milestone_index,
            self.pruning_index,
            self.features
        )
    }
}

impl core::fmt::Debug for InfoResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "InfoResponse({self})")
    }
}

impl From<RustInfoResponse> for InfoResponse {
    fn from(info: RustInfoResponse) -> Self {
        Self {
            name: info.name,
            version: info.version,
            is_healthy: info.is_healthy,
            network_id: info.network_id,
            bech32_hrp: info.bech32_hrp,
            messages_per_second: info.messages_per_second,
            referenced_messages_per_second: info.referenced_messages_per_second,
            referenced_rate: info.referenced_rate,
            latest_milestone_timestamp: info.latest_milestone_timestamp,
            latest_milestone_index: info.latest_milestone_index,
            confirmed_milestone_index: info.confirmed_milestone_index,
            pruning_index: info.pruning_index,
            features: info.features,
            min_pow_score: info.min_pow_score,
        }
    }
}
