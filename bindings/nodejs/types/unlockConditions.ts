// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from './addresses';

export type UnlockCondition =
    | AddressUnlockCondition
    | StorageDepositReturnUnlockCondition
    | TimelockUnlockCondition
    | ExpirationUnlockCondition
    | StateControllerAddressUnlockCondition
    | GovernorAddressUnlockCondition
    | ImmutableAliasAddressUnlockCondition;

export interface AddressUnlockCondition {
    type: number;
    address: Address;
}

export interface StorageDepositReturnUnlockCondition {
    type: number;
    returnAddress: Address;
    amount: string;
}

export interface TimelockUnlockCondition {
    type: number;
    milestoneIndex: number;
    timestamp: number;
}

export interface ExpirationUnlockCondition {
    type: number;
    returnAddress: Address;
    milestoneIndex: number;
    timestamp: number;
}

export interface StateControllerAddressUnlockCondition {
    type: number;
    address: Address;
}

export interface GovernorAddressUnlockCondition {
    type: number;
    address: Address;
}

export interface ImmutableAliasAddressUnlockCondition {
    type: number;
    address: Address;
}
