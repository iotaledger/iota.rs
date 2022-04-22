// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the ed25519 address type.
 */
export const ED25519_ADDRESS_TYPE = 0;

/**
 * Ed25519Address address.
 */
export interface Ed25519Address extends TypeBase<0> {
    /**
     * The public key hash.
     */
    pubKeyHash: string;
}
