// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const addon = require('../../index.node');
const utils = require('../utils.js');

const {
    clientBuilderNew,
} = addon;

// const methodNameAsync = utils.promisify(methodName);


class ClientBuilder {
    constructor(options) {
        this.clientBuilder = clientBuilderNew(JSON.stringify(options));
    }

    // id() {
    //     return id.apply(this.account);
    // }

    // async promote(messageId) {
    //     return await repostAsync.apply(this.clientBuilder, [messageId]);
    // }
}

module.exports.ClientBuilder = ClientBuilder;
