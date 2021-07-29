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
    // console.log(await client.getBalance('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2').accountIndex(0).initialAddressIndex(0).get());
    // console.log(await client.retry('8552b765a0025fe5104b5a99c770e989d675db8c909047c8bed776469e084c36'));

    // const address = "atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf"
    // let hexAddress = client.bech32ToHex(address)
    // let bech32Address = await client.hexToBech32(hexAddress, "atoi")
    // console.log(await client.getHealth());
    // console.log(await client.getTips());
    // console.log(await client.getPeers());
    // const indexation = {
    //     type: 2,
    //     index: Buffer.from(new TextEncoder().encode('iota.rs binding - wasm')).toString('hex'),
    //     data: Buffer.from(new TextEncoder().encode('indexation data')).toString('hex')
    // }
    // console.log(await client.postMessage({ payload: indexation }));
}

main().then(() => {
    console.log("All went fine")
}).catch((error) => {
    console.log("Err >", error)
})
