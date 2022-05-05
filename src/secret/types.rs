// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Miscellaneous types for secret managers.

use std::str::FromStr;

use bee_message::{
    address::Address,
    output::{Output, OutputId},
    payload::transaction::TransactionId,
    MessageId,
};
use bee_rest_api::types::responses::OutputResponse;
use crypto::keys::slip10::Chain;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Stronghold DTO to allow the creation of a Stronghold secret manager from bindings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg(feature = "stronghold")]
pub struct StrongholdDto {
    /// The Stronghold password
    pub password: Option<String>,
    /// The path for the Stronghold file
    #[serde(rename = "snapshotPath")]
    pub snapshot_path: Option<String>,
}

/// Metadata provided to [SecretManager::signature_unlock()](super::SecretManager::signature_unlock()).
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

/// Metadata provided to `generate_address()`.
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

///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMetadata {
    /// The identifier of the message in which the output was included.
    #[serde(rename = "messageId")]
    pub message_id: MessageId,
    /// The identifier of the transaction in which the output was included.
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
    /// The index of the output in the transaction.
    #[serde(rename = "outputIndex")]
    pub output_index: u16,
    /// Whether the output is spent or not.
    #[serde(rename = "isSpent")]
    pub is_spent: bool,
    /// If spent, the index of the milestone in which the output was spent.
    #[serde(rename = "milestoneIndexSpent", skip_serializing_if = "Option::is_none")]
    pub milestone_index_spent: Option<u32>,
    /// If spent, the timestamp of the milestone in which the output was spent.
    #[serde(rename = "milestoneTimestampSpent", skip_serializing_if = "Option::is_none")]
    pub milestone_timestamp_spent: Option<u32>,
    /// If spent, the identifier of the transaction that spent the output.
    #[serde(rename = "transactionIdSpent", skip_serializing_if = "Option::is_none")]
    pub transaction_id_spent: Option<TransactionId>,
    /// The index of the milestone that booked the output.
    #[serde(rename = "milestoneIndexBooked")]
    pub milestone_index_booked: u32,
    /// The timestamp of the milestone that booked the output.
    #[serde(rename = "milestoneTimestampBooked")]
    pub milestone_timestamp_booked: u32,
    /// The index of ledger when the output was fetched.
    #[serde(rename = "ledgerIndex", default)]
    pub ledger_index: u32,
}

impl TryFrom<&OutputResponse> for OutputMetadata {
    type Error = Error;

    fn try_from(response: &OutputResponse) -> Result<Self> {
        Ok(OutputMetadata {
            message_id: MessageId::from_str(&response.message_id)?,
            transaction_id: TransactionId::from_str(&response.transaction_id)?,
            output_index: response.output_index,
            is_spent: response.is_spent,
            milestone_index_spent: response.milestone_index_spent,
            milestone_timestamp_spent: response.milestone_timestamp_spent,
            transaction_id_spent: response
                .transaction_id_spent
                .as_ref()
                .map(|s| TransactionId::from_str(s))
                .transpose()?,
            milestone_index_booked: response.milestone_index_booked,
            milestone_timestamp_booked: response.milestone_timestamp_booked,
            ledger_index: response.ledger_index,
        })
    }
}

/// Data for transaction inputs for signing and ordering of unlock blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSigningData {
    /// The output
    pub output: Output,
    /// The output metadata
    #[serde(rename = "outputMetadata")]
    pub output_metadata: OutputMetadata,
    /// The chain derived from seed, only for ed25519 addresses
    pub chain: Option<Chain>,
    /// The bech32 encoded address, required because of alias outputs where we have multiple possible unlock
    /// conditions, because we otherwise don't know which one we need
    #[serde(rename = "bech32Address")]
    pub bech32_address: String,
}

impl InputSigningData {
    /// Return the [OutputId]
    pub fn output_id(&self) -> Result<OutputId> {
        Ok(OutputId::new(
            self.output_metadata.transaction_id,
            self.output_metadata.output_index,
        )?)
    }
}

impl PartialEq for InputSigningData {
    fn eq(&self, other: &Self) -> bool {
        self.output_metadata.transaction_id == other.output_metadata.transaction_id
            && self.output_metadata.output_index == other.output_metadata.output_index
    }
}