// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the option.
 */
export const POW_MILESTONE_OPTION_TYPE = 1;

/**
 * PoW milestone option.
 */
export interface PoWMilestoneOption extends TypeBase<1> {
    /**
     * The next PoW score.
     */
    nextPoWScore: number;

    /**
     * The milestone at which the next PoW score becomes active.
     */
    nextPoWScoreMilestoneIndex: number;
}
