// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from './addresses';

/**
 * The migrated funds for receipts.
 */
export interface MigratedFunds {
    /**
     * The tail transaction hash of the migration bundle.
     */
    tailTransactionHash: string;
    /**
     * The target address of the migrated funds.
     */
    address: Address;
    /**
     * The amount of the deposit.
     */
    amount: string;
}
