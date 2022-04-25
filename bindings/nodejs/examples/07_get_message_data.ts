// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/07_get_message_data.js

// In this example we will send a message and get the data and metadata for it
async function run() {
    initLogger({
        colorEnabled: true,
        name: './client.log',
        levelFilter: 'debug',
    });

    // client will connect to testnet by default
    const client = new Client({
        nodes: [
            {
                // Insert your node URL here.
                url: 'http://localhost:14265',
                disabled: false,
            },
        ],
        localPow: true,
    });

    try {
        // Create message with no payload
        const message = await client.generateMessage();
        console.log('Message:', message, '\n');

        // Send message
        const messageId = await client.postMessage(message);

        const messageData = await client.getMessageData(messageId);
        const messageMetadata = await client.getMessageMetadata(messageId);

        console.log('Message data: ', messageData, '\n');
        console.log('Message metadata: ', messageMetadata, '\n');
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
