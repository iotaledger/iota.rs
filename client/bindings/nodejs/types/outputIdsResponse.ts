// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * OutputIdsResponse.
 */
export interface OutputIdsResponse {
    ledgerIndex: number;
    cursor?: string;
    items: string[];
}
