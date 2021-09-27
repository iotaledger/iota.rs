async function run() {
    const crypto = require('crypto');
    const seed = crypto.createHash('sha256').update(crypto.randomBytes(256)).digest('hex');
    console.log(seed);

    const { ClientBuilder } = require('../node')
    const client = await new ClientBuilder().build();

    const mnemonic = client.generateMnemonic();
    console.log(mnemonic);

    const hexEncodedSeed = client.mnemonicToHexSeed(mnemonic);
    console.log(hexEncodedSeed);
}

run()
