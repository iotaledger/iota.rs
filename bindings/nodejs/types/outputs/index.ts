// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { AliasOutput } from './aliasOutput';
import type { BasicOutput } from './basicOutputs';
import type { FoundryOutput } from './foundryOutput';
import type { NftOutput } from './nftOutput';
import type { TreasuryOutput } from './treasuryOutput';

/**
 * Describes all the different output types.
 */
export type Output =
    | TreasuryOutput
    | BasicOutput
    | AliasOutput
    | FoundryOutput
    | NftOutput;
