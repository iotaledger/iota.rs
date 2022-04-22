// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { AliasAddress } from './aliasAddress';
import type { Ed25519Address } from './ed25519Address';
import type { NftAddress } from './nftAddress';

/**
 * All of the address types.
 */
export type Address = Ed25519Address | AliasAddress | NftAddress;
