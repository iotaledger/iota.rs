// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from '../addresses';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the immutable alias unlock condition.
 */
export const IMMUTABLE_ALIAS_UNLOCK_CONDITION_TYPE = 6;

export interface ImmutableAliasAddressUnlockCondition extends TypeBase<6> {
    address: Address;
}
