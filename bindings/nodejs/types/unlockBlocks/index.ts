// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { AliasUnlockBlock } from './aliasUnlockBlock';
import type { NftUnlockBlock } from './nftUnlockBlock';
import type { ReferenceUnlockBlock } from './referenceUnlockBlock';
import type { SignatureUnlockBlock } from './signatureUnlockBlock';

/**
 * All of the unlock block types.
 */
export type UnlockBlock =
    | AliasUnlockBlock
    | NftUnlockBlock
    | ReferenceUnlockBlock
    | SignatureUnlockBlock;
