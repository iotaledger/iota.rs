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
            '0xa22cba0667c922cbb1f8bdcaf970b2a881ccd6e88e2fcce50374de2aac7c37720000',
        );
        console.log('Output: ', output);
    } catch (error) {
        console.log('Error: ' + error);
    }
}

run();
