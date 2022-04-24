// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will send a message and get the data and metadata for it
async function run() {
    const { Client } = require('@iota/client');

    // client will connect to testnet by default
    const client = new Client({
        nodes: [
            {
                url: 'http://localhost:14265',
                auth: null,
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
        console.log(error);
    }
}

run().then(() => process.exit());
