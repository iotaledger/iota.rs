async function run() {
    const { ClientBuilder } = require('../node/iota_client_wasm')

    // client will connect to testnet by default
    const client = await new ClientBuilder().build();

    const messageId = await client.message().submit();
    console.log(messageId);
}

run()
