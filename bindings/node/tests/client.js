const { Client } = require('../lib/')

const client = new Client('http://localhost:14265')
client.getTips().then(console.log).catch(console.error)
