async function run() {
    const { ClientBuilder } = require('../node')

    // client will connect to testnet by default
    const client = await new ClientBuilder().build();

    const message = await client.message()
        .index(new TextEncoder().encode("iota.rs Wasm binding"))
        .data(new TextEncoder().encode('Testdata'))
        .submit();

    console.log(message);
    console.log("Index:", Buffer.from(message.message.payload.index, 'hex').toString("utf8"));
    console.log("Data:", Buffer.from(message.message.payload.data, 'hex').toString("utf8"));
    let r = await client.retryUntilIncluded(message.messageId);
    console.log(r);
}

run()
