// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
// Run with command:
// node ./dist/additional_methods/milestone.js

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
        const info = await client.getInfo();
        // @ts-ignore: INodeInfo type is incorrect?
        const milestoneIndex = info.nodeinfo.status.confirmedMilestone.index;
        console.log(milestoneIndex);

        // Look up a milestone by a given milestone index.
        const milestone = await client.getMilestoneByMilestoneIndex(
            milestoneIndex,
        );
        console.log('Milestone:', milestone);

        // Get all UTXO changes of a given milestone by milestone index.
        const milestoneUtxoChanges =
            await client.getUtxoChangesByMilestoneIndex(milestoneIndex);
        console.log('MilestoneUtxoChanges:', milestoneUtxoChanges);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
