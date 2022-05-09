// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
import { writeFile, readFile } from 'fs/promises';

// From examples directory, run with:
// node ./dist/offline_signing/1_transaction_preparation.js

// In this example we will get inputs and prepare a transaction
const ADDRESS_FILE_NAME = __dirname + '/../../offline_signing/addresses.json';
const PREPARED_TRANSACTION_FILE_NAME =
    __dirname + '/../../offline_signing/prepared_transaction.json';

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

    const address =
        'rms1qqv5avetndkxzgr3jtrswdtz5ze6mag20s0jdqvzk4fwezve8q9vkpnqlqe';
    const amount = 1000000;
    try {
        // Recovers addresses from example `0_address_generation`.
        const addresses = JSON.parse(await readFile(ADDRESS_FILE_NAME, 'utf8'));

        // Gets enough inputs related to these addresses to cover the amount.
        const inputs = await onlineClient.findInputs(addresses, amount);

        // Prepares the transaction
        const preparedTransaction = await onlineClient.prepareTransaction(
            undefined,
            {
                inputs,
                output: { address, amount: amount.toString() },
            },
        );

        console.log(`Prepared transaction sending ${amount} to ${address}.`);

        await writeFile(
            PREPARED_TRANSACTION_FILE_NAME,
            JSON.stringify(preparedTransaction),
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
