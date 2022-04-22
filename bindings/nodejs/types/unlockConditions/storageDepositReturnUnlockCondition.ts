// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from '../addresses';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the storage deposit return unlock condition.
 */
export const STORAGE_DEPOSIT_RETURN_UNLOCK_CONDITION_TYPE = 1;

export interface StorageDepositReturnUnlockCondition extends TypeBase<1> {
    returnAddress: Address;
    amount: string;
}
