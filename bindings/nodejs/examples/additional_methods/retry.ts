// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/additional_methods/retry.js

// In this example we will retry (promote or reattach) a message
// Message should be retried only if they are valid and haven't been confirmed for a while.
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

        // Retries (promotes or reattaches) a message for provided message id until it's included
        // (referenced by a milestone). Default interval is 5 seconds and max attempts is 40.
        const retryUntilIncluded = await client.retryUntilIncluded(messageId);
        console.log('Retried message: ', retryUntilIncluded);

        // Returns expected error: no need to promote or reattach.
        const retry = await client.retry(messageId);
        console.log('Retried message: ', retry);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
