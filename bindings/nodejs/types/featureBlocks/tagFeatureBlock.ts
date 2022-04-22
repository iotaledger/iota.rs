// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the tag feature block.
 */
export const TAG_FEATURE_BLOCK_TYPE = 3;

export interface TagFeatureBlock extends TypeBase<3> {
    /**
     * Defines a tag for the data.
     */
    tag: string;
}
