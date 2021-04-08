async function run(){
    const { ClientBuilder } = require('@iota/client');

    // client will connect to testnet by default
    const client = new ClientBuilder().build();

    const message_data = await client.getMessage().data("e2daa4c6b012b615becd6c12189b2c9e701ba0d53b31a15425b21af5105fc086");
    const message_metadata = await client.getMessage().metadata("e2daa4c6b012b615becd6c12189b2c9e701ba0d53b31a15425b21af5105fc086");
    console.log(message_metadata);
    console.log(message_data);
}

run()