// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will generate a mnemonic and generate the first address
async function run() {
    const { Client, initLogger } = require('@iota/client');

    initLogger({
        colorEnabled: true,
        name: './client.log',
        levelFilter: 'debug',
    });

    // client will connect to testnet by default
    const client = new Client({
        nodes: [
            {
                url: 'http://localhost:14265',
                auth: null,
                disabled: false,
            },
        ],
        localPow: true,
    });

    try {
        const mnemonic = await client.generateMnemonic();
        const signer = JSON.stringify({ Mnemonic: mnemonic });

        // Generate addresses with custom account index and range
        const options = {
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        };
        const addresses = await client.generateAddresses(signer, options);

        console.log('First public address: ', addresses[0]);
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
