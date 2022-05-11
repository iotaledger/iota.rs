// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{
    address::{dto::AddressDto, Address},
    output::{dto::OutputDto, Output},
    payload::transaction::{dto::TransactionEssenceDto, TransactionEssence},
    DtoError,
};

use crate::{
    crypto::keys::slip10::Chain,
    secret::types::{InputSigningData, InputSigningDataDto},
};

/// Helper struct for offline signing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedTransactionData {
    /// Transaction essence
    pub essence: TransactionEssence,
    /// Required address information for signing
    #[serde(rename = "inputsData")]
    pub inputs_data: Vec<InputSigningData>,
    /// Optional remainder output information
    pub remainder: Option<RemainderData>,
}

/// Data for a remainder output, used for ledger nano
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemainderData {
    /// The remainder output
    pub output: Output,
    /// The chain derived from seed, for the remainder addresses
    pub chain: Option<Chain>,
    /// The remainder address
    pub address: Address,
}

/// Data for a remainder output, used for ledger nano
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemainderDataDto {
    /// The remainder output
    pub output: OutputDto,
    /// The chain derived from seed, for the remainder addresses
    pub chain: Option<Chain>,
    /// The remainder address
    pub address: AddressDto,
}

impl TryFrom<&RemainderDataDto> for RemainderData {
    type Error = crate::Error;

    fn try_from(remainder: &RemainderDataDto) -> crate::Result<Self> {
        Ok(Self {
            output: Output::try_from(&remainder.output)?,
            chain: remainder.chain.clone(),
            address: Address::try_from(&remainder.address)?,
        })
    }
}
impl From<&RemainderData> for RemainderDataDto {
    fn from(remainder: &RemainderData) -> Self {
        Self {
            output: OutputDto::from(&remainder.output),
            chain: remainder.chain.clone(),
            address: AddressDto::from(&remainder.address),
        }
    }
}

/// PreparedTransactionData Dto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedTransactionDataDto {
    /// Transaction essence
    pub essence: TransactionEssenceDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningDataDto>,
    /// Optional remainder output information
    pub remainder: Option<RemainderDataDto>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        PreparedTransactionDataDto {
            essence: TransactionEssenceDto::from(&value.essence),
            inputs_data: value.inputs_data.iter().map(InputSigningDataDto::from).collect(),
            remainder: value.remainder.as_ref().map(RemainderDataDto::from),
        }
    }
}

impl TryFrom<&PreparedTransactionDataDto> for PreparedTransactionData {
    type Error = DtoError;
    fn try_from(value: &PreparedTransactionDataDto) -> Result<Self, Self::Error> {
        Ok(PreparedTransactionData {
            essence: TransactionEssence::try_from(&value.essence).map_err(|_| DtoError::InvalidField("essence"))?,
            inputs_data: value
                .inputs_data
                .iter()
                .map(InputSigningData::try_from)
                .collect::<crate::Result<Vec<InputSigningData>>>()
                .map_err(|_| DtoError::InvalidField("input_data"))?,
            remainder: match &value.remainder {
                Some(remainder) => {
                    Some(RemainderData::try_from(remainder).map_err(|_| DtoError::InvalidField("remainder"))?)
                }
                None => None,
            },
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
