// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
export enum Network {
    Mainnet,
    Testnet,
}
export interface Auth {
    jwt?: string;
    username?: string;
    password?: string;
}

export interface MqttBrokerOptions {
    automaticDisconnect?: boolean;
    /** timeout in seconds */
    timeout?: number;
    useWs?: boolean;
    port?: number;
    maxReconnectionAttempts?: number;
}

export type Node = {
    url: string;
    auth?: Auth;
    disabled?: boolean;
};

export interface NodeInfo {
    name: string;
    version: string;
    status: Status;
    metrics: Metrics;
    protocol: Protocol;
    features: string[];
    plugins: string[];
    url: string;
}

export interface Status {
    isHealthy: boolean;
    latestMilestoneTimestamp: number;
    latestMilestoneIndex: number;
    confirmedMilestoneIndex: number;
    pruningIndex: number;
}

export interface Metrics {
    messagesPerSecond: number;
    referencedMessagesPerSecond: number;
    referencedRate: number;
}

export interface Protocol {
    networkName: string;
    bech32HRP: string;
    minPoWScore: number;
    rentStructure?: {
        vByteCost: number;
        vByteFactorData: number;
        vByteFactorKey: number;
    };
}

/**
 * Struct containing network and PoW related information
 */
export interface NetworkInfo {
    network?: Network;
    networkId?: number;
    bech32HRP: number;
    /** Mininum proof of work score*/
    minPowScore: number;
    /** Local proof of work */
    localPow: boolean;
    /** Fallback to local proof of work if the node doesn't support remote PoW */
    fallbackToLocalPow: boolean;
    /** Tips request interval during PoW in seconds */
    tipsInterval: number;
    /** Rent structure of the protocol */
    rentStructure: RentStructureResponse;
}

/**
 * Rent information about the node.
 */
export interface RentStructureResponse {
    vByteCost: number;
    vByteFactorKey: number;
    vByteFactorData: number;
}
