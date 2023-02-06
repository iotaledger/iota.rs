// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::block::address::Address;

/// Defines the Address that owns this output, that is, it can unlock it with the proper Unlock in a transaction.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddressUnlockCondition(Address);

impl AddressUnlockCondition {
    /// The [`UnlockCondition`](crate::block::output::UnlockCondition) kind of an [`AddressUnlockCondition`].
    pub const KIND: u8 = 0;

    /// Creates a new [`AddressUnlockCondition`].
    #[inline(always)]
    pub fn new(address: Address) -> Self {
        Self(address)
    }

    /// Returns the address of a [`AddressUnlockCondition`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::block::{address::dto::AddressDto, error::dto::DtoError};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct AddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    impl From<&AddressUnlockCondition> for AddressUnlockConditionDto {
        fn from(value: &AddressUnlockCondition) -> Self {
            Self {
                kind: AddressUnlockCondition::KIND,
                address: value.address().into(),
            }
        }
    }

    impl TryFrom<&AddressUnlockConditionDto> for AddressUnlockCondition {
        type Error = DtoError;

        fn try_from(value: &AddressUnlockConditionDto) -> Result<Self, DtoError> {
            Ok(Self::new(
                (&value.address)
                    .try_into()
                    .map_err(|_e| DtoError::InvalidField("addressUnlockCondition"))?,
            ))
        }
    }
}
