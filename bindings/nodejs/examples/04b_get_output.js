// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will get output from a known outputId
async function run() {
    const { Client, initLogger } = require('@iota/client');

    initLogger({
        color_enabled: true,
        name: './client.log',
        level_filter: 'debug',
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
        const output = await client.getOutput(
            '0xd6d082e0f8a5e8c1ce109b8c45abf70bde4d23429ef9b90b40648c6d5408aa100100',
        );
        console.log('Output: ', output);
    } catch (error) {
        console.log('Error: ' + error);
    }
}

run().then(() => process.exit());
