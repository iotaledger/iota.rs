// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { sendMessageAsync, messageHandlerNew, listen } from './bindings';
import type { IClientOptions, __ClientMessages__ } from '../types';

/** The MessageHandler which sends the commands to the Rust side. */
export class MessageHandler {
    messageHandler: MessageHandler;

    constructor(options: IClientOptions) {
        this.messageHandler = messageHandlerNew(JSON.stringify(options));
    }

    async sendMessage(message: __ClientMessages__): Promise<string> {
        return sendMessageAsync(JSON.stringify(message), this.messageHandler);
    }

    // MQTT
    listen(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): void {
        return listen(topics, callback, this.messageHandler);
    }
}
