// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, CoinType, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/consolidate.js

// In this example we will consolidate all funds in a range of addresses
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
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }

        // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
        // balance
        const secretManager = {
            mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        };

        // Here all funds will be sent to the address with the lowest index in the range
        const address = await client.consolidateFunds(secretManager, {
            coinType: CoinType.Shimmer,
            accountIndex: 0,
            range: {
                start: 0,
                end: 10,
            },
            internal: false,
        });
        console.log('Funds consolidated to: ', address);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
