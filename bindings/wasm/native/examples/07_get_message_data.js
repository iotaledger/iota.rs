async function run() {
    const { ClientBuilder } = require('../node/iota_client_wasm')

    // client will connect to testnet by default
    const client = await new ClientBuilder().build();

    // get message data by message id (get a random message id with getTips)
    const tips = await client.getTips();
    const message_data = await client.getMessage().data(tips[0]);
    const message_metadata = await client.getMessage().metadata(tips[0]);
    console.log(message_metadata);
    console.log(message_data);

    // get indexation data by index
    const messages = await client.getMessage().index(new TextEncoder().encode("iota.rs Wasm binding"))
    for (message in messages) {
        const message = await client.getMessage().data(messages[0])
        console.log(Buffer.from(message.message.payload.data, 'hex').toString('utf8'));
    }
}

run()
