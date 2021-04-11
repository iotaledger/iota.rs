async function run() {
    const { ClientBuilder } = require('@iota/client');

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    const indexation = {
        index: 'IOTA.RS BINDING - NODE.JS',
        data: new TextEncoder().encode('some utf based data')
    };

    const messageId = await client.postMessage({
        payload: indexation
    });
}

run()