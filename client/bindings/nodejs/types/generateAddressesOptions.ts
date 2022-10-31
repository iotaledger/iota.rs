// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { CoinType } from '../lib';
import type { IRange } from './range';

/**
 * Input options for GenerateAddresses
 */
export interface IGenerateAddressesOptions {
    coinType?: CoinType;
    accountIndex?: number;
    range?: IRange;
    /**
     * Internal addresses
     */
    internal?: boolean;
    /**
     * Bech32 human readable part
     */
    bech32Hrp?: string;
    options?: IGenerateAddressOptions;
}

/**
 * Options provided to Generate Address
 */
export interface IGenerateAddressOptions {
    /**
     * Display the address on ledger devices.
     */
    ledgerNanoPrompt: boolean;
}
