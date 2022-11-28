// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/05_get_address_balance.js

// In this example we will get the outputs of an address that has no additional unlock
// conditions and sum the amounts and native tokens
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
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }
        const secretManager = {
            mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        };

        // Generate the first address
        const addresses = await client.generateAddresses(secretManager, {
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        });

        // Get output ids of basic outputs that can be controlled by this address without further unlock constraints
        const outputIds = await client.basicOutputIds([
            { address: addresses[0] },
            { hasExpiration: false },
            { hasTimelock: false },
            { hasStorageDepositReturn: false },
        ]);

        // Get outputs by their IDs
        const addressOutputs = await client.getOutputs(outputIds);

        // Calculate the total amount and native tokens
        let totalAmount = 0;
        const totalNativeTokens: { [id: string]: number } = {};
        for (const outputResponse of addressOutputs) {
            const output = outputResponse['output'];

            if ('nativeTokens' in output) {
                output.nativeTokens?.forEach(
                    (token) =>
                        (totalNativeTokens[token.id] =
                            (totalNativeTokens[token.id] || 0) +
                            parseInt(token.amount)),
                );
            }

            totalAmount += parseInt(output.amount);
        }

        console.log(
            `Outputs controlled by ${addresses[0]} have: ${totalAmount} glow and native tokens: `,
            totalNativeTokens,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
