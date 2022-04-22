// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { Address } from '../addresses';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the issuer feature block.
 */
export const ISSUER_FEATURE_BLOCK_TYPE = 1;

export interface IssuerFeatureBlock extends TypeBase<1> {
    address: Address;
}
