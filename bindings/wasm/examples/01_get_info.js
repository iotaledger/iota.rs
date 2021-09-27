const { ClientBuilder } = require('../node/')

async function run() {
    let client = await new ClientBuilder()
        .node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")
        .build();
    // Get the nodeinfo
    console.log(await client.getInfo());
}
run()
