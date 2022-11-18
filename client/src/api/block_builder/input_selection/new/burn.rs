// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use primitive_types::U256;

use crate::block::output::{AliasId, FoundryId, NftId, TokenId};

#[derive(Default)]
pub struct Burn {
    /// Aliases to burn.
    pub(crate) aliases: HashSet<AliasId>,
    /// NFTs to burn.
    pub(crate) nfts: HashSet<NftId>,
    /// Foundries to burn.
    pub(crate) foundries: HashSet<FoundryId>,
    /// Native tokens with the amount to burn.
    pub(crate) native_tokens: HashMap<TokenId, U256>,
}

impl Burn {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_alias(mut self, alias_id: AliasId) -> Self {
        self.aliases.insert(alias_id);
        self
    }

    pub fn set_aliases(mut self, aliases: HashSet<AliasId>) -> Self {
        self.aliases = aliases;
        self
    }

    pub fn add_nft(mut self, nft_id: NftId) -> Self {
        self.nfts.insert(nft_id);
        self
    }

    pub fn set_nfts(mut self, nfts: HashSet<NftId>) -> Self {
        self.nfts = nfts;
        self
    }

    pub fn add_foundry(mut self, foundry_id: FoundryId) -> Self {
        self.foundries.insert(foundry_id);
        self
    }

    pub fn set_foundries(mut self, foundries: HashSet<FoundryId>) -> Self {
        self.foundries = foundries;
        self
    }

    pub fn add_native_token(mut self, token_id: TokenId, amount: U256) -> Self {
        self.native_tokens.insert(token_id, amount);
        self
    }

    pub fn set_native_tokens(mut self, native_tokens: HashMap<TokenId, U256>) -> Self {
        self.native_tokens = native_tokens;
        self
    }
}
