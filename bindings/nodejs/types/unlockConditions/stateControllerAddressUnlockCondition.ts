// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from '../addresses';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the state controller unlock condition.
 */
export const STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE = 4;

export interface StateControllerAddressUnlockCondition extends TypeBase<4> {
    address: Address;
}
