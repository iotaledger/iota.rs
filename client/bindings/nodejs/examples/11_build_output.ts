// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
import type { UnlockConditionTypes } from '@iota/types';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/11_build_output.js

// Build a basic output
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        nodes: [process.env.NODE_URL],
    });

    try {
        const hexAddress = await client.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const addressUnlockCondition: UnlockConditionTypes = {
            type: 0,
            address: {
                type: 0,
                pubKeyHash: hexAddress,
            },
        };

        // Build most basic output with amound and a single address unlock condition
        const basicOutput = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [addressUnlockCondition],
        });

        console.log(JSON.stringify(basicOutput, null, 2));

        // Output with metadata feature block
        const basicOutputWithMetadata = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [addressUnlockCondition],
            features: [
                {
                    type: 2,
                    // "Hello, World!" hex encoded
                    data: '0x48656c6c6f2c20576f726c6421',
                },
            ],
        });

        console.log(JSON.stringify(basicOutputWithMetadata, null, 2));

        // Output with storage deposit return
        const basicOutputWithStorageReturn = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [
                addressUnlockCondition,
                {
                    type: 1,
                    returnAddress: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                    amount: '1000000',
                },
            ],
        });

        console.log(JSON.stringify(basicOutputWithStorageReturn, null, 2));

        // Output with expiration
        const basicOutputWithExpiration = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [
                addressUnlockCondition,
                {
                    type: 3,
                    returnAddress: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                    unixTime: 1,
                },
            ],
        });

        console.log(JSON.stringify(basicOutputWithExpiration, null, 2));

        // Output with timelock
        const basicOutputWithTimelock = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [
                addressUnlockCondition,
                {
                    type: 2,
                    unixTime: 1,
                },
            ],
        });

        console.log(JSON.stringify(basicOutputWithTimelock, null, 2));
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
