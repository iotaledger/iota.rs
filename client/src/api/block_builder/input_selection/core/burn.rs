// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::block::{
    dto::U256Dto,
    output::{AliasId, FoundryId, NftId, TokenId},
    DtoError,
};

/// A type to specify what needs to be burned during input selection.
/// Nothing will be burned that has not been explicitly set with this struct.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Burn {
    /// Aliases to burn.
    pub(crate) aliases: HashSet<AliasId>,
    /// NFTs to burn.
    pub(crate) nfts: HashSet<NftId>,
    /// Foundries to burn.
    pub(crate) foundries: HashSet<FoundryId>,
    /// Amounts of native tokens to burn.
    /// `hashbrown::HashMap` to allow seamless operations with `NativeTokens`.
    pub(crate) native_tokens: hashbrown::HashMap<TokenId, U256>,
}

impl Burn {
    /// Creates a new [`Burn`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an alias to [`Burn`].
    pub fn add_alias(mut self, alias_id: AliasId) -> Self {
        self.aliases.insert(alias_id);
        self
    }

    /// Sets the aliases to [`Burn`].
    pub fn set_aliases(mut self, aliases: HashSet<AliasId>) -> Self {
        self.aliases = aliases;
        self
    }

    /// Returns the aliases to [`Burn`].
    pub fn aliases(&self) -> &HashSet<AliasId> {
        &self.aliases
    }

    /// Adds an NFT to [`Burn`].
    pub fn add_nft(mut self, nft_id: NftId) -> Self {
        self.nfts.insert(nft_id);
        self
    }

    /// Sets the NFTs to [`Burn`].
    pub fn set_nfts(mut self, nfts: HashSet<NftId>) -> Self {
        self.nfts = nfts;
        self
    }

    /// Returns the NFTs to [`Burn`].
    pub fn nfts(&self) -> &HashSet<NftId> {
        &self.nfts
    }

    /// Adds a foundry to [`Burn`].
    pub fn add_foundry(mut self, foundry_id: FoundryId) -> Self {
        self.foundries.insert(foundry_id);
        self
    }

    /// Sets the foundries to [`Burn`].
    pub fn set_foundries(mut self, foundries: HashSet<FoundryId>) -> Self {
        self.foundries = foundries;
        self
    }

    /// Returns the foundries to [`Burn`].
    pub fn foundries(&self) -> &HashSet<FoundryId> {
        &self.foundries
    }

    /// Adds an amount of native token to [`Burn`].
    pub fn add_native_token(mut self, token_id: TokenId, amount: impl Into<U256>) -> Self {
        self.native_tokens.insert(token_id, amount.into());
        self
    }

    /// Sets the amounts of native tokens to [`Burn`].
    pub fn set_native_tokens(mut self, native_tokens: HashMap<TokenId, impl Into<U256>>) -> Self {
        self.native_tokens = native_tokens
            .into_iter()
            .map(|(token_id, amount)| (token_id, amount.into()))
            .collect();
        self
    }

    /// Returns the native tokens to [`Burn`].
    pub fn native_tokens(&self) -> &hashbrown::HashMap<TokenId, U256> {
        &self.native_tokens
    }
}

/// A DTO for [`Burn`].
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BurnDto {
    /// Aliases to burn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) aliases: Option<HashSet<AliasId>>,
    /// NFTs to burn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) nfts: Option<HashSet<NftId>>,
    /// Foundries to burn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) foundries: Option<HashSet<FoundryId>>,
    /// Amounts of native tokens to burn.
    /// `hashbrown::HashMap` to allow seamless operations with `NativeTokens`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) native_tokens: Option<HashMap<TokenId, U256Dto>>,
}

impl From<&Burn> for BurnDto {
    fn from(value: &Burn) -> Self {
        Self {
            aliases: (!value.aliases.is_empty()).then_some(value.aliases.clone()),
            nfts: (!value.nfts.is_empty()).then_some(value.nfts.clone()),
            foundries: (!value.foundries.is_empty()).then_some(value.foundries.clone()),
            native_tokens: (!value.native_tokens.is_empty()).then_some(HashMap::from_iter(
                value
                    .native_tokens
                    .iter()
                    .map(|(token_id, amount)| (*token_id, U256Dto::from(amount))),
            )),
        }
    }
}

impl TryFrom<&BurnDto> for Burn {
    type Error = DtoError;

    fn try_from(value: &BurnDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aliases: value.aliases.clone().unwrap_or_default(),
            nfts: value.nfts.clone().unwrap_or_default(),
            foundries: value.foundries.clone().unwrap_or_default(),
            native_tokens: value
                .native_tokens
                .as_ref()
                .map(|native_tokens| {
                    native_tokens
                        .iter()
                        .map(|(token_id, amount)| U256::try_from(amount).map(|amount| (*token_id, amount)))
                        .collect::<Result<hashbrown::HashMap<_, _>, _>>()
                })
                .transpose()
                .map_err(|_| DtoError::InvalidField("native_tokens"))?
                .unwrap_or_default(),
        })
    }
}
