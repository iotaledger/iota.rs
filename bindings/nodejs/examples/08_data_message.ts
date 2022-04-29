// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, utf8ToBytes, hexToUtf8, initLogger } from '@iota/client';

// Run with command:
// node ./dist/08_data_message.js

// In this example we will send a message with a tagged data payload
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

    const options = {
        tag: utf8ToBytes('Hello'),
        data: utf8ToBytes('Tangle'),
    };
    try {
        const mnemonic = await client.generateMnemonic();
        const secretManager = JSON.stringify({ Mnemonic: mnemonic });

        // Create message with tagged payload
        const message = await client.generateMessage(secretManager, options);
        console.log('Message:', message, '\n');

        // Send message
        const messageId = await client.postMessage(message);

        console.log(
            `Message sent: https://explorer.iota.org/devnet/message/${messageId}\n`,
        );

        const fetchedMessage = await client.getMessageData(messageId);
        console.log('Message data: ', fetchedMessage);

        const payload = fetchedMessage.payload;
        if (payload && 'data' in payload && payload.data) {
            console.log('Decoded data:', hexToUtf8(payload.data));
        }
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
