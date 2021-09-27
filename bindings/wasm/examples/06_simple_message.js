async function run() {
    const { ClientBuilder } = require('../node')

    // client will connect to testnet by default
    const client = await new ClientBuilder().build();

    const message = await client.message().submit();
    console.log(message);
}

run()
