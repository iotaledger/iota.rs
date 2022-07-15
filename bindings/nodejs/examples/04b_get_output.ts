// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/04b_get_output.js

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
            '0xc1d95ac9c8c0237c6929faf427556c3562055a7155c6d336ee7891691d5525c90100',
        );
        console.log('Output: ', output);
    } catch (error) {
        console.error('Eraror: ', error);
    }
}

run().then(() => process.exit());
