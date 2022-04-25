// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Network } from './network';

/**
 * Input options for GenerateAddresses
 */
export interface GenerateAddressesOptions {
    coinType?: number;
    accountIndex?: number;
    range?: {
        start: number;
        end: number;
    };
    /**
     * Internal addresses
     */
    internal?: boolean;
    /**
     * Bech32 human readable part
     */
    bech32Hrp?: string;
    metadata?: GenerateAddressMetadata;
}

/**
 * Metadata provided to Generate Address
 */
export interface GenerateAddressMetadata {
    /**
     * Indicates that the address is being generated as part of the account syncing process.
     * This means that the account might not be saved.
     * If it is false, the prompt will be displayed on ledger devices.
     */
    syncing: boolean;
    /**
     * The network which is used so the correct BIP32 path is used for the ledger. Debug mode starts with 44'/1' and
     * in mainnet-mode it's 44'/4218'
     */
    network: Network;
}
