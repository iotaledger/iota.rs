async function run() {
    const { ClientBuilder } = require('@iota/client');

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    const output = await client.getOutput('a22cba0667c922cbb1f8bdcaf970b2a881ccd6e88e2fcce50374de2aac7c37720000');
    console.log(output);
}

run()
