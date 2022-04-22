// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { TypeBase } from '../typeBase';

/**
 * The global type for the treasury input.
 */
export const TREASURY_INPUT_TYPE = 1;

/**
 * Treasury Input.
 */
export interface TreasuryInput extends TypeBase<1> {
    /**
     * The milestone id of the input.
     */
    milestoneId: string;
}
