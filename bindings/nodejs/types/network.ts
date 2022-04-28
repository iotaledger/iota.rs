// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
export enum Network {
    Mainnet,
    Testnet,
}
export interface IAuth {
    jwt?: string;
    username?: string;
    password?: string;
}

export interface IMqttBrokerOptions {
    automaticDisconnect?: boolean;
    /** timeout in seconds */
    timeout?: number;
    useWs?: boolean;
    port?: number;
    maxReconnectionAttempts?: number;
}

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
