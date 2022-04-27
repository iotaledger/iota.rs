// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { MigratedFunds } from '../migratedFunds';
import type { TypeBase } from '../typeBase';
import type { TreasuryTransactionPayload } from '../payloads/treasuryTransactionPayload';

/**
 * The global type for the option.
 */
export const RECEIPT_MILESTONE_OPTION_TYPE = 0;

/**
 * Receipt milestone option.
 */
export interface ReceiptMilestoneOption extends TypeBase<0> {
    /**
     * The milestone index at which the funds were migrated in the legacy network.
     */
    migratedAt: number;
    /**
     * Whether this Receipt is the final one for a given migrated at index.
     */
    final: boolean;
    /**
     * The index data.
     */
    funds: MigratedFunds[];
    /**
     * The TreasuryTransaction used to fund the funds.
     */
    transaction: TreasuryTransactionPayload;
}
