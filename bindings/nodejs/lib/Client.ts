// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {MessageHandler} from './MessageHandler';
import type {ClientOptions} from "../types";

export class Client {
    private messageHandler: MessageHandler;
    
    constructor(options: ClientOptions) {
        this.messageHandler = new MessageHandler(options);
    }

    async getInfo() {
        return JSON.parse(await this.messageHandler.sendMessage({
            cmd: 'CallClientMethod',
            payload: {
                name: 'GetInfo'
            },
        })).payload;
    }

    // MQTT
    async listen(topics: any, callback: any) {
        return this.messageHandler.listen(topics, callback);
    }
}

