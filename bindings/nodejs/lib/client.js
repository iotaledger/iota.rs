// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const mh = require('./messageHandler.js');
let { MessageHandler } = mh;

class Client {
    constructor(options) {
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
}

module.exports.Client = Client;
