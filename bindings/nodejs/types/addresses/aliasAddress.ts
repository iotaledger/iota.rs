// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the alias address type.
 */
export const ALIAS_ADDRESS_TYPE = 8;

/**
 * Alias address.
 */
export interface AliasAddress extends TypeBase<8> {
    /**
     * The alias id.
     */
    aliasId: string;
}
