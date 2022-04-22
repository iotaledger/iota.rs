// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from '../addresses';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the expiration unlock condition.
 */
export const EXPIRATION_UNLOCK_CONDITION_TYPE = 3;

export interface ExpirationUnlockCondition extends TypeBase<3> {
    returnAddress: Address;
    milestoneIndex: number;
    timestamp: number;
}
