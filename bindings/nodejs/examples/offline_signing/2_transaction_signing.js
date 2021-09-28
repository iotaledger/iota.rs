// In this example we will sign the prepared transaction
async function run() {
    const { ClientBuilder } = require('../../');
    const fs = require('fs')

    const iota_offline = new ClientBuilder()
        .offlineMode()
        .build();

    const PREPARED_TRANSACTION_FILE_NAME = "examples/offline_signing/prepared_transaction.json";
    const SIGNED_TRANSACTION_FILE_NAME = "examples/offline_signing/signed_transaction.json";
    const seed = process.env.IOTA_SEED_SECRET;

    const prepared_transaction_data = JSON.parse(fs.readFileSync(PREPARED_TRANSACTION_FILE_NAME, 'utf8'))

    let signed_transaction = await iota_offline
        .message().signTransaction(prepared_transaction_data, seed);

    console.log('Signed transaction')

    fs.writeFile(SIGNED_TRANSACTION_FILE_NAME, JSON.stringify(signed_transaction), err => {
        if (err) {
            console.error(err)
        }
    })
}

run()