// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use primitive_types::U256;

use crate::block::output::{AliasId, FoundryId, NftId, TokenId};

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
