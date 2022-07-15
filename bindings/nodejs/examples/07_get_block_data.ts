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
        const block = await client.generateBlock();
        console.log('Block:', block, '\n');

        // Hash the block to get the block id.
        const blockId = await client.blockId(block);

        // Get the metadata for the block.
        const blockMetadata = await client.getBlockMetadata(blockId);
        console.log('Block metadata: ', blockMetadata, '\n');

        // Request the block by it's id.
        const blockData = await client.getBlock(blockId);
        console.log('Block data: ', blockData, '\n');
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
