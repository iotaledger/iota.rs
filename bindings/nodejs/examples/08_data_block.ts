// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, utf8ToBytes, hexToUtf8, initLogger } from '@iota/client';
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
        tag: utf8ToBytes('Hello'),
        data: utf8ToBytes('Tangle'),
    };
    try {
        const mnemonic = await client.generateMnemonic();
        const secretManager = { Mnemonic: mnemonic };

        // Create block with tagged payload
        const block = await client.generateBlock(secretManager, options);
        console.log('Block:', block, '\n');

        // Send block
        const blockId = await client.postBlock(block);

        console.log(
            `Block sent: ${process.env.EXPLORER_URL}/block/${blockId}\n`,
        );

        const fetchedBlock = await client.getBlock(blockId);
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
