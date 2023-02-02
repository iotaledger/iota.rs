// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use core::borrow::Borrow;

use packable::{prefix::StringPrefix, Packable};

use crate::block::{helper::network_name_to_id, output::RentStructure, Error, PROTOCOL_VERSION};

/// Defines the parameters of the protocol.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error)]
pub struct ProtocolParameters {
    // The version of the protocol running.
    #[cfg_attr(feature = "serde", serde(alias = "protocolVersion"))]
    protocol_version: u8,
    // The human friendly name of the network.
    #[packable(unpack_error_with = |err| Error::InvalidNetworkName(err.into_item_err()))]
    #[cfg_attr(feature = "serde", serde(alias = "networkName"))]
    network_name: StringPrefix<u8>,
    // The HRP prefix used for Bech32 addresses in the network.
    #[packable(unpack_error_with = |err| Error::InvalidBech32Hrp(err.into_item_err()))]
    #[cfg_attr(feature = "serde", serde(alias = "bech32Hrp"))]
    bech32_hrp: StringPrefix<u8>,
    // The minimum pow score of the network.
    #[cfg_attr(feature = "serde", serde(alias = "minPowScore"))]
    min_pow_score: u32,
    // The below max depth parameter of the network.
    #[cfg_attr(feature = "serde", serde(alias = "belowMaxDepth"))]
    below_max_depth: u8,
    // The rent structure used by given node/network.
    #[cfg_attr(feature = "serde", serde(alias = "rentStructure"))]
    rent_structure: RentStructure,
    // TokenSupply defines the current token supply on the network.
    #[cfg_attr(feature = "serde", serde(alias = "tokenSupply"))]
    token_supply: u64,
}

// This implementation is required to make [`ProtocolParameters`] a [`Packable`] visitor.
impl Borrow<()> for ProtocolParameters {
    fn borrow(&self) -> &() {
        &()
    }
}

impl Default for ProtocolParameters {
    fn default() -> Self {
        // PANIC: These values are known to be correct.
        Self::new(
            PROTOCOL_VERSION,
            String::from("shimmer"),
            String::from("smr"),
            1500,
            15,
            RentStructure::default(),
            1_813_620_509_061_365,
        )
        .unwrap()
    }
}

impl ProtocolParameters {
    /// Creates a new [`ProtocolParameters`].
    pub fn new(
        protocol_version: u8,
        network_name: String,
        bech32_hrp: String,
        min_pow_score: u32,
        below_max_depth: u8,
        rent_structure: RentStructure,
        token_supply: u64,
    ) -> Result<Self, Error> {
        Ok(Self {
            protocol_version,
            network_name: <StringPrefix<u8>>::try_from(network_name).map_err(Error::InvalidStringPrefix)?,
            bech32_hrp: <StringPrefix<u8>>::try_from(bech32_hrp).map_err(Error::InvalidStringPrefix)?,
            min_pow_score,
            below_max_depth,
            rent_structure,
            token_supply,
        })
    }

    /// Returns the protocol version of the [`ProtocolParameters`].
    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    /// Returns the network name of the [`ProtocolParameters`].
    pub fn network_name(&self) -> &str {
        &self.network_name
    }

    /// Returns the network ID of the [`ProtocolParameters`].
    pub fn network_id(&self) -> u64 {
        network_name_to_id(&self.network_name)
    }

    /// Returns the bech32 HRP of the [`ProtocolParameters`].
    pub fn bech32_hrp(&self) -> &str {
        &self.bech32_hrp
    }

    /// Returns the minimum PoW score of the [`ProtocolParameters`].
    pub fn min_pow_score(&self) -> u32 {
        self.min_pow_score
    }

    /// Returns the below max depth of the [`ProtocolParameters`].
    pub fn below_max_depth(&self) -> u8 {
        self.below_max_depth
    }

    /// Returns the rent structure of the [`ProtocolParameters`].
    pub fn rent_structure(&self) -> &RentStructure {
        &self.rent_structure
    }

    /// Returns the token supply of the [`ProtocolParameters`].
    pub fn token_supply(&self) -> u64 {
        self.token_supply
    }
}

/// Returns a [`ProtocolParameters`] for testing purposes.
#[cfg(any(feature = "test", feature = "rand"))]
pub fn protocol_parameters() -> ProtocolParameters {
    ProtocolParameters::new(
        2,
        String::from("testnet"),
        String::from("rms"),
        1500,
        15,
        crate::block::output::RentStructure::new(500, 10, 1),
        1_813_620_509_061_365,
    )
    .unwrap()
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {

    use super::*;
    use crate::block::{error::dto::DtoError, output::dto::RentStructureDto};

    #[derive(Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Serialize, serde::Deserialize),
        serde(rename_all = "camelCase")
    )]
    pub struct ProtocolParametersDto {
        #[cfg_attr(feature = "serde", serde(rename = "version"))]
        pub protocol_version: u8,
        pub network_name: String,
        pub bech32_hrp: String,
        pub min_pow_score: u32,
        pub below_max_depth: u8,
        pub rent_structure: RentStructureDto,
        pub token_supply: String,
    }

    impl TryFrom<ProtocolParametersDto> for ProtocolParameters {
        type Error = DtoError;

        fn try_from(value: ProtocolParametersDto) -> Result<Self, Self::Error> {
            Ok(Self::new(
                value.protocol_version,
                value.network_name,
                value.bech32_hrp,
                value.min_pow_score,
                value.below_max_depth,
                value.rent_structure.into(),
                value
                    .token_supply
                    .parse()
                    .map_err(|_| DtoError::InvalidField("token_supply"))?,
            )?)
        }
    }
}
