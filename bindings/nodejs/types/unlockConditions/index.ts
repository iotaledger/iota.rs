// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { AddressUnlockCondition } from './addressUnlockCondition';
import type { ExpirationUnlockCondition } from './expirationUnlockCondition';
import type { GovernorAddressUnlockCondition } from './governorAddressUnlockCondition';
import type { ImmutableAliasAddressUnlockCondition } from './immutableAliasUnlockCondition';
import type { StateControllerAddressUnlockCondition } from './stateControllerAddressUnlockCondition';
import type { StorageDepositReturnUnlockCondition } from './storageDepositReturnUnlockCondition';
import type { TimelockUnlockCondition } from './timelockUnlockCondition';

/**
 * All the unlock conditions.
 */
export type UnlockCondition =
    | AddressUnlockCondition
    | StorageDepositReturnUnlockCondition
    | TimelockUnlockCondition
    | ExpirationUnlockCondition
    | StateControllerAddressUnlockCondition
    | GovernorAddressUnlockCondition
    | ImmutableAliasAddressUnlockCondition;
