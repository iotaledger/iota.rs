// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';
require('dotenv').config({ path: '../../../.env' });

// Run with command:
// node ./dist/additional_methods/utxo_indexer.js

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
        const aliasesOutputIds = await client.aliasesOutputIds([
            {
                stateController:
                    'rms1qq20p8cqdzkq2q42gy3ajm27m2da299x0xw9jpuv3njc6a4j9m7aujfg7pv',
            },
        ]);
        console.log('Aliases output IDs:', aliasesOutputIds);

        const nftsOutputIds = await client.nftsOutputIds([
            {
                address:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
        ]);
        console.log('Nfts output IDs:', nftsOutputIds);

        const foundriesOutputIds = await client.foundriesOutputIds([
            {
                address:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
        ]);
        console.log('Foundries output IDs:', foundriesOutputIds);

        // TODO: get valid alias ID to test with
        const aliasOutputId = await client.aliasOutputId(
            '0xb098c0ee963575a8d505c299a99ce9c32cd4ecb3',
        );
        console.log('Alias output ID:', aliasOutputId);

        // TODO: get valid nft ID to test with
        const nftOutputId = await client.nftOutputId(
            '0x4d09c5ef6a61766beb0b5655fdf0b379bac8a813',
        );
        console.log('Nft output ID:', nftOutputId);

        // TODO: get valid foundry ID to test with
        const foundryOutputId = await client.foundryOutputId(
            '0xcabce38123184841124d9489eaebc5c3a084a133bbbc45d89b30',
        );
        console.log('Foundry output ID:', foundryOutputId);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
