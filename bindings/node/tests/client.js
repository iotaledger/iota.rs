const { ClientBuilder } = require('../lib/')

const client = new ClientBuilder()
  .node('http://localhost:14265')
  .brokerOptions({ timeout: 50 })
  .build()
client.getTips().then(console.log).catch(console.error)

const addresses = client.findAddresses('b3a9bf35521157aa9c4508ab3a926634')
  .path("m/0'/0'")
  .range(0, 5)
  .get()
console.log(addresses)
