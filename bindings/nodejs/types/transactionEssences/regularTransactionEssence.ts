// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Input } from '../inputs';
import type { Output } from '../outputs';
import type { Payload } from '../payloads';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the Regular Transaction Essence.
 */
export const REGULAR_TRANSACTION_ESSENCE_TYPE = 1;

/**
 * Describes the essence data making up a transaction by defining its inputs and outputs and an optional payload.
 */
export interface RegularTransactionEssence extends TypeBase<1> {
    /**
     * The network id of the message.
     */
    networkId: string;
    /**
     * The inputs of the transaction.
     */
    inputs: Input[];
    /**
     * The commitment to the referenced inputs.
     */
    inputsCommitment: string;
    /**
     * The outputs of the transaction.
     */
    outputs: Output[];
    /**
     * Tagged data payload.
     */
    payload?: Payload;
}
