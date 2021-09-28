// In this example we will generate addresses which will be used later to find inputs
async function run() {
    const { ClientBuilder } = require('../../');
    const fs = require('fs')

    const iota_offline = new ClientBuilder()
        .offlineMode()
        .build();

    const ADDRESS_FILE_NAME = "examples/offline_signing/addresses.json";
    const seed = process.env.IOTA_SEED_SECRET;

    let addresses = await iota_offline
        .getAddresses(seed)
        .range(0, 10)
        .bech32Hrp("atoi")
        .get();

    console.log(addresses)

    fs.writeFile(ADDRESS_FILE_NAME, JSON.stringify(addresses), err => {
        if (err) {
            console.error(err)
        }
    })
}

run()
