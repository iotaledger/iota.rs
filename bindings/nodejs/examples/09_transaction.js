async function run() {
    const {
        ClientBuilder
    } = require('@iota/client');

    // Get the seed from environment variable
    const IOTA_SEED_SECRET = process.env.IOTA_SEED_SECRET;

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    const message = await client.message()
        .seed(IOTA_SEED_SECRET)
        .output('atoi1qqydc70mpjdvl8l2wyseaseqwzhmedzzxrn4l9g2c8wdcsmhldz0ulwjxpz', 1000000)
        .submit();

    console.log(message);
}

run()
