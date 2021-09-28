
async function run() {
    const { ClientBuilder } = require('../node')

    // client will connect to testnet by default
    const client = await new ClientBuilder()
        .node('https://api.lb-0.h.chrysalis-devnet.iota.cafe:443')    // custom node
        .localPow(true)                                         // pow is done locally
        .build();

    client.getInfo().then(console.log).catch(console.error);
}

run()
