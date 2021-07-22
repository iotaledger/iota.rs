async function run(Iota) {
  console.log(Iota)

  const {
    ClientBuilder
  } = Iota

  let iota_client = new ClientBuilder().node("https://chrysalis-nodes.iota.org/").build();
  // Get the nodeinfo
  console.log(await iota_client.getInfo());
  let message = await iota_client.message().index("test").submit()
  console.log(message);
  let message2 = await iota_client.getMessage().data(message.messageId)
  console.log(message2);
  let messageIds = await iota_client.getMessage().index(new TextEncoder().encode("test"))
  console.log(messageIds);

}

import("../pkg/index.js").then(async iota => {
  try {
    await run(iota)
  } catch (e) {
    console.error(e)
  }
})
