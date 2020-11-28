const { ClientBuilder } = require('../lib/')

const client = new ClientBuilder()
  .node('http://localhost:14265')
  .brokerOptions({ timeout: 50 })
  .build()
client.getTips().then(console.log).catch(console.error)
