// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will send a message without a payload
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

        // TODO: link doesn't work (Not found), same goes for the rust example (06_simple_message.rs)
        console.log(
            `Empty message sent: https://explorer.iota.org/devnet/message/${messageId}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
