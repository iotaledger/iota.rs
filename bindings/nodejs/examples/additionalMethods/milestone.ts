// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
// Run with command:
// node ./dist/additionalMethods/milestone.js

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
        nodeSyncEnabled: false,
        localPow: true,
    });

    try {
        const info = await client.getInfo();
        // @ts-ignore: INodeInfo type is incorrect?
        const milestoneIndex = info.nodeinfo.status.confirmedMilestone.index;
        console.log(milestoneIndex);

        // TODO:
        //  error: invalid milestone ID: 6405
        //  error: hex string without 0x prefix: code=400, message=invalid parameter
        //  endpoint called: https://nodeurl.net/api/v2/milestones/6405
        //  should be: https://nodeurl.net/api/v2/milestones/by-index/6405
        // Look up a milestone by a given milestone index.
        const milestone = await client.getMilestone(milestoneIndex);
        console.log('Milestone:', milestone);

        // TODO:
        //  Same errors as above
        //  endpoint called: https://nodeurl.net/api/v2/milestones/6405/utxo-changes
        //  should be: https://nodeurl.net/api/v2/milestones/by-index/6405/utxo-changes
        // Get all UTXO changes of a given milestone by milestone index.
        const milestoneUtxoChanges = await client.getMilestoneUtxoChanges(
            milestoneIndex,
        );
        console.log('MilestoneUtxoChanges:', milestoneUtxoChanges);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
