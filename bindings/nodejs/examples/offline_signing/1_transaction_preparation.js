// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will get inputs and prepare a transaction

const ADDRESS_FILE_NAME = './addresses.json';
const PREPARED_TRANSACTION_FILE_NAME = './prepared_transaction.json';

async function run() {
    const { Client, initLogger } = require('@iota/client');
    const { writeFile, readFile } = require('fs/promises');

    initLogger({
        colorEnabled: true,
        name: './client.log',
        levelFilter: 'debug',
    });

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

    let address =
        'rms1qqv5avetndkxzgr3jtrswdtz5ze6mag20s0jdqvzk4fwezve8q9vkpnqlqe';
    let amount = 1_000_000;
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
                output: { address, amount },
                allowBurning: false,
            },
        );

        console.log(`Prepared transaction sending ${amount} to ${address}.`);

        await writeFile(
            PREPARED_TRANSACTION_FILE_NAME,
            JSON.stringify(preparedTransaction),
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
