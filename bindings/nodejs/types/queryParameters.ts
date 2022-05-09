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

/** Bech32-encoded address that should be searched for. */
interface Address {
    address: string;
}
/** Filters outputs based on the presence of storage deposit return unlockcondition. */
interface HasStorageDepositReturnCondition {
    hasStorageDepositReturnCondition: boolean;
}
/** Filter outputs based on the presence of a specific Bech32-encoded return address
 * in the storage deposit return unlock condition.
 */
interface StorageDepositReturnAddress {
    storageDepositReturnAddress: string;
}
/** Filters outputs based on the presence of timelock unlock condition. */
interface HasTimelockCondition {
    hasTimelockCondition: boolean;
}
/** Return outputs that are timelocked before a certain Unix timestamp. */
interface TimelockedBefore {
    timelockedBefore: number;
}
/** Return outputs that are timelocked after a certain Unix timestamp. */
interface TimelockedAfter {
    timelockedAfter: number;
}
/** Return outputs that are timelocked before a certain milestone index. */
interface TimelockedBeforeMilestone {
    timelockedBeforeMilestone: number;
}
/** Return outputs that are timelocked ater a certain milestone index. */
interface TimelockedAfterMilestone {
    timelockedAfterMilestone: number;
}
/** Filters outputs based on the presence of expiration unlock condition. */
interface HasExpirationCondition {
    hasExpirationCondition: boolean;
}
/** Return outputs that expire before a certain Unix timestamp. */
interface ExpiresBefore {
    expiresBefore: number;
}
/** Return outputs that expire after a certain Unix timestamp. */
interface ExpiresAfter {
    expiresAfter: number;
}
/** Return outputs that expire before a certain milestone index. */
interface ExpiresBeforeMilestone {
    expiresBeforeMilestone: number;
}
/** Return outputs that expire after a certain milestone index. */
interface ExpiresAfterMilestone {
    expiresAfterMilestone: number;
}
/** Filter outputs based on the presence of a specific Bech32-encoded return
 * address in the expiration unlock condition.
 * */
interface ExpirationReturnAddress {
    expirationReturnAddress: string;
}
/** Filter for a certain sender */
interface Sender {
    sender: string;
}
/** Filter for a certain tag */
interface Tag {
    tag: string;
}
/** Return outputs that were created before a certain Unix timestamp. */
interface CreatedBefore {
    createdBefore: number;
}
/** Return outputs that were created after a certain Unix timestamp. */
interface CreatedAfter {
    createdAfter: number;
}
/** Pass the cursor(confirmationMS+outputId.pageSize) to start the results from */
interface Cursor {
    cursor: string;
}
/** Filter for a certain issuer */
interface Issuer {
    issuer: string;
}
/** Filter outputs based on bech32-encoded state controller address. */
interface StateController {
    stateController: string;
}
/** Filter outputs based on bech32-encoded governor (governance controller) address. */
interface Governor {
    governor: string;
}
/** Define the page size for the results */
interface PageSize {
    pageSize: string;
}
