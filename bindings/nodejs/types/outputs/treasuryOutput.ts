// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the treasury output.
 */
export const TREASURY_OUTPUT_TYPE = 2;

/**
 * Describes a treasury output.
 */
export interface TreasuryOutput extends TypeBase<2> {
    /**
     * Amount of IOTA tokens held by the output.
     */
    amount: string;
}
