// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { IUTXOInput, OutputTypes } from '@iota/types';
import type { IRange } from './range';

export interface IGenerateMessageOptions {
    coinType?: number;
    accountIndex?: number;
    initialAddressIndex?: number;
    inputs?: IUTXOInput[];
    inputRange?: IRange;
    /** Bech32 encoded output address and amount */
    output?: IClientMessageBuilderOutputAddress;
    /** Hex encoded output address and amount */
    outputHex?: IClientMessageBuilderOutputAddress;
    outputs?: OutputTypes[];
    customRemainderAddress?: string;
    tag?: number[];
    data?: number[];
    /** Parent message IDs */
    parents?: string[];
    /** Allow burning of native tokens */
    allowBurning?: boolean;
}

export interface IClientMessageBuilderOutputAddress {
    address: string;
    amount: string;
}
