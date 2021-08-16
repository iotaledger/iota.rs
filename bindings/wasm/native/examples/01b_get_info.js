
async function run() {
    const { ClientBuilder } = require('../node/iota_client_wasm')

    // client will connect to testnet by default
    const client = await new ClientBuilder()
        .node('https://api.lb-0.testnet.chrysalis2.com:443')    // custom node
        .localPow(true)                                         // pow is done locally
        .build();

    client.getInfo().then(console.log).catch(console.error);
}

run()
