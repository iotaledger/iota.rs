// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the NFT unlock block.
 */
export const NFT_UNLOCK_BLOCK_TYPE = 3;

/**
 * Points to the unlock block of a consumed NFT output.
 */
export interface NftUnlockBlock extends TypeBase<3> {
    /**
     * The reference.
     */
    reference: number;
}
