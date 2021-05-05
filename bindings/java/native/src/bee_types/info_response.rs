// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use iota_client::{
    bee_rest_api::types::responses::InfoResponse as RustInfoResponse,
};

impl From<RustInfoResponse> for NodeInfo {
    fn from(info: RustInfoResponse) -> Self {
        Self {
            name: info.name,
            version: info.version,
            is_healthy: info.is_healthy,
            network_id: info.network_id,
            bech32_hrp: info.bech32_hrp,
            min_pow_score: info.min_pow_score,
            latest_milestone_index: info.latest_milestone_index,
            confirmed_milestone_index: info.confirmed_milestone_index,
            pruning_index: info.pruning_index,
            features: info.features,
        }
    }
}

#[derive(Clone, PartialEq, Getters, CopyGetters)]
pub struct NodeInfo {
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
    latest_milestone_index: u32,
    #[getset(get_copy = "pub")]
    confirmed_milestone_index: u32,
    #[getset(get_copy = "pub")]
    pruning_index: u32,
    features: Vec<String>,
}

impl NodeInfo {
    pub fn features(&self) -> Vec<String> {
        self.features.to_vec()
    }
}