// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    CoinType,
    initLogger,
    SHIMMER_TESTNET_BECH32_HRP,
} from '@iota/client';
import { writeFile } from 'fs/promises';

require('dotenv').config({ path: '../../../.env' });

// From examples directory, run with:
// node ./dist/offline_signing/0_address_generation.js

const ADDRESS_FILE_NAME = __dirname + '/../../offline_signing/address.json';

// In this example we will generate an address offline which will be used later to find inputs
async function run() {
    initLogger();
    const offlineClient = new Client({});

    try {
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }

        const secretManager = {
            mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        };

        // Generates an address offline.
        const offlineGeneratedAddress = await offlineClient.generateAddresses(
            secretManager,
            {
                coinType: CoinType.Shimmer,
                range: {
                    start: 0,
                    end: 10,
                },
                bech32Hrp: SHIMMER_TESTNET_BECH32_HRP,
            },
        );

        await writeFile(
            ADDRESS_FILE_NAME,
            JSON.stringify(offlineGeneratedAddress),
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
