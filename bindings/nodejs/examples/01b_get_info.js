
function run() {
    const { ClientBuilder } = require('@iota/client');

    // client will connect to testnet by default
    const client = new ClientBuilder()
        .node('https://api.lb-0.testnet.chrysalis2.com:443')    // custom node
        .localPow(true)                                         // pow is done locally
        .disableNodeSync()                                      // even non-synced node is fine - do not use in production
        .build();

    client.getInfo().then(console.log).catch(console.error);
}

run()
