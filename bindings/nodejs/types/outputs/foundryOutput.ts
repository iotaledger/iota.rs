// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { FeatureBlock } from '../featureBlocks';
import type { TokenScheme } from '../tokenSchemes';
import type { TypeBase } from '../typeBase';
import type { CommonOutput } from './commonOutput';

/**
 * The global type for the foundry output.
 */
export const FOUNDRY_OUTPUT_TYPE = 5;

export interface FoundryOutput extends TypeBase<5>, CommonOutput {
    /**
     * Amount of IOTA tokens held by the output.
     */
    amount: string;
    // The serial number of the foundry with respect to the controlling alias.
    serialNumber: number;
    // Data that is always the last 12 bytes of ID of the tokens produced by this foundry.
    tokenTag: string;
    tokenScheme: TokenScheme;
    immutableFeatureBlocks: FeatureBlock[];
}
