// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::signing::types::InputSigningData;

use bee_message::{address::Address, payload::transaction::TransactionEssence};
use bee_rest_api::types::responses::OutputResponse;

/// Helper struct for offline signing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedTransactionData {
    /// Transaction essence
    pub essence: TransactionEssence,
    /// Required address information for signing
    pub input_signing_data_entrys: Vec<InputSigningData>,
}

#[derive(Debug, Clone)]
pub(crate) struct OutputWrapper {
    pub(crate) output: OutputResponse,
    pub(crate) address_index: u32,
    pub(crate) internal: bool,
    pub(crate) amount: u64,
    pub(crate) bech32_address: String,
}

/// Generated addresses
#[derive(Debug, Clone)]
pub struct RawAddresses {
    /// Public addresses
    pub public: Vec<Address>,
    /// Internal/change addresses https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki#change
    pub internal: Vec<Address>,
}

/// Generated addresses bech32 encoded
#[derive(Debug, Clone)]
pub struct Bech32Addresses {
    /// Public addresses
    pub public: Vec<String>,
    /// Internal/change addresses https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki#change
    pub internal: Vec<String>,
}
