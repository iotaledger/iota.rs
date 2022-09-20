// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {
    AddressTypes,
    IOutputMetadataResponse,
    ITransactionEssence,
    OutputTypes,
} from '@iota/types';

/**
 * Helper struct for offline signing
 */
export interface IPreparedTransactionData {
    /**
     * Transaction essence
     */
    essence: ITransactionEssence;
    /**
     * Required address information for signing
     */
    inputsData: IInputSigningData[];
    /**
     * Optional remainder output information
     */
    remainder?: IRemainder;
}

/**
 * Data for transaction inputs for signing and ordering of unlock blocks
 */
export interface IInputSigningData {
    /**
     * The output
     */
    output: OutputTypes;
    /**
     * The output metadata
     */
    outputMetaData: IOutputMetadataResponse;
    /**
     * The chain derived from seed, only for ed25519 addresses
     */
    chain?: ISegment[];
    /**
     * The bech32 encoded address, required because of alias outputs where we have multiple possible unlock
     * conditions, because we otherwise don't know which one we need
     */
    bech32Address: string;
}

export interface IRemainder {
    /**
     * The remainder output
     */
    output: OutputTypes;
    /**
     * The chain derived from seed, for the remainder addresses
     */
    chain?: ISegment[];
    /**
     * The remainder address
     */
    address: AddressTypes;
}
export interface ISegment {
    hardened: boolean;
    bs: Uint8Array;
}
