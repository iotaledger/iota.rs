// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Address } from '../addresses';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the sender feature block.
 */
export const SENDER_FEATURE_BLOCK_TYPE = 0;

export interface SenderFeatureBlock extends TypeBase<0> {
    address: Address;
}
