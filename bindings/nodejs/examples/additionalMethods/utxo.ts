// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/additionalMethods/utxo.js

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
        const receipts = await client.getReceipts();
        console.log('Receipts:', receipts);

        const receiptsMigratedAt = await client.getReceiptsMigratedAt(154862);
        console.log('Receipts:', receiptsMigratedAt);

        const treasury = await client.getTreasury();
        console.log('Treasury:', treasury);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
