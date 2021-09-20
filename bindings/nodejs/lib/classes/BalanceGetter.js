// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const addon = require('../../index.node');
const utils = require('../utils.js');

const {
    balanceGetterNew,
} = addon;

// const methodNameAsync = utils.promisify(methodName);


class BalanceGetter {
    constructor(options) {
        this.balanceGetter = balanceGetterNew(JSON.stringify(options));
    }

    // id() {
    //     return id.apply(this.account);
    // }

    // async promote(messageId) {
    //     return await repostAsync.apply(this.balanceGetter, [messageId]);
    // }
}

module.exports.BalanceGetter = BalanceGetter;
