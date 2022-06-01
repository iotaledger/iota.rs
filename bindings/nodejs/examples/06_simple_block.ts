// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/06_simple_block.js

// In this example we will send a block without a payload
async function run() {
    initLogger();

    // client will connect to testnet by default
    const client = new Client({
        nodes: [
            {
                // Insert your node URL here.
                url: 'http://localhost:14265',
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

        console.log(
            `Empty block sent: https://explorer.iota.org/devnet/block/${blockId}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
