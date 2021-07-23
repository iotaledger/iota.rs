// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { ClientBuilder } = require('../node/iota_client_wasm')

async function main() {
    let iota_client = new ClientBuilder().node("https://api.lb-0.testnet.chrysalis2.com").build();
    // Get the nodeinfo
    // console.log(await iota_client.getInfo());
    // let message = await iota_client.message().index(new TextEncoder().encode("test index")).data(new TextEncoder().encode("test data")).submit()
    // console.log(message);
    // console.log(await iota_client.getMessage().data(message.messageId));
    // console.log(await iota_client.getMessage().raw(message.messageId));
    // console.log(await iota_client.getMessage().children(message.messageId));
    // console.log(await iota_client.getMessage().metadata(message.messageId));
    // console.log(await iota_client.getMessage().index(new TextEncoder().encode("test")));
    // const message = await iota_client
    //     .message()
    //     .seed('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2')
    //     .accountIndex(0)
    //     .output('atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf', BigInt(1000000))
    //     .submit()
    // console.log(message);
}

main().then(() => {
    console.log("All went fine")
}).catch((error) => {
    console.log("Err >", error)
})
