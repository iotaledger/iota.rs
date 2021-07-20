// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Client } = require('../node/iota_client_wasm')

async function main() {
    // Get the nodeinfo
    let iota_client = await Client.withNode("https://api.lb-0.testnet.chrysalis2.com/");
    return await iota_client.getInfo()
}

main().then((output) => {
    console.log("Ok >", output)
}).catch((error) => {
    console.log("Err >", error)
})
