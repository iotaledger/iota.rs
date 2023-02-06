// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_types::block::{
    address::{dto::AddressDto, Address},
    output::{dto::OutputDto, Output},
    payload::{
        transaction::{
            dto::{TransactionEssenceDto, TransactionPayloadDto},
            TransactionEssence,
        },
        TransactionPayload,
    },
    protocol::ProtocolParameters,
    DtoError,
};

use crate::{
    crypto::keys::slip10::Chain,
    secret::types::{InputSigningData, InputSigningDataDto},
};

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreparedTransactionData {
    /// Transaction essence
    pub essence: TransactionEssence,
    /// Required input information for signing. Inputs need to be ordered by address type
    #[serde(rename = "inputsData")]
    pub inputs_data: Vec<InputSigningData>,
    /// Optional remainder output information
    pub remainder: Option<RemainderData>,
}

/// PreparedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreparedTransactionDataDto {
    /// Transaction essence
    pub essence: TransactionEssenceDto,
    /// Required address information for signing
    #[serde(rename = "inputsData")]
    pub inputs_data: Vec<InputSigningDataDto>,
    /// Optional remainder output information
    pub remainder: Option<RemainderDataDto>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        Self {
            essence: TransactionEssenceDto::from(&value.essence),
            inputs_data: value.inputs_data.iter().map(InputSigningDataDto::from).collect(),
            remainder: value.remainder.as_ref().map(RemainderDataDto::from),
        }
    }
}

impl PreparedTransactionData {
    /// Conversion from [`PreparedTransactionDataDto`] to [`PreparedTransactionData`].
    pub fn try_from_dto(
        value: &PreparedTransactionDataDto,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Self, DtoError> {
        Ok(Self {
            essence: TransactionEssence::try_from_dto(&value.essence, protocol_parameters)
                .map_err(|_| DtoError::InvalidField("essence"))?,
            inputs_data: value
                .inputs_data
                .iter()
                .map(|i| InputSigningData::try_from_dto(i, protocol_parameters.token_supply()))
                .collect::<crate::Result<Vec<InputSigningData>>>()
                .map_err(|_| DtoError::InvalidField("input_data"))?,
            remainder: match &value.remainder {
                Some(remainder) => Some(
                    RemainderData::try_from_dto(remainder, protocol_parameters.token_supply())
                        .map_err(|_| DtoError::InvalidField("remainder"))?,
                ),
                None => None,
            },
        })
    }

    /// Unverified conversion from [`PreparedTransactionDataDto`] to [`PreparedTransactionData`].
    pub fn try_from_dto_unverified(value: &PreparedTransactionDataDto) -> Result<Self, DtoError> {
        Ok(Self {
            essence: TransactionEssence::try_from_dto_unverified(&value.essence)
                .map_err(|_| DtoError::InvalidField("essence"))?,
            inputs_data: value
                .inputs_data
                .iter()
                .map(InputSigningData::try_from_dto_unverified)
                .collect::<crate::Result<Vec<InputSigningData>>>()
                .map_err(|_| DtoError::InvalidField("inputs_data"))?,
            remainder: match &value.remainder {
                Some(remainder) => Some(
                    RemainderData::try_from_dto_unverified(remainder)
                        .map_err(|_| DtoError::InvalidField("remainder"))?,
                ),
                None => None,
            },
        })
    }
}

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignedTransactionData {
    /// Signed transaction payload
    #[serde(rename = "transactionPayload")]
    pub transaction_payload: TransactionPayload,
    /// Required address information for signing
    #[serde(rename = "inputsData")]
    pub inputs_data: Vec<InputSigningData>,
}

/// SignedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignedTransactionDataDto {
    /// Transaction essence
    #[serde(rename = "transactionPayload")]
    pub transaction_payload: TransactionPayloadDto,
    /// Required address information for signing
    #[serde(rename = "inputsData")]
    pub inputs_data: Vec<InputSigningDataDto>,
}

impl From<&SignedTransactionData> for SignedTransactionDataDto {
    fn from(value: &SignedTransactionData) -> Self {
        Self {
            transaction_payload: TransactionPayloadDto::from(&value.transaction_payload),
            inputs_data: value.inputs_data.iter().map(InputSigningDataDto::from).collect(),
        }
    }
}

impl SignedTransactionData {
    /// Conversion from [`SignedTransactionDataDto`] to [`SignedTransactionData`].
    pub fn try_from_dto(
        value: &SignedTransactionDataDto,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Self, DtoError> {
        Ok(Self {
            transaction_payload: TransactionPayload::try_from_dto(&value.transaction_payload, protocol_parameters)
                .map_err(|_| DtoError::InvalidField("transaction_payload"))?,
            inputs_data: value
                .inputs_data
                .iter()
                .map(|i| InputSigningData::try_from_dto(i, protocol_parameters.token_supply()))
                .collect::<crate::Result<Vec<InputSigningData>>>()
                .map_err(|_| DtoError::InvalidField("input_data"))?,
        })
    }

    /// Unverified conversion from [`SignedTransactionDataDto`] to [`SignedTransactionData`].
    pub fn try_from_dto_unverified(value: &SignedTransactionDataDto) -> Result<Self, DtoError> {
        Ok(Self {
            transaction_payload: TransactionPayload::try_from_dto_unverified(&value.transaction_payload)
                .map_err(|_| DtoError::InvalidField("transaction_payload"))?,
            inputs_data: value
                .inputs_data
                .iter()
                .map(InputSigningData::try_from_dto_unverified)
                .collect::<crate::Result<Vec<InputSigningData>>>()
                .map_err(|_| DtoError::InvalidField("inputs_data"))?,
        })
    }
}

/// Data for a remainder output, used for ledger nano
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RemainderData {
    /// The remainder output
    pub output: Output,
    /// The chain derived from seed, for the remainder addresses
    pub chain: Option<Chain>,
    /// The remainder address
    pub address: Address,
}

/// Data for a remainder output, used for ledger nano
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RemainderDataDto {
    /// The remainder output
    pub output: OutputDto,
    /// The chain derived from seed, for the remainder addresses
    pub chain: Option<Chain>,
    /// The remainder address
    pub address: AddressDto,
}

impl RemainderData {
    pub(crate) fn try_from_dto(remainder: &RemainderDataDto, token_supply: u64) -> crate::Result<Self> {
        Ok(Self {
            output: Output::try_from_dto(&remainder.output, token_supply)?,
            chain: remainder.chain.clone(),
            address: Address::try_from(&remainder.address)?,
        })
    }

    pub(crate) fn try_from_dto_unverified(remainder: &RemainderDataDto) -> crate::Result<Self> {
        Ok(Self {
            output: Output::try_from_dto_unverified(&remainder.output)?,
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
