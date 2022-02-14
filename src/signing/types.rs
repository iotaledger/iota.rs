// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Result;

use bee_message::{address::Address, output::OutputId, payload::transaction::TransactionId};
use bee_rest_api::types::responses::OutputResponse;
use crypto::keys::slip10::Chain;

use serde::{Deserialize, Serialize};

use std::str::FromStr;

/// The signer types.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum SignerType {
    /// Stronghold signer.
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Stronghold,
    /// Ledger Device
    #[cfg(feature = "ledger")]
    LedgerNano,
    /// Ledger Speculos Simulator
    #[cfg(feature = "ledger")]
    LedgerNanoSimulator,
    /// Mnemonic, not as secure as Stronghold or Ledger
    Mnemonic,
}

/// Metadata provided to [sign_message](trait.Signer.html#method.sign_message).
pub struct SignMessageMetadata<'a> {
    /// The transfer's remainder value.
    pub remainder_value: u64,
    /// The transfer's deposit address for the remainder value if any.
    pub remainder_deposit_address: Option<&'a AccountAddress>,
    /// The network which is used so the correct BIP32 path is used for the ledger. Debug mode starts with 44'/1' and
    /// in mainnet-mode it's 44'/4218'
    pub network: Network,
}

/// An account address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountAddress {
    /// The address.
    // todo: should we also use an address wrapper like in wallet.rs or the bech32 representation?
    pub address: Address,
    /// The address key index.
    #[serde(rename = "keyIndex")]
    pub key_index: u32,
    /// Determines if an address is a public or an internal (change) address.
    pub internal: bool,
}

/// Metadata provided to [generate_address](trait.Signer.html#method.generate_address).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateAddressMetadata {
    /// Indicates that the address is being generated as part of the account syncing process.
    /// This means that the account might not be saved.
    /// If it is false, the prompt will be displayed on ledger devices.
    pub syncing: bool,
    /// The network which is used so the correct BIP32 path is used for the ledger. Debug mode starts with 44'/1' and
    /// in mainnet-mode it's 44'/4218'
    pub network: Network,
}

/// Network enum for ledger metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Network {
    /// Mainnet
    Mainnet,
    /// Testnet
    Testnet,
}

/// The Ledger device status.
#[derive(Debug, Serialize)]
pub struct LedgerApp {
    /// Opened app name.
    pub(crate) name: String,
    /// Opened app version.
    pub(crate) version: String,
}

/// The Ledger device status.
#[derive(Debug, Serialize)]
pub struct LedgerStatus {
    /// Ledger is available and ready to be used.
    pub(crate) connected: bool,
    /// Ledger is connected and locked.
    pub(crate) locked: bool,
    /// Ledger opened app.
    pub(crate) app: Option<LedgerApp>,
}

/// Data for transaction inputs for signing and ordering of unlock blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSigningData {
    /// The output response
    pub output_response: OutputResponse,
    /// The chain derived from seed, only for ed25519 addresses
    pub chain: Option<Chain>,
    /// The bech32 encoded address, required because of alias outputs where we have multiple possible unlock
    /// conditions, because we otherwise don't know which one we need
    pub bech32_address: String,
}

impl InputSigningData {
    /// Return the [OutputId]
    pub fn output_id(&self) -> Result<OutputId> {
        Ok(OutputId::new(
            TransactionId::from_str(&self.output_response.transaction_id)?,
            self.output_response.output_index,
        )?)
    }
}

impl PartialEq for InputSigningData {
    fn eq(&self, other: &Self) -> bool {
        self.output_response.transaction_id == other.output_response.transaction_id
            && self.output_response.output_index == other.output_response.output_index
    }
}
