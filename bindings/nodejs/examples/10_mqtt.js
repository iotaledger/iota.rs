async function run() {
    const {
        ClientBuilder
    } = require('@iota/client');

    // client connects to a node that has MQTT enabled
    const client = new ClientBuilder()
        .node('https://api.hornet-2.testnet.chrysalis2.com')
        .build();

    client.subscriber().topics(['milestones/confirmed', 'messages']).subscribe((err, data) => {
        console.log(data);
        // To get the message id from messages `client.getMessageId(data.payload)` can be used
    })

    await new Promise(resolve => setTimeout(resolve, 1500));
    // unsubscribe from 'messages' topic, will continue to receive events for 'milestones/confirmed'
    client.subscriber().topics(['messages']).unsubscribe((err, data) => {
        console.log(data);
    })
}

run()
