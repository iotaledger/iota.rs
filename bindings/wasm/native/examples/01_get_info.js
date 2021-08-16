const { ClientBuilder } = require('../node/iota_client_wasm')

async function run() {
    let client = await new ClientBuilder()
        .node("https://api.lb-0.testnet.chrysalis2.com")
        .build();
    // Get the nodeinfo
    console.log(await client.getInfo());
}
run()