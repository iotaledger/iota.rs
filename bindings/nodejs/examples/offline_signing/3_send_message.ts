// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
import { readFile } from 'fs/promises';
import path = require('path');

// From examples directory, run with:
// node ./dist/offline_signing/3_send_message.js

// In this example we will send the signed transaction in a message
const SIGNED_TRANSACTION_FILE_NAME = path.join(
    __dirname,
    '../signed_transaction.json',
);

async function run() {
    initLogger();

    // client will connect to testnet by default
    const onlineClient = new Client({
        nodes: [
            {
                // Insert your node URL here.
                url: 'https://api.alphanet.iotaledger.net/',
                disabled: false,
            },
        ],
        localPow: true,
    });

    try {
        const signedTransaction = JSON.parse(
            await readFile(SIGNED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        const message = await onlineClient.submitPayload(signedTransaction);

        // TODO: get messageId from message (Blake2b256 hash of message)
        console.log(
            'Transaction sent: https://explorer.iota.org/devnet/message/' +
                message.messageId,
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
