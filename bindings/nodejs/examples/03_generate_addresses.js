async function run() {
  const { ClientBuilder } = require('@iota/client');

  // Get the seed from environment variable
  const IOTA_SEED_SECRET = process.env.IOTA_SEED_SECRET;

  // client will connect to testnet by default
  const client = new ClientBuilder().build();

  // const addresses = await client.getAddresses(IOTA_SEED_SECRET)
  //   .accountIndex(0)
  //   .range(0, 2)
  //   .get();

  // console.log(addresses);
  const addresses2 = await client.getAddresses(IOTA_SEED_SECRET).range(0, 2).get();

  console.log(addresses2);
}

run()
