// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { ClientBuilder } = require('../node')

async function main() {
    let client = await new ClientBuilder()
        .node("https://api.lb-0.testnet.chrysalis2.com:443")
        .localPow(false)
        .nodeSyncDisabled(true)
        .quorumSize(1)
        .tipsInterval(19)
        .build();
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
    // console.log(await client.findInputs(["atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf"], BigInt(1000000)));
    // console.log(await client.networkInfo());
    // console.log(await client.getNetworkId());
    // console.log(await client.getBech32Hrp());
    // console.log(await client.getMinPowScore());
    // console.log(await client.retryUntilIncluded(message.messageId, BigInt(5), BigInt(5)));
    // console.log(await client.consolidateFunds('256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2', 0, 0, 10));
}

main().then(() => {
    console.log("All went fine")
}).catch((error) => {
    console.log("Err >", error)
})
