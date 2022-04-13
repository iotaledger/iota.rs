// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{address::Address, payload::transaction::TransactionEssence};

use crate::signing::types::InputSigningData;

/// Helper struct for offline signing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedTransactionData {
    /// Transaction essence
    pub essence: TransactionEssence,
    /// Required address information for signing
    pub input_signing_data_entries: Vec<InputSigningData>,
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
