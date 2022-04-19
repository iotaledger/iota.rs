// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { ClientOptions } from '../types';
import type {
    __ClientPayloadMethods__,
    __SendMessagePayload__,
} from '../types/bridge';
import { sendMessageAsync, messageHandlerNew, listen } from './bindings';

export class MessageHandler {
    messageHandler: any;

    constructor(options: ClientOptions) {
        this.messageHandler = messageHandlerNew(JSON.stringify(options));
    }

    async sendMessage(message: __SendMessagePayload__): Promise<string> {
        return sendMessageAsync(JSON.stringify(message), this.messageHandler);
    }

    async callClientMethod(method: __ClientPayloadMethods__): Promise<string> {
        return this.sendMessage({
            cmd: 'CallClientMethod',
            payload: method,
        });
    }

    // MQTT
    listen(
        topics: any,
        callback: (error: Error, result: string) => void,
    ): void {
        return listen(topics, callback, this.messageHandler);
    }
}
