// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/07_get_block_data.js

// In this example we will send a block and get the data and metadata for it
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

    try {
        // Create block with no payload
        const block = await client.generateBlock();
        console.log('Block:', block, '\n');

        // Send block
        const blockId = await client.postBlock(block);

        const blockData = await client.getBlock(blockId);
        const blockMetadata = await client.getBlockMetadata(blockId);

        console.log('Block data: ', blockData, '\n');
        console.log('Block metadata: ', blockMetadata, '\n');
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
