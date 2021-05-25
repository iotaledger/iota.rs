async function run() {
    const {
        ClientBuilder
    } = require('@iota/client');

    // client connects to a node that has MQTT enabled
    const client = new ClientBuilder()
        .node('https://api.hornet-0.testnet.chrysalis2.com')
        .build();

    client.subscriber().topics(['messages']).subscribe((err, data) => {
        console.log(client.getMessageId(data.payload));
        console.log(data);
    })

    const message = await client.message()
        .index('IOTA.RS BINDING - NODE.JS')
        .data('some utf based data')
        .submit();
    console.log(message.message);
    console.log(client.getMessageId(JSON.stringify(message.message)));
    console.log(message);
}

run()
