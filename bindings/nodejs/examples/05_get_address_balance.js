// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will get the outputs of an address that has no additional unlock
// conditions and sum the amounts and native tokens
async function run() {
    const { Client, initLogger } = require('@iota/client');

    initLogger({
        colorEnabled: true,
        name: './client.log',
        levelFilter: 'debug',
    });

    // client will connect to testnet by default
    const client = new Client({
        nodes: [
            {
                url: 'http://localhost:14265',
                auth: null,
                disabled: false,
            },
        ],
        localPow: true,
    });

    require('dotenv').config();
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
        let totalNativeTokens = {};
        for (const outputResponse of addressOutputs) {
            const output = outputResponse['output'];

            output.nativeTokens.forEach(
                (token) =>
                    (totalNativeTokens[token.id] =
                        (totalNativeTokens[token.id] || 0) +
                        parseInt(token.amount)),
            );

            totalAmount += parseInt(output.amount);
        }

        console.log(
            `Outputs controlled by ${addresses[0]} have: ${totalAmount}i and native tokens: `,
            totalNativeTokens,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
