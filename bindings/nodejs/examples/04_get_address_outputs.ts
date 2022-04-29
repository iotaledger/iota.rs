// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/04_get_address_outputs.js

// In this example we will get the outputs of a known address
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
        // Get output ids of outputs that can be controlled by this address without further unlock constraints
        const outputIds = await client.outputIds([
            {
                address:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
            { hasExpirationCondition: false },
            { hasTimelockCondition: false },
            { hasStorageDepositReturnCondition: false },
        ]);
        console.log('Output ids: ', outputIds, '\n');

        const addressOutputs = await client.getOutputs(outputIds);
        console.log('Address outputs: ', addressOutputs);
    } catch (error) {
        console.error('Error: ' + error);
    }
}

run().then(() => process.exit());
