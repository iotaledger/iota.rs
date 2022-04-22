// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { FeatureBlock } from '../featureBlocks';
import type { TypeBase } from '../typeBase';
import type { CommonOutput } from './commonOutput';

/**
 * The global type for the NFT output.
 */
export const NFT_OUTPUT_TYPE = 6;

/**
 * NFT output.
 */
export interface NftOutput extends TypeBase<6>, CommonOutput {
    /**
     * Amount of IOTA tokens held by the output.
     */
    amount: string;
    /**
     * Unique identifier of the NFT, which is the BLAKE2b-160 hash of the Output ID that created it.
     */
    nftId: string;
    /**
     * Immutable blocks contained by the output.
     */
    immutableFeatureBlocks: FeatureBlock[];
}
