// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
import { readFile } from 'fs/promises';

require('dotenv').config({ path: '../../../.env' });

// From examples directory, run with:
// node ./dist/offline_signing/3_send_block.js

const SIGNED_TRANSACTION_FILE_NAME =
    __dirname + '/../../offline_signing/signed_transaction.json';

// In this example we will send the signed transaction in a block
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    const onlineClient = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
        localPow: true,
    });

    try {
        const signedTransaction = JSON.parse(
            await readFile(SIGNED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        // Send block with the signed transaction as a payload
        const blockIdAndBlock = await onlineClient.postBlockPayload(
            signedTransaction,
        );

        console.log(
            `Transaction sent: ${process.env.EXPLORER_URL}/block/` +
                blockIdAndBlock[0],
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
