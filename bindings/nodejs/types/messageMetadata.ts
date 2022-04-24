// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Response from the metadata endpoint.
 */
export interface MessageMetadata {
    /**
     * The message id.
     */
    messageId: string;
    /**
     * The parent message ids.
     */
    parentMessageIds?: string[];
    /**
     * Is the message solid.
     */
    isSolid: boolean;
    /**
     * Is the message referenced by a milestone.
     */
    referencedByMilestoneIndex?: number;
    /**
     * Is this message a valid milestone.
     */
    milestoneIndex?: number;
    /**
     * The ledger inclusion state.
     */
    ledgerInclusionState?: 'noTransaction' | 'included' | 'conflicting';
    /**
     * The conflict reason.
     */
    conflictReason?: number;
    /**
     * Should the message be promoted.
     */
    shouldPromote?: boolean;
    /**
     * Should the message be reattached.
     */
    shouldReattach?: boolean;
}
