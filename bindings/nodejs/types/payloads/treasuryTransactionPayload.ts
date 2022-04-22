// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Input } from '../inputs';
import type { TypeBase } from '../typeBase';
import type { Output } from '../outputs';

/**
 * The global type for the payload.
 */
export const TREASURY_TRANSACTION_PAYLOAD_TYPE = 4;

/**
 * Receipt payload.
 */
export interface TreasuryTransactionPayload extends TypeBase<4> {
    /**
     * The input of this transaction.
     */
    input: Input;

    /**
     * The output of this transaction.
     */
    output: Output;
}
