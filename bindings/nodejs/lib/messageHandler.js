// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const addon = require('../build/Release/index.node');
const utils = require('./utils.js');

let { sendMessage, messageHandlerNew, listen } = addon;

const sendMessageAsync = utils.promisify(sendMessage);
const listenAsync = utils.promisify(listen);

class MessageHandler {
    constructor(options) {
        this.messageHandler = messageHandlerNew(JSON.stringify(options));
    }

    async sendMessage(message) {
        return sendMessageAsync(JSON.stringify(message), this.messageHandler);
    }

    // MQTT
    async listen(topics, callback) {
        return listenAsync(topics, callback, this.messageHandler);
    }
}

module.exports.MessageHandler = MessageHandler;
