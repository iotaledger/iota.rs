export type Auth = {
    jwt?: string;
    username?: string;
    password?: string;
};

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
    isHealthy: boolean;
    networkId: string;
    bech32HRP: string;
    minPoWScore: number;
    messagesPerSecond: number;
    referencedMessagesPerSecond: number;
    referencedRate: number;
    latestMilestoneTimestamp: number;
    latestMilestoneIndex: number;
    confirmedMilestoneIndex: number;
    pruningIndex: number;
    features: string[];
}
