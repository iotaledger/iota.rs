// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
import { writeFile, readFile } from 'fs/promises';

require('dotenv').config({ path: '../../../.env' });

// From examples directory, run with:
// node ./dist/offline_signing/2_transaction_signing.js

const PREPARED_TRANSACTION_FILE_NAME =
    __dirname + '/../../offline_signing/prepared_transaction.json';
const SIGNED_TRANSACTION_FILE_NAME =
    __dirname + '/../../offline_signing/signed_transaction.json';

// In this example we will sign the prepared transaction
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    const offlineClient = new Client({
        // TODO: remove when https://github.com/iotaledger/iota.rs/issues/1263 is implemented
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
    });

    try {
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }

        const secretManager = {
            mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        };

        // Read in prepared transaction from example 2_transaction_preparation
        const preparedTransaction = JSON.parse(
            await readFile(PREPARED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        // Signs prepared transaction offline.
        const signedTransaction = await offlineClient.signTransaction(
            secretManager,
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
