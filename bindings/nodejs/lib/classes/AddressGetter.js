// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const addon = require('../../index.node');
const utils = require('../utils.js');

const {
    addressGetterNew,
} = addon;

// const methodNameAsync = utils.promisify(methodName);


class AddressGetter {
    constructor(options) {
        this.addressGetter = addressGetterNew(JSON.stringify(options));
    }

    // id() {
    //     return id.apply(this.account);
    // }

    // async promote(messageId) {
    //     return await repostAsync.apply(this.addressGetter, [messageId]);
    // }
}

module.exports.AddressGetter = AddressGetter;
