// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Query parameter for filtering output requests
 */
export type QueryParameter =
    | Address
    | AliasAddress
    | HasStorageReturnCondition
    | StorageReturnAddress
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
    | Issuer
    | StateController
    | Governor
    | CommonQueryParameters;

/** Query parameters for filtering Alias Outputs */
export type AliasQueryParameter =
    | StateController
    | Governor
    | Issuer
    | Sender
    | CommonQueryParameters;

/** Query parameters for filtering Foundry Outputs */
export type FoundryQueryParameter = AliasAddress | CommonQueryParameters;

/** Query parameters for filtering Nft Outputs */
export type NftQueryParameter =
    | Address
    | AliasAddress
    | HasStorageReturnCondition
    | StorageReturnAddress
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
    | CommonQueryParameters;

/** Shared query parameters*/
type CommonQueryParameters =
    | HasNativeTokens
    | MinNativeTokenCount
    | MaxNativeTokenCount
    | CreatedAfter
    | CreatedBefore
    | PageSize
    | Cursor;

/** Bech32-encoded address that should be searched for. */
interface Address {
    address: string;
}
/** Filter foundry outputs based on bech32-encoded address of the controlling alias. */
interface AliasAddress {
    aliasAddress: string;
}
/** Filters outputs based on the presence of storage return unlockcondition. */
interface HasStorageReturnCondition {
    hasStorageReturnCondition: boolean;
}
/** Filter outputs based on the presence of a specific Bech32-encoded return address
 * in the storage return unlock condition.
 */
interface StorageReturnAddress {
    storageReturnAddress: string;
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
/** Filters outputs based on the presence of native tokens. */
interface HasNativeTokens {
    hasNativeTokens: boolean;
}
/** Filters outputs that have at most a certain number of distinct native tokens. */
interface MaxNativeTokenCount {
    maxNativeTokenCount: number;
}
/** Filters outputs that have at least a certain number of distinct native tokens. */
interface MinNativeTokenCount {
    minNativeTokenCount: number;
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
