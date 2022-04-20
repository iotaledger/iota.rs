// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { sendMessageAsync, messageHandlerNew, listen } from './bindings';
import type {
    ClientOptions,
    __ClientPayloadMethods__,
    __SendMessagePayload__,
} from '../types';

export class MessageHandler {
    messageHandler: MessageHandler;

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
        } as __SendMessagePayload__);
    }

    // MQTT
    listen(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): void {
        return listen(topics, callback, this.messageHandler);
    }
}
