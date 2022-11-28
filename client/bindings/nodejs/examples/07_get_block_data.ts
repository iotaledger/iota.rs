// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/07_get_block_data.js

// In this example we will send a block and get the data and metadata for it
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
    });

    try {
        // Create block with no payload.
        const blockIdAndBlock = await client.buildAndPostBlock();
        console.log('Block:', blockIdAndBlock, '\n');

        // Get the metadata for the block.
        const blockMetadata = await client.getBlockMetadata(blockIdAndBlock[0]);
        console.log('Block metadata: ', blockMetadata, '\n');

        // Request the block by its id.
        const blockData = await client.getBlock(blockIdAndBlock[0]);
        console.log('Block data: ', blockData, '\n');
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
