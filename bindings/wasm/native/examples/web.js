async function run(Iota) {
  console.log(Iota)

  const {
    ClientBuilder
  } = Iota

  let client = await new ClientBuilder().node("https://api.lb-0.h.chrysalis-devnet.iota.cafe").build();
  // Get the nodeinfo
  console.log(await client.getInfo());
  // let message = await client.message().index(new TextEncoder().encode("test")).submit();
  // let read = await client.getMessage().data(message.messageId)
  // // function to convert hex data back to utf
  // function hexToUtf8(hex) {
  //   for (var bytes = [], c = 0; c < hex.length; c += 2)
  //     bytes.push(parseInt(hex.substr(c, 2), 16));
  //   return new TextDecoder().decode(new Uint8Array(bytes));
  // }
  // console.log(hexToUtf8(read.message.payload.index));

  // console.log(await client.message().index(new TextEncoder().encode("test")).submit());
  // console.log(await client.getMessage().index(new TextEncoder().encode("test")));
  // console.log(await client.retryUntilIncluded("131d9d289ca4258a1b1874257532340bcd8155a548bbacdeec147c69a1636b51", BigInt(2), BigInt(2)));
  // console.log(await client.getBalance('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2').accountIndex(0).initialAddressIndex(0).get());
  // console.log(await client.retry('8552b765a0025fe5104b5a99c770e989d675db8c909047c8bed776469e084c36'));
  // const address = "atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf"
  // let hexAddress = client.bech32ToHex(address)
  // let bech32Address = await client.hexToBech32(hexAddress, "atoi")
  // console.log(await client.getHealth());
  // console.log(await client.getTips());
  // console.log(await client.getPeers());
  // function ascii_to_hex(str) {
  //   var arr1 = [];
  //   for (var n = 0, l = str.length; n < l; n++) {
  //     var hex = Number(str.charCodeAt(n)).toString(16);
  //     arr1.push(hex);
  //   }
  //   return arr1.join('');
  // }
  // const indexation = {
  //   type: 2,
  //   index: ascii_to_hex('iota.rs binding - wasm'),
  //   data: ascii_to_hex('indexation data')
  // }
  // console.log(await client.postMessage({ payload: indexation }));
  // console.log(await client.findMessages(["iota.rs binding - wasm"], ["3866d368da51b4f731c0643106b7d191bba0c9dd553e5786e62294b028dc4d6e"]));
  // console.log(await client.getAddressBalances(["atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf"]));
  // console.log(client.isAddressValid("atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf"));
  // console.log(client.generateMnemonic());
  // console.log(client.mnemonicToHexSeed(client.generateMnemonic()));
  // console.log(await client.getOutput("17057e92991f836ff2f0f88f2abb93ba0d8eda37efc1312daad599c1326bce310100"));
  // console.log(await client.getAddress().balance("atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf"));
  // console.log(await client.getAddress().outputs("atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf", { includeSpent: false }));
  // console.log(await client.findOutputs(["17057e92991f836ff2f0f88f2abb93ba0d8eda37efc1312daad599c1326bce310100"], ["atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf"]));
  // console.log(await client.getMilestone(973351));
  // console.log(await client.getMilestoneUtxoChanges(973351));
  // console.log(await client.getReceipts());
  // console.log(await client.getReceiptsMigratedAt(973351));
  // console.log(await client.getTreasury());
  // console.log(await client.getIncludedMessage("17057e92991f836ff2f0f88f2abb93ba0d8eda37efc1312daad599c1326bce31"));
  // console.log(await client.retryUntilIncluded(message.messageId, BigInt(5), BigInt(5)));
  // console.log(await client.consolidateFunds('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2', 0, 0, 10));

}

import("../pkg/index.js").then(async iota => {
  try {
    await run(iota)
  } catch (e) {
    console.error(e)
  }
})
