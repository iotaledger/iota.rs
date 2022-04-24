// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { UTXOInput } from './inputs/UTXOInput';
import type { Output } from './outputs';

export interface GenerateMessageOptions {
    coinType?: number;
    accountIndex?: number;
    initialAddressIndex?: number;
    inputs?: UTXOInput[];
    inputRange?: {
        start: number;
        end: number;
    };
    /** Bech32 encoded output address and amount */
    output?: ClientMessageBuilderOutputAddress;
    /** Hex encoded output address and amount */
    outputHex?: ClientMessageBuilderOutputAddress;
    outputs?: Output[];
    customRemainderAddress?: string;
    tag?: number[];
    data?: number[];
    /** Parent message IDs */
    parents?: string[];
}

export interface ClientMessageBuilderOutputAddress {
    address: string;
    amount: number;
}
