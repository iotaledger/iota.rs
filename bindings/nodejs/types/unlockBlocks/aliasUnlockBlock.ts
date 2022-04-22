// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the alias unlock block.
 */
export const ALIAS_UNLOCK_BLOCK_TYPE = 2;

/**
 * Points to the unlock block of a consumed alias output.
 */
export interface AliasUnlockBlock extends TypeBase<2> {
    /**
     * The reference.
     */
    reference: number;
}
