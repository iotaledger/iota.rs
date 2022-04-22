// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the NFT address type.
 */
export const NFT_ADDRESS_TYPE = 16;

/**
 * NFT address.
 */
export interface NftAddress extends TypeBase<16> {
    /**
     * The NFT Id.
     */
    nftId: string;
}
