// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
import { writeFile, readFile } from 'fs/promises';
import 'dotenv/config';
import path = require('path');

// From examples directory, run with:
// node ./dist/offline_signing/2_transaction_signing.js

// In this example we will sign the prepared transaction
const PREPARED_TRANSACTION_FILE_NAME = path.join(
    __dirname,
    '../../offline_signing/prepared_transaction.json',
);
const SIGNED_TRANSACTION_FILE_NAME = path.join(
    __dirname,
    '../../offline_signing/signed_transaction.json',
);

async function run() {
    initLogger();

    // client will connect to testnet by default
    const offlineClient = new Client({
        offline: true,
        nodes: [
            {
                // Insert your node URL here.
                url: 'http://localhost:14265/',
                disabled: false,
            },
        ],
        localPow: true,
    });

    const signer = JSON.stringify({
        Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
    });
    try {
        // Read in prepared transaction from example 2_transaction_preparation
        const preparedTransaction = JSON.parse(
            await readFile(PREPARED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        // Signs prepared transaction offline.
        const signedTransaction = await offlineClient.signTransaction(
            signer,
            preparedTransaction,
        );

        console.log('Signed transaction.');

        await writeFile(
            SIGNED_TRANSACTION_FILE_NAME,
            JSON.stringify(signedTransaction),
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
