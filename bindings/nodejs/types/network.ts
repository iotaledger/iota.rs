// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Network types.
 */
export enum Network {
    Mainnet,
    Testnet,
}

/**
 * Basic Auth or JWT.
 */
export interface IAuth {
    jwt?: string;
    username?: string;
    password?: string;
}

/**
 * Options for the MQTT broker.
 */
export interface IMqttBrokerOptions {
    automaticDisconnect?: boolean;
    /** timeout in seconds */
    timeout?: number;
    useWs?: boolean;
    port?: number;
    maxReconnectionAttempts?: number;
}

/**
 * A node object for the client.
 */
export interface INode {
    url: string;
    auth?: IAuth;
    disabled?: boolean;
}

/**
 * Struct containing network and PoW related information
 */
export interface INetworkInfo {
    network?: Network;
    networkId?: number;
    bech32Hrp: string;
    /** Mininum proof of work score*/
    minPowScore: number;
    /** Local proof of work */
    localPow: boolean;
    /** Fallback to local proof of work if the node doesn't support remote Pow */
    fallbackToLocalPow: boolean;
    /** Tips request interval during PoW in seconds */
    tipsInterval: number;
    /** Rent structure of the protocol */
    rentStructure: IRentStructureResponse;
}

/**
 * Rent information about the node.
 */
export interface IRentStructureResponse {
    vByteCost: number;
    vByteFactorKey: number;
    vByteFactorData: number;
}
