async function run() {
    const {
        ClientBuilder
    } = require('@iota/client');

    // client connects to a node that has MQTT enabled
    const client = new ClientBuilder()
        .node('https://api.hornet-0.testnet.chrysalis2.com')
        .build();

    client.subscriber().topics(['milestones/confirmed', 'messages']).subscribe((err, data) => {
        console.log(data);
    })
}

run()
