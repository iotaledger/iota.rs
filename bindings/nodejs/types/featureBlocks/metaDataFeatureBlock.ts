// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the metadata feature block.
 */
export const METADATA_FEATURE_BLOCK_TYPE = 2;

export interface MetadataFeatureBlock extends TypeBase<2> {
    /**
     * Defines metadata (arbitrary binary data) that will be stored in the output.
     */
    data: string;
}
