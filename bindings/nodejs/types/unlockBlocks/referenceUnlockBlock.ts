// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the reference unlock block.
 */
export const REFERENCE_UNLOCK_BLOCK_TYPE = 1;

/**
 * References a previous unlock block in order to substitute the duplication of the same unlock
 * block data for inputs which unlock through the same data.
 */
export interface ReferenceUnlockBlock extends TypeBase<1> {
    /**
     * The reference.
     */
    reference: number;
}
