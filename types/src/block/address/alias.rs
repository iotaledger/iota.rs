// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref, From};

use crate::block::{output::AliasId, Error};

/// An alias address.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, packable::Packable)]
#[as_ref(forward)]
pub struct AliasAddress(AliasId);

impl AliasAddress {
    /// The [`Address`](crate::block::address::Address) kind of an [`AliasAddress`].
    pub const KIND: u8 = 8;
    /// The length of an [`AliasAddress`].
    pub const LENGTH: usize = AliasId::LENGTH;

    /// Creates a new [`AliasAddress`].
    #[inline(always)]
    pub fn new(id: AliasId) -> Self {
        Self::from(id)
    }

    /// Returns the [`AliasId`] of an [`AliasAddress`].
    #[inline(always)]
    pub fn alias_id(&self) -> &AliasId {
        &self.0
    }

    /// Consumes an [`AliasAddress`] and returns its [`AliasId`].
    #[inline(always)]
    pub fn into_alias_id(self) -> AliasId {
        self.0
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(AliasAddress);

impl FromStr for AliasAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(AliasId::from_str(s)?))
    }
}

impl core::fmt::Display for AliasAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for AliasAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AliasAddress({self})")
    }
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::block::error::dto::DtoError;

    /// Describes an alias address.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct AliasAddressDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(rename = "aliasId")]
        pub alias_id: String,
    }

    impl From<&AliasAddress> for AliasAddressDto {
        fn from(value: &AliasAddress) -> Self {
            Self {
                kind: AliasAddress::KIND,
                alias_id: value.to_string(),
            }
        }
    }

    impl TryFrom<&AliasAddressDto> for AliasAddress {
        type Error = DtoError;

        fn try_from(value: &AliasAddressDto) -> Result<Self, Self::Error> {
            value
                .alias_id
                .parse::<Self>()
                .map_err(|_| DtoError::InvalidField("aliasId"))
        }
    }
}
