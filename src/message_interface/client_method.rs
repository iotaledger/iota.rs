// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::signing::GenerateAddressMetadata;
use serde::Deserialize;
use std::ops::Range;

/// Options for generating addresses
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateAddressesOptions {
    pub coin_type: Option<u32>,
    pub account_index: Option<u32>,
    pub range: Option<Range<u32>>,
    pub bech32_hrp: Option<String>,
    pub metadata: Option<GenerateAddressMetadata>,
}

/// Each public client method.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "name", content = "data")]
pub enum ClientMethod {
    /// Generate a new unused address.
    GenerateAddresses {
        /// Create singer from json
        signer: String,
        /// Addresses generation options
        options: Option<GenerateAddressesOptions>,
    },
}
