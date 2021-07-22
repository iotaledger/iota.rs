// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { ClientBuilder } = require('../node/iota_client_wasm')

async function main() {
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

main().then(() => {
    console.log("All went fine")
}).catch((error) => {
    console.log("Err >", error)
})
