async function run() {
    const { ClientBuilder } = require('@iota/client');

    // Get the seed from environment variable
    const IOTA_SEED_SECRET = process.env.IOTA_SEED_SECRET;

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    // Get the balance of a single known address
    console.log(
        await client.getAddressBalance("atoi1qp9427varyc05py79ajku89xarfgkj74tpel5egr9y7xu3wpfc4lkpx0l86")
    );

    // Get the balance of addresses from an account
    const balance = await client.getBalance(IOTA_SEED_SECRET)
        .accountIndex(0)
        .initialAddressIndex(0)
        .get();

    console.log("Account balance: " + balance);
}

run()
