// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, CoinType, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/02_mnemonic.js

// In this example we will generate a mnemonic and generate the first address with the Shimmer coin type,
// following BIP-0044
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
        const mnemonic = await client.generateMnemonic();

        const secretManager = { Mnemonic: mnemonic };

        // Generate addresses with custom account index and range
        const addresses = await client.generateAddresses(secretManager, {
            coinType: CoinType.Shimmer,
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        });

        console.log('First public address: ', addresses[0]);
        // Example output:
        // First public address:  rms1qpvmkrdne3camug49emj84vsxvthjk9cwgyvmykg2a6umcjl749p5mjwckd
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
