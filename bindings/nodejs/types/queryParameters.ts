// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Query parameter for output requests
 */
export type QueryParameter =
    | Address
    | HasStorageDepositReturnCondition
    | StorageDepositReturnAddress
    | HasTimelockCondition
    | TimelockedBefore
    | TimelockedAfter
    | TimelockedBeforeMilestone
    | TimelockedAfterMilestone
    | HasExpirationCondition
    | ExpiresBefore
    | ExpiresAfter
    | ExpiresBeforeMilestone
    | ExpiresAfterMilestone
    | ExpirationReturnAddress
    | Sender
    | Tag
    | CreatedBefore
    | CreatedAfter
    | Cursor
    | Issuer
    | StateController
    | Governor
    | PageSize;

interface Address {
    address: string;
}
interface HasStorageDepositReturnCondition {
    hasStorageDepositReturnCondition: boolean;
}
interface StorageDepositReturnAddress {
    storageDepositReturnAddress: string;
}
interface HasTimelockCondition {
    hasTimelockCondition: boolean;
}
interface TimelockedBefore {
    timelockedBefore: number;
}
interface TimelockedAfter {
    timelockedAfter: number;
}
interface TimelockedBeforeMilestone {
    timelockedBeforeMilestone: number;
}
interface TimelockedAfterMilestone {
    timelockedAfterMilestone: number;
}
interface HasExpirationCondition {
    hasExpirationCondition: boolean;
}
interface ExpiresBefore {
    expiresBefore: number;
}
interface ExpiresAfter {
    expiresAfter: number;
}
interface ExpiresBeforeMilestone {
    expiresBeforeMilestone: number;
}
interface ExpiresAfterMilestone {
    expiresAfterMilestone: number;
}
interface ExpirationReturnAddress {
    expirationReturnAddress: string;
}
interface Sender {
    sender: string;
}
interface Tag {
    tag: string;
}
interface CreatedBefore {
    createdBefore: string;
}
interface CreatedAfter {
    createdAfter: string;
}
interface Cursor {
    cursor: string;
}
interface Issuer {
    issuer: string;
}
interface StateController {
    stateController: string;
}
interface Governor {
    governor: string;
}
interface PageSize {
    pageSize: string;
}
