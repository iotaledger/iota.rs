async function run(Iota) {
  console.log(Iota)

  const {
    Client
  } = Iota

  let iota_client = await Client.withNode("https://api.lb-0.testnet.chrysalis2.com/");

  console.log("Nodeinfo: ", await iota_client.getInfo())

}

import("../pkg/index.js").then(async iota => {
  try {
    await run(iota)
  } catch (e) {
    console.error(e)
  }
})
