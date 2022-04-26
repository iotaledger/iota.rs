// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will send the signed transaction in a message

const SIGNED_TRANSACTION_FILE_NAME = './signed_transaction.json';

async function run() {
    const { Client, initLogger } = require('@iota/client');
    const { readFile } = require('fs/promises');

    initLogger();

    // client will connect to testnet by default
    const onlineClient = new Client({
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
        const signedTransaction = JSON.parse(
            await readFile(SIGNED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        let message = await onlineClient.submitPayload(signedTransaction);

        // TODO: get messageId from message (Blake2b256 hash of message)
        console.log(
            'Transaction sent: https://explorer.iota.org/devnet/message/' +
                message.messageId,
        );
        console.log(message);
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
