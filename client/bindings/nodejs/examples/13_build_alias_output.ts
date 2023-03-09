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
        const hexAddress = await client.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const aliasOutput = await client.buildAliasOutput({
            aliasId:
                '0x0000000000000000000000000000000000000000000000000000000000000000',
            // `hello` hex encoded
            stateMetadata: '0x68656c6c6f',
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
            features: [
                {
                    // sender feature
                    type: 0,
                    address: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                },
                {
                    // MetadataFeature
                    type: 2,
                    // `hello` hex encoded
                    data: '0x68656c6c6f',
                },
            ],
            immutableFeatures: [
                {
                    // issuer feature
                    type: 1,
                    address: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                },
                {
                    // MetadataFeature
                    type: 2,
                    // `hello` hex encoded
                    data: '0x68656c6c6f',
                },
            ],
        });

        console.log(JSON.stringify(aliasOutput, null, 2));
    } catch (error) {
        console.error('Error: ', error);
    }
}

run();
