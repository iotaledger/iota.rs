// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { ClientBuilder } = require('../node/iota_client_wasm')

async function main() {
    let client = new ClientBuilder().node("https://api.lb-0.testnet.chrysalis2.com").build();
    // Get the nodeinfo
    console.log(await client.getInfo());
    // let message = await client.message().index(new TextEncoder().encode("test index")).data(new TextEncoder().encode("test data")).submit()
    // console.log(message);
    // console.log(await client.getMessage().data(message.messageId));
    // console.log(await client.getMessage().raw(message.messageId));
    // console.log(await client.getMessage().children(message.messageId));
    // console.log(await client.getMessage().metadata(message.messageId));
    // console.log(await client.getMessage().index(new TextEncoder().encode("test")));
    // const message = await client
    //     .message()
    //     .seed('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2')
    //     .accountIndex(0)
    //     .output('atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf', BigInt(1000000))
    //     .submit()
    // console.log(message);
    // const addresses = await client.getAddresses('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2')
    //     .accountIndex(0)
    //     .range(0, 5)
    //     .includeInternal()
    //     .get();
    // console.log(addresses);
    // const unspent_address = await client.getUnspentAddress('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2')
    //     .accountIndex(0)
    //     .initialAddressIndex(0)
    //     .get();
    // console.log(unspent_address);
}

main().then(() => {
    console.log("All went fine")
}).catch((error) => {
    console.log("Err >", error)
})
