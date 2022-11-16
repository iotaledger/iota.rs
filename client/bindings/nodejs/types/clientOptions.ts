// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { IMqttBrokerOptions, INetworkInfo, INode } from './network';

/** Options for the client builder */
export interface IClientOptions {
    /** Node which will be tried first for all requests */
    primaryNode?: string | INode;
    /** Node which will be tried first when using remote PoW, even before the primary_node */
    primaryPowNode?: string | INode;
    nodes?: Array<string | INode>;
    permanodes?: Array<string | INode>;
    /** If the node health status should be ignored */
    ignoreNodeHealth?: boolean;
    /** Interval in which nodes will be checked for their sync status and the NetworkInfo gets updated */
    nodeSyncInterval?: IDuration;
    /** If node quorum is enabled. Will compare the responses from multiple nodes and only returns the
     * response if quorum_threshold of the nodes return the same one
     */
    quorum?: boolean;
    /** Minimum amount of nodes required for request when quorum is enabled */
    minQuorumSize?: number;
    /** % of nodes that have to return the same response so it gets accepted */
    quorumThreshold?: number;
    /** Data related to the used network */
    networkInfo?: INetworkInfo;
    /** Options for the MQTT broker */
    brokerOptions?: IMqttBrokerOptions;
    /** Timeout for API requests */
    apiTimeout?: IDuration;
    /** Timeout when sending a block that requires remote proof of work */
    remotePowTimeout?: IDuration;
    /** The amount of threads to be used for proof of work */
    powWorkerCount?: number;
    /** Whether the PoW should be done locally or remotely. */
    localPow?: boolean;
}

/** Time duration */
export interface IDuration {
    secs: number;
    nanos: number;
}
