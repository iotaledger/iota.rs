// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/consolidate.js

// In this example we will consolidate all funds in a range of addresses
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

    const addressRange = {
        start: 0,
        end: 10,
    };

    try {
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }
        const secretManager = {
            Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        };

        // Here all funds will be sent to the address with the lowest index in the range
        const address = await client.consolidateFunds(
            secretManager,
            0,
            addressRange,
        );
        console.log('Funds consolidated to: ', address);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
