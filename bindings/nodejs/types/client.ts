// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { MqttBrokerOptions, NetworkInfo, Node } from './network';

// TODO: update this type
export interface ClientOptions {
    /** If the Client should be able to use without a node connection */
    offline?: boolean;
    /** Node which will be tried first for all requests */
    primaryNode?: string | Node;
    /** Node which will be tried first when using remote PoW, even before the primary_node */
    primaryPoWNode?: string | Node;
    nodes?: Array<string | Node>;
    permanodes?: Array<string | Node>;
    /** If node syncing is enabled */
    nodeSyncEnabled?: boolean;
    /** Interval in which nodes will be checked for their sync status and the NetworkInfo gets updated */
    nodeSyncInterval?: Duration;
    /** If node quorum is enabled. Will compare the responses from multiple nodes and only returns the
     * response if quorum_threshold of the nodes return the same one
     */
    quorum?: boolean;
    /** Minimum amount of nodes required for request when quorum is enabled */
    minQuorumSize?: number;
    /** % of nodes that have to return the same response so it gets accepted */
    quorumThreshold?: number;
    /** Data related to the used network */
    networkInfo?: NetworkInfo;
    /** Options for the MQTT broker */
    brokerOptions?: MqttBrokerOptions;
    /** Timeout for API requests */
    apiTimeout?: Duration;
    /** Timeout when sending a message that requires remote proof of work */
    remotePowTimeout?: Duration;
    /** The amount of threads to be used for proof of work */
    powWorkerCount?: number;
    /** Whether the PoW should be done locally or remotely. */
    localPow?: boolean;
}

export interface Duration {
    secs: number;
    nanos: number;
}
