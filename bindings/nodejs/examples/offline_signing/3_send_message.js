// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// In this example we will send the signed transaction in a message

const SIGNED_TRANSACTION_FILE_NAME = './signed_transaction.json';

async function run() {
    const { Client, initLogger } = require('@iota/client');
    const { readFile } = require('fs/promises');

    initLogger({
        colorEnabled: true,
        name: './client.log',
        levelFilter: 'debug',
    });

    // client will connect to testnet by default
    const onlineClient = new Client({
        nodes: [
            {
                // Insert your node URL here.
                url: 'http://localhost:14265/',
                disabled: false,
            },
        ],
        localPow: true,
    });

    try {
        const signedTransaction = JSON.parse(
            await readFile(SIGNED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        // TODO: fix error from submitPayload method:
        // {"type":"Error","payload":{"type":"NodeError","error":"Response error with status code 400:
        // {\"error\":{\"code\":\"400\",\"message\":\"invalid parameter, error: invalid payload,
        // error: wrong networkID: 10859088168901003000: code=400, message=invalid parameter\"}}\n,
        // URL: REDACTED"},"action":"CallClientMethod"}
        let message = await onlineClient.submitPayload(signedTransaction);

        console.log(
            'Transaction sent: https://explorer.iota.org/devnet/message/' +
                message.messageId,
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
