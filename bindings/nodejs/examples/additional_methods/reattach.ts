// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/additional_methods/reattach.js

// In this example we will reattach a message
// Messages can be reattached only if they are valid and haven't been confirmed for a while.
async function run() {
    initLogger();

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

        const messageId = await client.postMessage(message);
        console.log('Message Id: ', messageId, '\n');

        // Reattach a message without checking if it should be reattached
        const reattachUnchecked = await client.reattachUnchecked(messageId);
        console.log('Reattached message: ', reattachUnchecked);

        // Returns expected error: no need to promote or reattach.
        const reattach = await client.reattach(messageId);
        console.log('Reattached message: ', reattach);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
