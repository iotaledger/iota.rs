// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/additional_methods/node_info.js

// In this example we will get information about the node
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
        const nodeInfo = await client.getNode();
        console.log('Node info: ', nodeInfo);

        const getNodeHealth = await client.getNodeHealth(nodeInfo.url);
        console.log('NodeHealthByUrl: ', getNodeHealth);

        const getHealth = await client.getHealth();
        console.log('NodeHealth: ', getHealth);

        const getNodeInfo = await client.getNodeInfo(nodeInfo.url);
        console.log('NodeInfo by URL: ', getNodeInfo);

        const unsyncedNodes = await client.unsyncedNodes();
        console.log('Unsynced nodes: ', unsyncedNodes);

        const tips = await client.getTips();
        console.log('Tips: ', tips);

        // TODO: fix error: missing or malformed JWT
        const getPeers = await client.getPeers();
        console.log('Peers: ', getPeers);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
