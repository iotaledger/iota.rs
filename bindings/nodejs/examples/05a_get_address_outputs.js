async function run() {
    const { Client, initLogger } = require('@iota/client');

    initLogger({
        color_enabled: true,
        name: './client.log',
        level_filter: 'debug',
    });

    // client will connect to testnet by default
    const client = new Client({
        nodes: [
            {
                url: 'http://localhost:14265/',
                auth: null,
                disabled: false,
            },
        ],
        localPow: true,
    });

    try {
        const outputIds = await client.getOutputIds([
            {
                address:
                    'atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r',
            },
        ]);
        console.log('Output ids: ', outputIds);

        const address_outputs = await client.getOutputs(outputIds);
        console.log('Address outputs: ', address_outputs);
    } catch (error) {
        console.log('Error: ' + error);
    }
}

run();
