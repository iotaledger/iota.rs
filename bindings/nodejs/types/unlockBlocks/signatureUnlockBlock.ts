// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Ed25519Signature } from '../ed25519Signature';
import type { TypeBase } from '../typeBase';

/**
 * The global type for the unlock block.
 */
export const SIGNATURE_UNLOCK_BLOCK_TYPE = 0;

/**
 * An unlock block holding one or more signatures unlocking one or more inputs..
 */
export interface SignatureUnlockBlock extends TypeBase<0> {
    /**
     * The signature.
     */
    signature: Ed25519Signature;
}
