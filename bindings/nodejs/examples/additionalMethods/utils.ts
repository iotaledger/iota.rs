// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Client, initLogger } from '@iota/client';

// Run with command:
// node ./dist/additionalMethods/utils.js

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
        const hex = await client.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );
        console.log('Bech32 to hex:', hex);

        const bech32 = await client.hexToBech32(hex);
        console.log('Hex to bech32:', bech32);

        const isAddressValid = await client.isAddressValid(bech32);
        console.log('Is address valid:', isAddressValid);

        // TODO: get public key to test with
        // const hexPublicKeyToBech32Address = await client.hexPublicKeyToBech32Address(hexPublicKey, "rms");
        // console.log('Public to address:', hexPublicKeyToBech32Address);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
