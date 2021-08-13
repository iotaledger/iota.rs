async function run() {
    const { ClientBuilder } = require('test-iota-client-wasm');

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    const messageId = await client.message().submit();
    console.log(messageId);
}

run()
