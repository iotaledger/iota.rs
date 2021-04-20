function run() {
    const crypto = require('crypto');
    var seed = crypto.createHash('sha256').update(crypto.randomBytes(256)).digest('hex');
    console.log(seed);
}

run()
