// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';

/**
 * The global type for the payload.
 */
export const TAGGED_DATA_PAYLOAD_TYPE = 5;

/**
 * Tagged data payload.
 */
export interface TaggedDataPayload extends TypeBase<5> {
    /**
     * The hex encoded tag used to categorize the data.
     */
    tag?: string;

    /**
     * The hex encoded data.
     */
    data?: string;
}
