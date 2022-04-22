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
    // timeout in seconds
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
