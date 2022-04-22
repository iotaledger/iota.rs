// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Output } from './outputs';

export interface OutputResponse {
    messageId: string;
    transactionId: string;
    outputIndex: number;
    isSpent: boolean;
    milestoneIndexSpent?: number;
    milestoneTimestampSpent?: number;
    transactionIdSpent?: string;
    milestoneIndexBooked: number;
    milestoneTimestampBooked: number;
    ledgerIndex: number;
    // TODO: Verify this type is correct.
    output: Output;
}
