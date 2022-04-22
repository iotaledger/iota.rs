async function run() {
    const { Client } = require('@iota/client');

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
        const mnemonic = await client.generateMnemonic();

        // Generate addresses with custom account index and range
        const options = {
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        };
        const signer = JSON.stringify({ Mnemonic: mnemonic });

        const addresses = await client.generateAddresses(signer, options);

        console.log('First public address: ', addresses[0]);
    } catch (error) {
        console.log('Error: ', error);
    }

    process.exit();
}

run();
