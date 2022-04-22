// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Payload } from './payloads';

export interface Message {
    /**
     * Protocol version identifier. It also tells which protocol rules apply to the message.
     */
    protocolVersion: number;
    /**
     * The identifiers of the messages this message references. Hex-encoded data with 0x prefix.
     */
    parentMessageIds: string[];
    /**
     * The optional Payload of the message.
     */

    payload?: Payload;
    /**
     * The result of the Proof of Work in order for the message to be accepted into the tangle.
     * Plain string encoded number.
     */
    nonce: number;
}
