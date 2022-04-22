// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from '../addresses';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the governor address unlock condition.
 */
export const GOVERNOR_ADDRESS_UNLOCK_CONDITION_TYPE = 5;

export interface GovernorAddressUnlockCondition extends TypeBase<5> {
    address: Address;
}
