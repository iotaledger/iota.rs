// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from '../addresses';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the address unlock condition.
 */
export const ADDRESS_UNLOCK_CONDITION_TYPE = 0;

export interface AddressUnlockCondition extends TypeBase<0> {
    address: Address;
}
