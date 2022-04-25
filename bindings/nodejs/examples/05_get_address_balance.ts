// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
import 'dotenv/config';

// Run with command:
// node ./dist/05_get_address_balance.js

// In this example we will get the outputs of an address that has no additional unlock
// conditions and sum the amounts and native tokens
async function run() {
    initLogger({
        colorEnabled: true,
        name: './client.log',
        levelFilter: 'debug',
    });

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

    const signer = JSON.stringify({
        Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
    });

    try {
        // Generate the first address
        const addresses = await client.generateAddresses(signer, {
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        });

        // Get output ids of outputs that can be controlled by this address without further unlock constraints
        const outputIds = await client.outputIds([
            { address: addresses[0] },
            { hasExpirationCondition: false },
            { hasTimelockCondition: false },
            { hasStorageDepositReturnCondition: false },
        ]);

        // Get outputs by their IDs
        const addressOutputs = await client.getOutputs(outputIds);

        // Calculate the total amount and native tokens
        let totalAmount = 0;
        let totalNativeTokens: { [id: string]: number } = {};
        for (const outputResponse of addressOutputs) {
            const output = outputResponse['output'];

            if ('nativeTokens' in output) {
                output.nativeTokens.forEach(
                    (token) =>
                        (totalNativeTokens[token.id] =
                            (totalNativeTokens[token.id] || 0) +
                            parseInt(token.amount)),
                );
            }

            totalAmount += parseInt(output.amount);
        }

        console.log(
            `Outputs controlled by ${addresses[0]} have: ${totalAmount}i and native tokens: `,
            totalNativeTokens,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
