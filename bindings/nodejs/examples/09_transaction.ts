// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/09_transaction.js

// In this example we will send a transaction
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
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }

        // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
        // balance
        const secretManager = {
            Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        };

        // We generate an address from our own mnemonic so that we send the funds to ourselves
        const addresses = await client.generateAddresses(secretManager, {
            range: {
                start: 1,
                end: 2,
            },
        });

        // We prepare the transaction
        // Insert the output address and amount to spend. The amount cannot be zero.
        const block = await client.generateBlock(secretManager, {
            output: {
                address: addresses[0],
                amount: '1000000',
            },
        });
        console.log('Block: ', block, '\n');

        // Send transaction
        const blockId = await client.postBlock(block);

        console.log(
            `Transaction sent: https://explorer.iota.org/devnet/block/${blockId}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
