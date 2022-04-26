// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will sign the prepared transaction

const PREPARED_TRANSACTION_FILE_NAME = './prepared_transaction.json';
const SIGNED_TRANSACTION_FILE_NAME = './signed_transaction.json';

async function run() {
    const { Client, initLogger } = require('@iota/client');
    const { writeFile, readFile } = require('fs/promises');
    require('dotenv').config({ path: '../.env' });

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

    try {
        // Read in prepared transaction from example 2_transaction_preparation
        const preparedTransaction = JSON.parse(
            await readFile(PREPARED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        const signer = JSON.stringify({
            Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        });

        // Signs prepared transaction offline.
        const signedTransaction = await offlineClient.signTransaction(
            signer,
            preparedTransaction,
        );

        console.log('Signed transaction.');

        await writeFile(
            SIGNED_TRANSACTION_FILE_NAME,
            JSON.stringify(signedTransaction),
            (err) => {
                if (err) {
                    console.error(err);
                }
            },
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
