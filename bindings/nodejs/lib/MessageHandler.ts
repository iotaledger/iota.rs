// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {sendMessageAsync, messageHandlerNew, listen} from './bindings';

export class MessageHandler {
    messageHandler: any;

    constructor(options: any) {
        this.messageHandler = messageHandlerNew(JSON.stringify(options));
    }

    async sendMessage(message: unknown): Promise<string> {
        return sendMessageAsync(JSON.stringify(message), this.messageHandler);
    }

    // MQTT
    listen(topics: any, callback: (error: Error, result: string) => void): void {
        return listen(topics, callback, this.messageHandler);
    }
}
