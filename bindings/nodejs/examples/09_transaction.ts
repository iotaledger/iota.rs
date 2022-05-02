// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
import 'dotenv/config';

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
                disabled: false,
            },
        ],
        localPow: true,
    });

    try {
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }

        const secretManager = JSON.stringify({
            Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        });

        // We generate an address from our seed so that we send the funds to ourselves
        const addresses = await client.generateAddresses(secretManager, {
            range: {
                start: 1,
                end: 2,
            },
        });

        // We prepare the transaction
        // Insert the output address and amount to spend. The amount cannot be zero.
        // TODO: fix error: {"type":"MessageDtoError","error":"tokenId"}
        const message = await client.generateMessage(secretManager, {
            output: {
                address: addresses[0],
                amount: '1000000',
            },
        });
        console.log('Message: ', message, '\n');

        // Send transaction
        const messageId = await client.postMessage(message);

        console.log(
            `Transaction sent: https://explorer.iota.org/devnet/message/${messageId}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
