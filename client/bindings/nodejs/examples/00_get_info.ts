// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/00_get_info.js

// In this example we will get information about the node
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
        localPow: true,
    });

    try {
        const nodeInfo = await client.getInfo();
        console.log('Node info: ', nodeInfo);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());

// Example output:
// Node info:  {
//     nodeInfo: {
//       name: 'HORNET',
//       version: '2.0.0-alpha.25',
//       status: {
//         isHealthy: true,
//         latestMilestone: [Object],
//         confirmedMilestone: [Object],
//         pruningIndex: 0
//       },
//       supportedProtocolVersions: [ 2 ],
//       protocol: {
//         version: 2,
//         networkName: 'dummy-1',
//         bech32Hrp: 'rms',
//         minPowScore: 1500,
//         rentStructure: [Object],
//         tokenSupply: '1450896407249092'
//       },
//       pendingProtocolParameters: [],
//       baseToken: {
//         name: 'Shimmer',
//         tickerSymbol: 'SMR',
//         unit: 'SMR',
//         subunit: 'glow',
//         decimals: 6,
//         useMetricPrefix: false
//       },
//       metrics: {
//         blocksPerSecond: 1.2,
//         referencedBlocksPerSecond: 1.2,
//         referencedRate: 100
//       },
//       features: []
//     },
//     url: 'https://api.testnet.shimmer.network'
// }
