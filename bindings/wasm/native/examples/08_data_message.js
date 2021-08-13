async function run() {
    const { ClientBuilder } = require('test-iota-client-wasm');

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    const message = await client.message()
        .index('IOTA.RS BINDING - NODE.JS')
        .data('some utf based data')
        .submit();

    console.log(message);
}

run()
