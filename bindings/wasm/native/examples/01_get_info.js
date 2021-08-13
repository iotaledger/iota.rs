const { ClientBuilder } = require('../node/iota_client_wasm')
// async function run() {

//     // client will connect to testnet by default
//     const client = new ClientBuilder()
//         .localPow(true)
//         .build();

//     console.log(await client.getInfo());
// }

// run()

// const { ClientBuilder } = require('../node/iota_client_wasm')

async function run() {
    try {
        let client = new ClientBuilder()
            // .node("https://api.lb-0.testnet.chrysalis2.com")
            //todo fix nodePoolUrls
            .nodePoolUrls(["https://giftiota.com/nodes.json"])
            .localPow(true).build();
        // Get the nodeinfo
        console.log(await client.getInfo());

    } catch (error) {
        console.log(error);
    }
}
run()