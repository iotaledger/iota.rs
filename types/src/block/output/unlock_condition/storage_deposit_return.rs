// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::{address::Address, output::verify_output_amount, protocol::ProtocolParameters, Error};

/// Defines the amount of IOTAs used as storage deposit that have to be returned to the return [`Address`].
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct StorageDepositReturnUnlockCondition {
    // The [`Address`] to return the amount to.
    // TODO remove pub(crate) when there are specific DTOs for all unlock conditions.
    pub(crate) return_address: Address,
    // Amount of IOTA coins the consuming transaction should deposit to `return_address`.
    // TODO remove pub(crate) when there are specific DTOs for all unlock conditions.
    #[packable(verify_with = verify_amount_packable)]
    pub(crate) amount: u64,
}

impl StorageDepositReturnUnlockCondition {
    /// The [`UnlockCondition`](crate::block::output::UnlockCondition) kind of a
    /// [`StorageDepositReturnUnlockCondition`].
    pub const KIND: u8 = 1;

    /// Creates a new [`StorageDepositReturnUnlockCondition`].
    #[inline(always)]
    pub fn new(return_address: Address, amount: u64, token_supply: u64) -> Result<Self, Error> {
        verify_amount::<true>(&amount, &token_supply)?;

        Ok(Self { return_address, amount })
    }

    /// Returns the return address.
    #[inline(always)]
    pub fn return_address(&self) -> &Address {
        &self.return_address
    }

    /// Returns the amount.
    #[inline(always)]
    pub fn amount(&self) -> u64 {
        self.amount
    }
}

fn verify_amount<const VERIFY: bool>(amount: &u64, token_supply: &u64) -> Result<(), Error> {
    if VERIFY {
        verify_output_amount::<VERIFY>(amount, token_supply)
            .map_err(|_| Error::InvalidStorageDepositAmount(*amount))?;
    }

    Ok(())
}

fn verify_amount_packable<const VERIFY: bool>(
    amount: &u64,
    protocol_parameters: &ProtocolParameters,
) -> Result<(), Error> {
    verify_amount::<VERIFY>(amount, &protocol_parameters.token_supply())
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::block::{address::dto::AddressDto, error::dto::DtoError};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct StorageDepositReturnUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(rename = "returnAddress")]
        pub return_address: AddressDto,
        pub amount: String,
    }

    impl From<&StorageDepositReturnUnlockCondition> for StorageDepositReturnUnlockConditionDto {
        fn from(value: &StorageDepositReturnUnlockCondition) -> Self {
            Self {
                kind: StorageDepositReturnUnlockCondition::KIND,
                return_address: AddressDto::from(value.return_address()),
                amount: value.amount().to_string(),
            }
        }
    }

    impl StorageDepositReturnUnlockCondition {
        pub fn try_from_dto(
            value: &StorageDepositReturnUnlockConditionDto,
            token_supply: u64,
        ) -> Result<Self, DtoError> {
            Ok(Self::new(
                Address::try_from(&value.return_address)?,
                value
                    .amount
                    .parse::<u64>()
                    .map_err(|_| DtoError::InvalidField("amount"))?,
                token_supply,
            )?)
        }

        pub fn try_from_dto_unverified(value: &StorageDepositReturnUnlockConditionDto) -> Result<Self, DtoError> {
            Ok(Self {
                return_address: Address::try_from(&value.return_address)?,
                amount: value
                    .amount
                    .parse::<u64>()
                    .map_err(|_| DtoError::InvalidField("amount"))?,
            })
        }
    }
}
