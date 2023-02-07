// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { INativeToken } from '@iota/types';

/// A DTO for [`Burn`].
export interface Burn {
    /** Aliases to burn */
    aliases: string[];
    /** NFTs to burn */
    nfts: string[];
    /** Foundries to burn */
    foundries: string[];
    /** Amounts of native tokens to burn */
    native_tokens: INativeToken;
}