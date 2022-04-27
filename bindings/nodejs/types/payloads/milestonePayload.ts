// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { Ed25519Signature } from '../ed25519Signature';
import type { MilestoneOption } from '../milestoneOptions';
import type { TypeBase } from '../typeBase';

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
     * The timestamp of the milestone.
     */
    lastMilestoneId: string;

    /**
     * The parents where this milestone attaches to.
     */
    parentMessageIds: string[];

    /**
     * The Merkle tree hash of all messages confirmed by this milestone.
     */
    confirmedMerkleRoot: string;

    /**
     * The Merkle tree hash of all messages applied by this milestone.
     */
    appliedMerkleRoot: string;

    /**
     * The metadata.
     */
    metadata: string;

    /**
     * The milestone options.
     */
    options?: MilestoneOption[];

    /**
     * The signatures.
     */
    signatures: Ed25519Signature[];
}
