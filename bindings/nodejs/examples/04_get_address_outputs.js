// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will get the outputs of a known address
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
        const outputIds = await client.outputIds([
            {
                address:
                    'rms1qqv5avetndkxzgr3jtrswdtz5ze6mag20s0jdqvzk4fwezve8q9vkpnqlqe',
            },
        ]);
        console.log('Output ids: ', outputIds, '\n');

        const addressOutputs = await client.getOutputs(outputIds);
        console.log('Address outputs: ', addressOutputs);
    } catch (error) {
        console.log('Error: ' + error);
    }
}

run().then(() => process.exit());
