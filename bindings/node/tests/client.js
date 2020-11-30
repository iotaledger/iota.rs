const { ClientBuilder } = require('../lib')
const { assertAddress, assertMessageId, assertMessage } = require('./assertions')
const assert = require('assert')

const seed = 'b3a9bf35521157aa9c4508ab3a926634'

const client = new ClientBuilder()
  .node('http://localhost:14265')
  .brokerOptions({ timeout: 50 })
  .build()

describe('gets tips', async () => {
  const tips = await client.getTips()
  assert.strictEqual(Array.isArray(tips), true)
  assert.strictEqual(tips.length, 2)
  assertMessageId(tips[0])
  assertMessageId(tips[1])
})

describe('finds addresses', () => {
  const addresses = client.findAddresses(seed)
    .path("m/0'/0'")
    .range(0, 5)
    .get()
  assert.strictEqual(Array.isArray(addresses), true)
  assert.strictEqual(addresses.length, 5)
  addresses.forEach(assertAddress)
})

describe('get milestone and message', async () => {
  
  const milestone = await client.getMilestone(1)
  assert.strictEqual(typeof milestone, 'object')
  assert.strictEqual('messageId' in milestone, true)
  assertMessageId(milestone.messageId)

  const message = await client.getMessage().data(milestone.messageId)
  assertMessage(message)

  
  const children = await client.getMessage().children(milestone.messageId)
  assert.strictEqual(Array.isArray(children), true)

  const metadata = await client.getMessage().metadata(milestone.messageId)
  assert.strictEqual(typeof metadata, 'object')
  assert.strictEqual('messageId' in metadata, true)
  assertMessageId(metadata.messageId)
  assert.strictEqual(metadata.messageId, milestone.messageId)

  const raw = await client.getMessage().raw(milestone.messageId)
  assert.strictEqual(typeof raw, 'string')
})

/* describe('get address outputs', async () => {
  const outputs = await client.getAddressOutputs()
})*/

/* describe('gets info', async () => {
  const info = await client.getInfo()
  assert.strictEqual(typeof info, 'object')
  assert.strictEqual('name' in info, true)
  assert.strictEqual(info.name, 'Hornet')
}) */
