async function run(Iota) {
  console.log(Iota)

  const {
    ClientBuilder
  } = Iota

  let iota_client = new ClientBuilder().node("https://api.lb-0.testnet.chrysalis2.com").build();
  // Get the nodeinfo
  console.log(await iota_client.getInfo());
  // let message = await iota_client.message().index("test").submit()
  // console.log(message);
  // let message2 = await iota_client.getMessage().data(message.messageId)
  // console.log(message2);
  // let messageIds = await iota_client.getMessage().index(new TextEncoder().encode("test"))
  // console.log(messageIds);
  // console.log(await iota_client.retryUntilIncluded(message.messageId, BigInt(2), BigInt(2)));

  // Browser only
  // console.log(await iota_client.retryUntilIncluded(message.messageId, BigInt(2), BigInt(2)));
  // console.log(await iota_client.consolidateFunds('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2', 0, 0, 10));
}

import("../pkg/index.js").then(async iota => {
  try {
    await run(iota)
  } catch (e) {
    console.error(e)
  }
})
