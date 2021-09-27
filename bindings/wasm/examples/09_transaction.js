async function run() {
    const { ClientBuilder } = require('../node')

    // Get the seed from environment variable
    const IOTA_SEED_SECRET = process.env.IOTA_SEED_SECRET;

    // client will connect to testnet by default
    const client = await new ClientBuilder().build();

    const message = await client.message()
        .seed(IOTA_SEED_SECRET)
        .output('atoi1qqydc70mpjdvl8l2wyseaseqwzhmedzzxrn4l9g2c8wdcsmhldz0ulwjxpz', BigInt(1000000))
        .submit();

    console.log(message);
}

run()
