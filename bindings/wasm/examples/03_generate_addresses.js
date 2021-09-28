async function run() {
  const { ClientBuilder } = require('../node')

  // Get the seed from environment variable
  const IOTA_SEED_SECRET = process.env.IOTA_SEED_SECRET;

  // client will connect to testnet by default
  const client = await new ClientBuilder().build();

  const addresses = await client.getAddresses(IOTA_SEED_SECRET)
    .accountIndex(0)
    .range(0, 5)
    .get();

  console.log(addresses);
}

run()
