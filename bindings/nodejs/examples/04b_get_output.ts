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
            },
        ],
        localPow: true,
    });

    try {
        const output = await client.getOutput(
            '0xc1d95ac9c8c0237c6929faf427556c3562055a7155c6d336ee7891691d5525c90100',
        );
        console.log('Output: ', output);
    } catch (error) {
        console.error('Error: ' + error);
    }
}

run().then(() => process.exit());
