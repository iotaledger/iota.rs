// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the timelock unlock condition.
 */
export const TIMELOCK_UNLOCK_CONDITION_TYPE = 2;

export interface TimelockUnlockCondition extends TypeBase<2> {
    /**
     * The milestone index starting from which the output can be consumed.
     */
    milestoneIndex: number;
    /**
     * Unix time (seconds since Unix epoch) starting from which the output can be consumed.
     */
    timestamp: number;
}
