// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{address::Address, input::Input, payload::transaction::TransactionEssence};
use bee_rest_api::types::responses::OutputResponse;
use crypto::keys::slip10::Chain;

/// Helper struct for offline signing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedTransactionData {
    /// Transaction essence
    pub essence: TransactionEssence,
    /// Required address information for signing
    pub address_index_recorders: Vec<AddressIndexRecorder>,
}

/// Structure for sorting of UnlockBlocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressIndexRecorder {
    /// Index of the account
    pub account_index: u32,
    /// The input used
    pub input: Input,
    /// The output information
    pub output: OutputResponse,
    /// index of this address on the seed
    pub address_index: u32,
    /// The chain derived from seed
    pub chain: Chain,
    /// Whether this is an internal address
    pub internal: bool,
    /// The address
    pub bech32_address: String,
}

#[derive(Debug, Clone)]
pub(crate) struct OutputWrapper {
    pub(crate) output: OutputResponse,
    pub(crate) address_index: u32,
    pub(crate) internal: bool,
    pub(crate) amount: u64,
    pub(crate) address: String,
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
