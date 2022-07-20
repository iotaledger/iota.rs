// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../.env' });

// Run with command:
// node ./dist/ledger_nano.js

// In this example we will consolidate all funds in a range of addresses
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

    const addressRange = {
        start: 0,
        end: 10,
    };

    try {
        const isSimulator = false;

        const secretManager = {
            Ledger: isSimulator,
        };

        const ledgerStatus = await client.getLedgerStatus(
            isSimulator,
        );
        console.log(ledgerStatus);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
