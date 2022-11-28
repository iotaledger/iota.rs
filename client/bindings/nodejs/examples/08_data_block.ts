// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, hexToUtf8, initLogger, utf8ToHex } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/08_data_block.js

// In this example we will send a block with a tagged data payload
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
    });

    const options = {
        tag: utf8ToHex('Hello'),
        data: utf8ToHex('Tangle'),
    };
    try {
        const mnemonic = await client.generateMnemonic();
        const secretManager = { mnemonic: mnemonic };

        // Create block with tagged payload
        const blockIdAndBlock = await client.buildAndPostBlock(
            secretManager,
            options,
        );
        console.log('Block:', blockIdAndBlock, '\n');

        console.log(
            `Block sent: ${process.env.EXPLORER_URL}/block/${blockIdAndBlock[0]}\n`,
        );

        const fetchedBlock = await client.getBlock(blockIdAndBlock[0]);
        console.log('Block data: ', fetchedBlock);

        const payload = fetchedBlock.payload;
        if (payload && 'data' in payload && payload.data) {
            console.log('Decoded data:', hexToUtf8(payload.data));
        }
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
