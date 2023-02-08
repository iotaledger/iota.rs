// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::block::{
    address::{Address, AliasAddress},
    Error,
};

/// Defines the permanent [`AliasAddress`] that owns this output.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImmutableAliasAddressUnlockCondition(#[packable(verify_with = verify_alias_address)] Address);

impl ImmutableAliasAddressUnlockCondition {
    /// The [`UnlockCondition`](crate::block::output::UnlockCondition) kind of an
    /// [`ImmutableAliasAddressUnlockCondition`].
    pub const KIND: u8 = 6;

    /// Creates a new [`ImmutableAliasAddressUnlockCondition`].
    #[inline(always)]
    pub fn new(address: AliasAddress) -> Self {
        Self(Address::Alias(address))
    }

    /// Returns the address of an [`ImmutableAliasAddressUnlockCondition`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        // An ImmutableAliasAddressUnlockCondition must have an AliasAddress.
        // It has already been validated at construction that the address is an `AliasAddress`.
        debug_assert!(&self.0.is_alias());
        &self.0
    }

    /// Returns the alias address of an [`ImmutableAliasAddressUnlockCondition`].
    pub fn alias_address(&self) -> &AliasAddress {
        // It has already been validated at construction that the address is an `AliasAddress`.
        self.0.as_alias()
    }
}

fn verify_alias_address<const VERIFY: bool>(address: &Address, _: &()) -> Result<(), Error> {
    if VERIFY && !address.is_alias() {
        Err(Error::InvalidAddressKind(address.kind()))
    } else {
        Ok(())
    }
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::block::{address::dto::AddressDto, error::dto::DtoError};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct ImmutableAliasAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    impl From<&ImmutableAliasAddressUnlockCondition> for ImmutableAliasAddressUnlockConditionDto {
        fn from(value: &ImmutableAliasAddressUnlockCondition) -> Self {
            Self {
                kind: ImmutableAliasAddressUnlockCondition::KIND,
                address: value.address().into(),
            }
        }
    }

    impl TryFrom<&ImmutableAliasAddressUnlockConditionDto> for ImmutableAliasAddressUnlockCondition {
        type Error = DtoError;

        fn try_from(value: &ImmutableAliasAddressUnlockConditionDto) -> Result<Self, DtoError> {
            let address: Address = (&value.address)
                .try_into()
                .map_err(|_e| DtoError::InvalidField("immutableAliasAddressUnlockCondition"))?;

            // An ImmutableAliasAddressUnlockCondition must have an AliasAddress.
            if let Address::Alias(alias_address) = &address {
                Ok(Self::new(*alias_address))
            } else {
                Err(DtoError::InvalidField("immutableAliasAddressUnlockCondition"))
            }
        }
    }
}
