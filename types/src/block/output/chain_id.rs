// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::block::output::{AliasId, FoundryId, NftId, OutputId};

///
#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChainId {
    ///
    Alias(AliasId),
    ///
    Foundry(FoundryId),
    ///
    Nft(NftId),
}

impl ChainId {
    ///
    pub fn is_null(&self) -> bool {
        match self {
            Self::Alias(alias_id) => alias_id.is_null(),
            Self::Foundry(foundry_id) => foundry_id.is_null(),
            Self::Nft(nft_id) => nft_id.is_null(),
        }
    }

    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if !self.is_null() {
            return self;
        }

        match self {
            Self::Alias(_) => Self::Alias(AliasId::from(output_id)),
            Self::Foundry(_) => self,
            Self::Nft(_) => Self::Nft(NftId::from(output_id)),
        }
    }
}

impl core::fmt::Display for ChainId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alias(id) => write!(f, "{id}"),
            Self::Foundry(id) => write!(f, "{id}"),
            Self::Nft(id) => write!(f, "{id}"),
        }
    }
}

impl core::fmt::Debug for ChainId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alias(id) => write!(f, "ChainId({id:?})"),
            Self::Foundry(id) => write!(f, "ChainId({id:?})"),
            Self::Nft(id) => write!(f, "ChainId({id:?})"),
        }
    }
}
