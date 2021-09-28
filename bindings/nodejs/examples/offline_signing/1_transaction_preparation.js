// In this example we will get inputs and prepare a transaction
async function run() {
    const { ClientBuilder } = require('../../');
    const fs = require('fs')

    const iota_online = new ClientBuilder()
        .node("https://api.lb-0.testnet.chrysalis2.com")
        .build();

    let address = "atoi1qruzprxum2934lr3p77t96pzlecxv8pjzvtjrzdcgh2f5exa22n6gek0qdq";
    let amount = 1_000_000;
    const ADDRESS_FILE_NAME = "examples/offline_signing/addresses.json";
    const PREPARED_TRANSACTION_FILE_NAME = "examples/offline_signing/prepared_transaction.json";

    const addresses = JSON.parse(fs.readFileSync(ADDRESS_FILE_NAME, 'utf8'))

    let inputs = await iota_online
        .findInputs(addresses, amount);

    let transaction_builder = iota_online.message();
    for (input of inputs) {
        transaction_builder = transaction_builder.input(input);
    }
    let prepared_transaction_data = await transaction_builder.output(address, amount).prepareTransaction();
    console.log(`Prepared transaction sending ${amount} to ${address}`)

    fs.writeFile(PREPARED_TRANSACTION_FILE_NAME, JSON.stringify(prepared_transaction_data), err => {
        if (err) {
            console.error(err)
        }
    })
}

run()