// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::block::output::{AliasId, FoundryId, NftId, TokenId};

/// A type to specify what needs to be burned during input selection.
/// Nothing will be burned that has not been explicitly set with this struct.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
}
