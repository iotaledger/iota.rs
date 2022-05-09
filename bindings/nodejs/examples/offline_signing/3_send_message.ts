// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
import { readFile } from 'fs/promises';

// From examples directory, run with:
// node ./dist/offline_signing/3_send_message.js

// In this example we will send the signed transaction in a message
const SIGNED_TRANSACTION_FILE_NAME =
    __dirname + '/../../offline_signing/signed_transaction.json';

async function run() {
    initLogger();

    // client will connect to testnet by default
    const onlineClient = new Client({
        nodes: [
            {
                // Insert your node URL here.
                url: 'http://localhost:14265/',
                disabled: false,
            },
        ],
        localPow: true,
    });

    try {
        const signedTransaction = JSON.parse(
            await readFile(SIGNED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        // Send message with the signed transaction as a payload
        const message = await onlineClient.submitPayload(signedTransaction);

        // Get the message ID from the message (Blake2b256 hash of the message bytes)
        const messageId = await onlineClient.messageId(message);

        console.log(
            'Transaction sent: https://explorer.iota.org/devnet/message/' +
                messageId,
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
