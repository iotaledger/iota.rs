// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/13_build_alias_output.js

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
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }
        const secretManager = {
            mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        };

        const addresses = await client.generateAddresses(secretManager, {
            range: {
                start: 0,
                end: 1,
            },
        });

        const hexAddress = await client.bech32ToHex(addresses[0]);

        const aliasOutput = await client.buildAliasOutput({
            aliasId:
                '0x0000000000000000000000000000000000000000000000000000000000000000',
            amount: '1000000',
            unlockConditions: [
                {
                    // StateControllerAddressUnlockCondition
                    type: 4,
                    address: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                },
                {
                    // GovernorAddressUnlockCondition
                    type: 5,
                    address: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                },
            ],
        });

        console.log(aliasOutput);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run();
