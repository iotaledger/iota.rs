// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/additional_methods/message.js

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

        const jsonMessageId = await client.postMessageJson(message);
        console.log('JsonMessage ID: ', jsonMessageId);

        const getMessageChildren = await client.getMessageChildren(
            jsonMessageId,
        );
        console.log('Message children: ', getMessageChildren);

        const findMessages = await client.findMessages([jsonMessageId]);
        console.log('Messages found: ', findMessages);

        // TODO: Error: 404 message not found. However, if calling getMessageData/metadata
        // on the same ID, the message is found
        const getMessageRaw = await client.getMessageRaw(jsonMessageId);
        console.log('Raw message: ', getMessageRaw);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
