// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/10_mqtt.js

// Initialize MQTT listener
async function run() {
    initLogger();

    // client connects to a node that has MQTT enabled
    const client = new Client({
        nodes: ['http://localhost:14265'],
    });

    // Array of topics to subscribe to
    const topics = ['messages'];

    const callback = function (error: Error, data: string) {
        console.log(JSON.parse(data));
    };

    client.listen(topics, callback);
}

run();
