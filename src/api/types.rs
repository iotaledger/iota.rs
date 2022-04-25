// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{
    address::Address,
    payload::transaction::{dto::TransactionEssenceDto, TransactionEssence},
    DtoError,
};

use crate::secret::types::InputSigningData;

/// Helper struct for offline signing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedTransactionData {
    /// Transaction essence
    pub essence: TransactionEssence,
    /// Required address information for signing
    #[serde(rename = "inputSigningDataEntries")]
    pub input_signing_data_entries: Vec<InputSigningData>,
}

/// PreparedTransactionData Dto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedTransactionDataDto {
    /// Transaction essence
    pub essence: TransactionEssenceDto,
    /// Required address information for signing
    pub input_signing_data_entries: Vec<InputSigningData>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        PreparedTransactionDataDto {
            essence: TransactionEssenceDto::from(&value.essence),
            input_signing_data_entries: value.input_signing_data_entries.clone(),
        }
    }
}

impl TryFrom<&PreparedTransactionDataDto> for PreparedTransactionData {
    type Error = DtoError;
    fn try_from(value: &PreparedTransactionDataDto) -> Result<Self, Self::Error> {
        Ok(PreparedTransactionData {
            essence: TransactionEssence::try_from(&value.essence).map_err(|_| DtoError::InvalidField("essence"))?,
            input_signing_data_entries: value.input_signing_data_entries.clone(),
        })
    }
}

/// Generated addresses
#[derive(Debug, Clone)]
pub struct RawAddresses {
    /// Public addresses
    pub public: Vec<Address>,
    /// Internal/change addresses <https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki#change>
    pub internal: Vec<Address>,
}

/// Generated addresses bech32 encoded
#[derive(Debug, Clone)]
pub struct Bech32Addresses {
    /// Public addresses
    pub public: Vec<String>,
    /// Internal/change addresses <https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki#change>
    pub internal: Vec<String>,
}
