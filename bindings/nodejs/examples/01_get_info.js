
function run() {
    const { Client } = require('@iota/client');

    // client will connect to testnet by default
    const client = new Client({
        "nodes":[
           {
              "url":"http://localhost:14265/",
              "auth":null,
              "disabled":false
           }
        ],
        "localPow":true,
     });

    client.getInfo().then(console.log).catch(console.error);
}

run()
