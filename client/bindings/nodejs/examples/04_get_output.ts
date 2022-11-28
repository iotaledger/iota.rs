// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/04_get_output.js

// In this example we will get output from a known outputId
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
        localPow: true,
    });

    try {
        const output = await client.getOutput(
            '0xa0b9ad3f5aa2bfcaed30cde6e1d572e93b7e8bb5a417f5a7ef3502889b5dbcb40000',
        );
        console.log('Output: ', output);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
