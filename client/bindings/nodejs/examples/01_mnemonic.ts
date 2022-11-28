// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/01_mnemonic.js

// In this example we will generate a random BIP39 mnemonic
async function run() {
    initLogger();

    const client = new Client({});

    try {
        const mnemonic = await client.generateMnemonic();

        console.log('Mnemonic: ' + mnemonic);
        // Example output:
        // Mnemonic: endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
