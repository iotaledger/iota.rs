// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will get inputs and prepare a transaction
async function run() {
    const { Client, initLogger } = require('@iota/client');
    const { writeFile, readFile } = require('fs/promises');
    require('dotenv').config({ path: '../.env' });

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
                auth: null,
                disabled: false,
            },
        ],
        localPow: true,
    });

    const signer = JSON.stringify({
        Mnemonic: 'None',
    });

    let address =
        'rms1qqv5avetndkxzgr3jtrswdtz5ze6mag20s0jdqvzk4fwezve8q9vkpnqlqe';
    let amount = 1_000_000;
    const ADDRESS_FILE_NAME = './addresses.json';
    const PREPARED_TRANSACTION_FILE_NAME = './prepared_transaction.json';

    try {
        const addresses = JSON.parse(await readFile(ADDRESS_FILE_NAME, 'utf8'));

        const inputs = await onlineClient.findInputs(addresses, amount);
        console.log(inputs);

        // TODO: This is incorrect, is it because a message can't be built
        // without a signer via the message handler? The incorrect output is
        // in examples/offline_signing/test_example_prepared_transaction.json
        // We prepare the transaction.
        const transaction = await onlineClient.generateMessage(signer, {
            inputs,
            output: { address, amount },
        });

        console.log(`Prepared transaction sending ${amount} to ${address}.`);

        await writeFile(
            PREPARED_TRANSACTION_FILE_NAME,
            JSON.stringify(transaction),
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
