// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { IUTXOInput, OutputTypes } from '@iota/types';
import type { CoinType } from '../lib';
import type { IRange } from './range';

/** Options to build a new block, possibly with payloads */
export interface IBuildBlockOptions {
    coinType?: CoinType;
    accountIndex?: number;
    initialAddressIndex?: number;
    inputs?: IUTXOInput[];
    inputRange?: IRange;
    /** Bech32 encoded output address and amount */
    output?: IClientBlockBuilderOutputAddress;
    /** Hex encoded output address and amount */
    outputHex?: IClientBlockBuilderOutputAddress;
    outputs?: OutputTypes[];
    customRemainderAddress?: string;
    tag?: string;
    data?: string;
    /** Parent block IDs */
    parents?: string[];
    /** Allow burning of native tokens */
    allowBurning?: boolean;
}

/** Address with base coin amount */
export interface IClientBlockBuilderOutputAddress {
    address: string;
    amount: string;
}
