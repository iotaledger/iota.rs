// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { MilestonePayload } from './milestonePayload';
import type { ReceiptPayload } from './receiptPayload';
import type { TaggedDataPayload } from './taggedDataPayload';
import type { TransactionPayload } from './transactionPayload';
import type { TreasuryTransactionPayload } from './treasuryTransactionPayload';

/**
 * All of the payload types.
 */
export type Payload =
    | TransactionPayload
    | MilestonePayload
    | ReceiptPayload
    | TaggedDataPayload
    | TreasuryTransactionPayload;
