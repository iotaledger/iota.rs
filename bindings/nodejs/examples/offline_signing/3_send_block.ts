// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
import { readFile } from 'fs/promises';

// From examples directory, run with:
// node ./dist/offline_signing/3_send_block.js

// In this example we will send the signed transaction in a block
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

        // Send block with the signed transaction as a payload
        const block = await onlineClient.submitPayload(signedTransaction);

        // Get the block ID from the block (Blake2b256 hash of the block bytes)
        const blockId = await onlineClient.blockId(block);

        console.log(
            'Transaction sent: https://explorer.iota.org/devnet/block/' +
                blockId,
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
