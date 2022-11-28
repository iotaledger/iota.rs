// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/10_mqtt.js

// Initialize MQTT listener
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    // Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
    const client = new Client({
        nodes: [process.env.NODE_URL],
    });

    // Array of topics to subscribe to
    // Topics can be found here https://studio.asyncapi.com/?url=https://raw.githubusercontent.com/iotaledger/tips/stardust-event-api/tips/TIP-0028/event-api.yml
    const topics = ['blocks'];

    const callback = function (error: Error, data: string) {
        console.log(JSON.parse(data));
    };

    client.listen(topics, callback);
}

run();
