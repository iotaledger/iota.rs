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
        const output = await client.getOutput(
            '0xc1e8fe6424b93c4f6aef98b356dd47af3f3bdba510e1f5567f6d4b4c96b78dc80000',
        );
        console.log('Output: ', output);
    } catch (error) {
        console.log('Error: ' + error);
    }
}

run();
