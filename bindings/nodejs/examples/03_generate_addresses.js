async function run() {
    // client will connect to testnet by default
    const { Client } = require('@iota/client');

    // client will connect to testnet by default
    const client = new Client({
        nodes: [
            {
                url: 'http://localhost:14265/',
                auth: null,
                disabled: false,
            },
        ],
        localPow: true,
    });

    require('dotenv').config();
    const signer = JSON.stringify({
        Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
    });
    const SHIMMER_TESTNET_BECH32_HRP = 'rms';

    const defaultOptions = {};
    const customOptions = {
        accountIndex: 0,
        range: {
            start: 0,
            end: 4,
        },
    };
    const offlineGeneratedOptions = {
        accountIndex: 0,
        range: {
            start: 0,
            end: 4,
        },
        bech32Hrp: SHIMMER_TESTNET_BECH32_HRP,
    };

    try {
        // Generate addresses with default account index and range
        const defaultAddresses = await client.generateAddresses(
            signer,
            defaultOptions,
        );
        console.log(
            `List of generated public addresses: \n${defaultAddresses}\n`,
        );

        // Generate addresses with custom account index and range
        const customAddresses = await client.generateAddresses(
            signer,
            customOptions,
        );
        console.log(
            `List of generated public addresses: \n${customAddresses}\n`,
        );

        // TODO: How to implement this? Is a new client_method required?
        // Generate public (false) & internal (true) addresses
        // console.log(
        //     `List of generated public and internal addresses: \n${bech32Addresses}\n`,
        // );

        // Generate public addresses offline with the bech32_hrp defined
        const offlineGeneratedAddresses = await client.generateAddresses(
            signer,
            offlineGeneratedOptions,
        );
        console.log(
            `List of offline generated public addresses: \n${offlineGeneratedAddresses}\n`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }

    process.exit();
}

run();
