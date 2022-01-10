const { ClientBuilder } = require('../node/')

async function run() {
    let client = await new ClientBuilder()
        .node("http://localhost:14265")
        .build();
    // Get the nodeinfo
    console.log(await client.getInfo());
}
run()
