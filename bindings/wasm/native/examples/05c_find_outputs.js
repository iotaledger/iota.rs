async function run() {
    const { ClientBuilder } = require('test-iota-client-wasm');

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    const outputs = await client.findOutputs(outputIds = [], addresses = ["atoi1qp9427varyc05py79ajku89xarfgkj74tpel5egr9y7xu3wpfc4lkpx0l86"]);
    console.log(outputs);
}

run()
