// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { INodeInfoMetrics, INodeInfoProtocol } from '@iota/types';

/** NodeInfo wrapper which contains the nodeinfo and the url from the node (useful when multiple nodes are used) */
export interface INodeInfoWrapper {
    nodeInfo: INodeInfo;
    url: string;
}

// Temporarily implemented type until @iota/types INodeInfo type is updated
/**
 * Response from the /info endpoint.
 */
export interface INodeInfo {
    /**
     * The name of the node software.
     */
    name: string;
    /**
     * The version of the software running on the node.
     */
    version: string;
    /**
     * The status of the node.
     */
    status: INodeInfoStatus;
    /**
     * The protocol information of the node.
     */
    protocol: INodeInfoProtocol;
    /**
     * The metrics for the node.
     */
    metrics: INodeInfoMetrics;
    /**
     * Information about the base token.
     */
    baseToken: IBaseToken;
    /**
     * Features supported by the node.
     */
    features: string[];
    /**
     * The plugins the node exposes.
     */
    plugins: string[];
}

export interface INodeInfoStatus {
    /**
     * Is the node healthy.
     */
    isHealthy: boolean;
    /**
     * The latest milestone timestamp.
     */
    latestMilestone: IMilestone;
    /**
     * The confirmed milestone index.
     */
    confirmedMilestone: IMilestone;
    /**
     * The pruning index.
     */
    pruningIndex: number;
}

export interface IMilestone {
    index: number;
    timestamp: number;
    milestoneId: string;
}

export interface IBaseToken {
    name: string;
    tickerSymbol: string;
    unit: string;
    decimals: number;
    subunit: string;
    useMetricPrefix: boolean;
}
