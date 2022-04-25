// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { OutputResponse } from './outputResponse';
import type { TransactionEssence } from './transactionEssences';

/**
 * Helper struct for offline signing
 */
export interface PreparedTransactionData {
    /**
     * Transaction essence
     */
    essence: TransactionEssence;
    /**
     * Required address information for signing
     */
    inputSigningDataEntries: InputSigningData[];
}

/**
 * Data for transaction inputs for signing and ordering of unlock blocks
 */
export interface InputSigningData {
    outputResponse: OutputResponse;
    /**
     * The chain derived from seed, only for ed25519 addresses
     */
    chain?: Segment[];
    /**
     * The bech32 encoded address, required because of alias outputs where we have multiple possible unlock
     * conditions, because we otherwise don't know which one we need
     */
    bech32Address: string;
}

export interface Segment {
    hardened: boolean;
    bs: number[];
}
