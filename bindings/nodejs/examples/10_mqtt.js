async function run() {
    const { Client } = require('@iota/client');

    // client connects to a node that has MQTT enabled
    const client = new Client({
        nodes: ['http://localhost:14265/'],
    });

    const callback = function (err, data) {
        console.log(JSON.parse(data));
    };

    client.listen(['milestones/confirmed', 'messages'], callback);
}

run();
