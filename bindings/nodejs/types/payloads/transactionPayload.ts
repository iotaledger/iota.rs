// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TransactionEssence } from '../transactionEssences';
import type { TypeBase } from '../typeBase';
import type { UnlockBlock } from '../unlockBlocks';

/**
 * The global type for the payload.
 */
export const TRANSACTION_PAYLOAD_TYPE = 6;

/**
 * Transaction payload.
 */
export interface TransactionPayload extends TypeBase<6> {
    /**
     * The index name.
     */
    essence: TransactionEssence;
    /**
     * The unlock blocks.
     */
    unlockBlocks: UnlockBlock;
}
