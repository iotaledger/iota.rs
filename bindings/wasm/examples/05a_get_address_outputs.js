async function run() {
    const { ClientBuilder } = require('../node')

    // client will connect to testnet by default
    const client = await new ClientBuilder().build();

    const outputs = await client.getAddress().outputs('atoi1qp9427varyc05py79ajku89xarfgkj74tpel5egr9y7xu3wpfc4lkpx0l86', { includeSpent: false, outputType: "SignatureLockedSingle" });
    console.log(outputs);
}

run()
