async function run() {
    const {
        ClientBuilder
    } = require('@iota/client');

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    client.subscriber().topics(['milestones/confirmed', 'messages']).subscribe((err, data) => {
        console.log(data);
    })
}

run()