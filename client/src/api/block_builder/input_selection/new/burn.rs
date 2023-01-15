// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use hashbrown::HashMap;
use primitive_types::U256;
use serde::Deserialize;

use crate::block::output::{AliasId, FoundryId, NftId, TokenId};

/// A type to explicit what needs to be burned during input selection.
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Burn {
    /// Aliases to burn.
    pub(crate) aliases: HashSet<AliasId>,
    /// NFTs to burn.
    pub(crate) nfts: HashSet<NftId>,
    /// Foundries to burn.
    pub(crate) foundries: HashSet<FoundryId>,
    /// Amounts of native tokens to burn.
    pub(crate) native_tokens: HashMap<TokenId, U256>,
}

impl Burn {
    /// Creates a new [`Burn`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an alias to burn.
    pub fn add_alias(mut self, alias_id: AliasId) -> Self {
        self.aliases.insert(alias_id);
        self
    }

    /// Sets the aliases to burn.
    pub fn set_aliases(mut self, aliases: HashSet<AliasId>) -> Self {
        self.aliases = aliases;
        self
    }

    /// Adds an NFT to burn.
    pub fn add_nft(mut self, nft_id: NftId) -> Self {
        self.nfts.insert(nft_id);
        self
    }

    /// Sets the NFTs to burn.
    pub fn set_nfts(mut self, nfts: HashSet<NftId>) -> Self {
        self.nfts = nfts;
        self
    }

    /// Adds a foundry to burn.
    pub fn add_foundry(mut self, foundry_id: FoundryId) -> Self {
        self.foundries.insert(foundry_id);
        self
    }

    /// Sets the foundries to burn.
    pub fn set_foundries(mut self, foundries: HashSet<FoundryId>) -> Self {
        self.foundries = foundries;
        self
    }

    /// Adds a native token to burn.
    pub fn add_native_token(mut self, token_id: TokenId, amount: U256) -> Self {
        self.native_tokens.insert(token_id, amount);
        self
    }

    /// Sets the native tokens to burn.
    pub fn set_native_tokens(mut self, native_tokens: HashMap<TokenId, U256>) -> Self {
        self.native_tokens = native_tokens;
        self
    }
}
