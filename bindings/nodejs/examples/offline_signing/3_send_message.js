// In this example we will send the signed transaction in a message
async function run() {
    const { ClientBuilder } = require('../../');
    const fs = require('fs')

    const iota_online = new ClientBuilder()
        .node("https://api.lb-0.testnet.chrysalis2.com")
        .build();


    const SIGNED_TRANSACTION_FILE_NAME = "examples/offline_signing/signed_transaction.json";

    const signed_transaction = JSON.parse(fs.readFileSync(SIGNED_TRANSACTION_FILE_NAME, 'utf8'))

    let message = await iota_online
        .message().finishMessage(signed_transaction);

    console.log('Transaction sent: https://explorer.iota.org/devnet/message/' + message.messageId)
}

run()