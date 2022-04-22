// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TreasuryInput } from './treasuryInput';
import type { UTXOInput } from './UTXOInput';

/**
 * All of the input types.
 */
export type Input = UTXOInput | TreasuryInput;
