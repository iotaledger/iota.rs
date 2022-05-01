// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/additional_methods/promote.js

// In this example we will promote a message
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

        // Promote a message without checking if it should be promoted
        const promoteUnchecked = await client.promoteUnchecked(messageId);
        console.log('Promoted message: ', promoteUnchecked);

        // Returns expected error: no need to promote or reattach.
        const promote = await client.promote(messageId);
        console.log('Promote message: ', promote);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
