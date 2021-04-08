
function run(){
    const { ClientBuilder } = require('@iota/client')

    // client will connect to testnet by default
    const client = new ClientBuilder().build()

    client.getInfo().then(console.log).catch(console.error)
}

run()