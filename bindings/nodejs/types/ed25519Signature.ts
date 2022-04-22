// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from './typeBase';

/**
 * The global type for the signature type.
 */
export const ED25519_SIGNATURE_TYPE = 0;

/**
 * Ed25519 signature.
 */
export interface Ed25519Signature extends TypeBase<0> {
    /**
     * The public key.
     */
    publicKey: string;

    /**
     * The signature.
     */
    signature: string;
}
