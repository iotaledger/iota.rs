// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/additional_methods/network_info.js

// In this example we will get information about the network
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
        const networkInfo = await client.getNetworkInfo();
        console.log('Network info: ', networkInfo);

        const networkId = await client.getNetworkId();
        console.log('NetworkId: ', networkId);

        const getBech32Hrp = await client.getBech32Hrp();
        console.log('Bech32Hrp: ', getBech32Hrp);

        const getMinPowScore = await client.getMinPowScore();
        console.log('MinPowScore: ', getMinPowScore);

        const getTipsInterval = await client.getTipsInterval();
        console.log('TipsInterval: ', getTipsInterval);

        const getLocalPow = await client.getLocalPow();
        console.log('LocalPow: ', getLocalPow);

        const getFallbackToLocalPow = await client.getFallbackToLocalPow();
        console.log('FallbackToLocalPow: ', getFallbackToLocalPow);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
