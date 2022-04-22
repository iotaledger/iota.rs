// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the input.
 */
export const UTXO_INPUT_TYPE = 0;

/**
 * UTXO Transaction Input.
 */
export interface UTXOInput extends TypeBase<0> {
    /**
     * The transaction Id.
     */
    transactionId: string;

    /**
     * The output index.
     */
    transactionOutputIndex: number;
}
