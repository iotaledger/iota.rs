// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/14_build_foundry_output.js

// Build a foundry output
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

        const aliasId =
            '0xff311f59790ccb85343a36fbac2f06d233734794404142b308c13f2c616935b5';

        const foundryOutput = await client.buildFoundryOutput({
            serialNumber: 0,
            tokenScheme: {
                type: 0,
                // 10 hex encoded
                mintedTokens: '0xa',
                meltedTokens: '0x0',
                maximumSupply: '0xa',
            },
            amount: '1000000',
            unlockConditions: [
                {
                    // ImmutableAliasAddressUnlockCondition
                    type: 6,
                    address: {
                        // AliasAddress
                        type: 8,
                        aliasId,
                    },
                },
            ],
        });

        console.log(foundryOutput);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run();
