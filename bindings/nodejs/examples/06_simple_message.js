async function run() {
    const { ClientBuilder } = require('@iota/client');

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    const messageId = await client.postMessage();
    console.log(messageId);
}

run()