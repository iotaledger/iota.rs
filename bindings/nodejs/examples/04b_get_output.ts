// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/04b_get_output.js

// In this example we will get output from a known outputId
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
        const output = await client.getOutput(
            '0xee8255ece109f4d460fa85d34f2a5f152014633db571220c84d6ebb944f129c00000',
        );
        console.log('Output: ', output);
    } catch (error) {
        console.error('Error: ' + error);
    }
}

run().then(() => process.exit());
