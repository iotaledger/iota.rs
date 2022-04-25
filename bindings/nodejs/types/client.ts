// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { MqttBrokerOptions, Node } from './network';

// TODO: update this type
export interface ClientOptions {
    offline?: boolean;
    primaryNode?: string | Node;
    primaryPoWNode?: string | Node;
    node?: string | Node;
    nodes?: Array<string | Node>;
    network?: string;
    mqttBrokerOptions?: MqttBrokerOptions;
    quorumSize?: number;
    quorumThreshold?: number;
    localPow?: boolean;
}
