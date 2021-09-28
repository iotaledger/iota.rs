async function run() {
    const { ClientBuilder } = require('../node')

    // client will connect to testnet by default
    const client = await new ClientBuilder().build();

    const output = await client.getOutput('3e18e19045d0b44dd2be3c466d6fe419c09342bacdb587f2985f2e607a92e38e0100');
    console.log(output);
}

run()
