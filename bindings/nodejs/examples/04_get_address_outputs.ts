// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/04_get_address_outputs.js

// In this example we will get the outputs of a known address
async function run() {
    initLogger();

    // client will connect to testnet by default
    const client = new Client({
        nodes: [
            {
                // Insert your node URL here.
                url: 'http://localhost:14265',
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
        console.error('Error: ' + error);
    }
}

run().then(() => process.exit());
