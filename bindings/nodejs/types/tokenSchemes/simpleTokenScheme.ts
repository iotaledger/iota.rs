// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the simple token scheme.
 */
export const SIMPLE_TOKEN_SCHEME_TYPE = 0;

export interface SimpleTokenScheme extends TypeBase<0> {
    /**
     * Amount of tokens minted by a foundry.
     */
    mintedTokens: string;
    /**
     * Amount of tokens melted by a foundry.
     */
    meltedTokens: string;
    /**
     * Maximum supply of tokens controlled by a foundry.
     */
    maximumSupply: string;
}
