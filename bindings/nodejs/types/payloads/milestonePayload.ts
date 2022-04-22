// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Ed25519Signature } from '../ed25519Signature';
import type { TypeBase } from '../typeBase';
import type { ReceiptPayload } from './receiptPayload';

/**
 * The global type for the payload.
 */
export const MILESTONE_PAYLOAD_TYPE = 7;

/**
 * Milestone payload.
 */
export interface MilestonePayload extends TypeBase<7> {
    /**
     * The index name.
     */
    index: number;
    /**
     * The timestamp of the milestone.
     */
    timestamp: number;
    /**
     * The parents where this milestone attaches to.
     */
    parentMessageIds: string[];
    /**
     * The merkle proof inclusions.
     */
    inclusionMerkleProof: string;
    /**
     * The next PoW score.
     */
    nextPoWScore: number;
    /**
     * The milestone at which the next PoW score becomes active.
     */
    nextPoWScoreMilestoneIndex: number;
    /**
     * The metadata.
     */
    metadata: string;
    /**
     * The signatures.
     */
    signatures: Ed25519Signature[];
    /**
     * Receipt payload.
     */
    receipt?: ReceiptPayload;
}
