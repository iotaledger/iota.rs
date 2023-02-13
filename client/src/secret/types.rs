// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Miscellaneous types for secret managers.

use crypto::keys::slip10::Chain;
use iota_types::block::{
    address::Address,
    output::{
        dto::{OutputDto, OutputMetadataDto},
        Output, OutputId, OutputMetadata,
    },
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "stronghold")]
use zeroize::ZeroizeOnDrop;

use crate::Result;

/// Stronghold DTO to allow the creation of a Stronghold secret manager from bindings
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ZeroizeOnDrop)]
#[cfg(feature = "stronghold")]
#[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
pub struct StrongholdDto {
    /// The Stronghold password
    pub password: Option<String>,
    /// The timeout for auto key clearing, in seconds
    pub timeout: Option<u64>,
    /// The path for the Stronghold file
    #[serde(rename = "snapshotPath")]
    pub snapshot_path: String,
}
/// An account address.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccountAddress {
    /// The address.
    pub address: Address,
    /// The address key index.
    #[serde(rename = "keyIndex")]
    pub key_index: u32,
    /// Determines if an address is a public or an internal (change) address.
    pub internal: bool,
}

/// Options provided to `generate_address()`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GenerateAddressOptions {
    /// Display the address on ledger devices.
    #[serde(rename = "ledgerNanoPrompt")]
    pub ledger_nano_prompt: bool,
}

/// The Ledger device status.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LedgerApp {
    /// Opened app name.
    pub(crate) name: String,
    /// Opened app version.
    pub(crate) version: String,
}

impl LedgerApp {
    /// Opened app name.
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Opened app version.
    pub fn version(&self) -> &String {
        &self.version
    }
}

/// Ledger Device Type
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LedgerDeviceType {
    /// Device Type Nano S
    #[serde(alias = "ledgerNanoS")]
    LedgerNanoS,
    /// Device Type Nano X
    #[serde(alias = "ledgerNanoX")]
    LedgerNanoX,
    /// Device Type Nano S Plus
    #[serde(alias = "ledgerNanoSPlus")]
    LedgerNanoSPlus,
}

/// The Ledger device status.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LedgerNanoStatus {
    /// Ledger is available and ready to be used.
    pub(crate) connected: bool,
    /// Ledger is connected and locked.
    pub(crate) locked: bool,
    /// Ledger blind signing enabled
    #[serde(rename = "blindSigningEnabled")]
    pub(crate) blind_signing_enabled: bool,
    /// Ledger opened app.
    pub(crate) app: Option<LedgerApp>,
    /// Ledger device
    pub(crate) device: Option<LedgerDeviceType>,
    /// Buffer size on device
    #[serde(rename = "bufferSize")]
    pub(crate) buffer_size: Option<usize>,
}

impl LedgerNanoStatus {
    /// Ledger is available and ready to be used.
    pub fn connected(&self) -> bool {
        self.connected
    }
    /// Ledger is connected and locked.
    pub fn locked(&self) -> bool {
        self.locked
    }
    /// Ledger blind signing enabled
    pub fn blind_signing_enabled(&self) -> bool {
        self.blind_signing_enabled
    }
    /// Ledger opened app.
    pub fn app(&self) -> Option<&LedgerApp> {
        self.app.as_ref()
    }
    /// Ledger device
    pub fn device(&self) -> Option<LedgerDeviceType> {
        self.device
    }
    /// Buffer size on device
    pub fn buffer_size(&self) -> Option<usize> {
        self.buffer_size
    }
}

/// Data for transaction inputs for signing and ordering of unlock blocks
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InputSigningData {
    /// The output
    pub output: Output,
    /// The output metadata
    #[serde(rename = "outputMetadata")]
    pub output_metadata: OutputMetadata,
    /// The chain derived from seed, only for ed25519 addresses
    pub chain: Option<Chain>,
}

impl InputSigningData {
    /// Return the [OutputId]
    pub fn output_id(&self) -> &OutputId {
        self.output_metadata.output_id()
    }
}

/// Dto for data for transaction inputs for signing and ordering of unlock blocks
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InputSigningDataDto {
    /// The output
    pub output: OutputDto,
    /// The output metadata
    #[serde(rename = "outputMetadata")]
    pub output_metadata: OutputMetadataDto,
    /// The chain derived from seed, only for ed25519 addresses
    pub chain: Option<Chain>,
}

impl InputSigningData {
    pub(crate) fn try_from_dto(input: &InputSigningDataDto, token_supply: u64) -> Result<Self> {
        Ok(Self {
            output: Output::try_from_dto(&input.output, token_supply)?,
            output_metadata: OutputMetadata::try_from(&input.output_metadata)?,
            chain: input.chain.clone(),
        })
    }

    pub(crate) fn try_from_dto_unverified(input: &InputSigningDataDto) -> Result<Self> {
        Ok(Self {
            output: Output::try_from_dto_unverified(&input.output)?,
            output_metadata: OutputMetadata::try_from(&input.output_metadata)?,
            chain: input.chain.clone(),
        })
    }
}

impl From<&InputSigningData> for InputSigningDataDto {
    fn from(input: &InputSigningData) -> Self {
        Self {
            output: OutputDto::from(&input.output),
            output_metadata: OutputMetadataDto::from(&input.output_metadata),
            chain: input.chain.clone(),
        }
    }
}
