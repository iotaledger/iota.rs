// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will generate addresses which will be used later to find inputs
async function run() {
    const {
        Client,
        initLogger,
        SHIMMER_TESTNET_BECH32_HRP,
    } = require('@iota/client');
    const { writeFile } = require('fs/promises');
    require('dotenv').config({ path: '../.env' });

    initLogger({
        colorEnabled: true,
        name: './client.log',
        levelFilter: 'debug',
    });

    // client will connect to testnet by default
    const offlineClient = new Client({
        offline: true,
        nodes: [
            {
                // Insert your node URL here.
                url: 'http://localhost:14265/',
                auth: null,
                disabled: false,
            },
        ],
        localPow: true,
    });

    const signer = JSON.stringify({
        Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
    });
    const ADDRESS_FILE_NAME = './addresses.json';

    const offlineGeneratedOptions = {
        range: {
            start: 0,
            end: 10,
        },
        bech32Hrp: SHIMMER_TESTNET_BECH32_HRP,
    };

    try {
        // Generates addresses offline.
        const offlineGeneratedAddresses = await offlineClient.generateAddresses(
            signer,
            offlineGeneratedOptions,
        );

        await writeFile(
            ADDRESS_FILE_NAME,
            JSON.stringify(offlineGeneratedAddresses),
            (err) => {
                if (err) {
                    console.error(err);
                }
            },
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
