// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const addon = require('../index.node');
const mh = require('./messageHandler.js');
const {Client} = require('./client.js');

let { initLogger } = addon;
let { MessageHandler } = mh;

module.exports = {
  initLogger: (config) => initLogger(JSON.stringify(config)),
  MessageHandler,
  Client
}
