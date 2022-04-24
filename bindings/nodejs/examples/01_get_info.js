// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will get information about the node
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
        const nodeInfo = await client.getInfo();
        console.log('Node info: ', nodeInfo);
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
