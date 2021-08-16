async function run() {
    const { ClientBuilder } = require('../node/iota_client_wasm')

    // client will connect to testnet by default
    const client = await new ClientBuilder().build();

    const output = await client.getOutput('fb95fc3c2d587e12a91df7e3e9e7a63648e621c5946ce5db750a4421cfd5fbff0100');
    console.log(output);
}

run()
