const { ClientBuilder } = require('../lib')
const { assertAddress, assertMessageId, assertMessage } = require('./assertions')
const assert = require('assert')

const seed = '256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2'

const client = new ClientBuilder()
  .node('http://localhost:14265')
  .brokerOptions({ timeout: 50 })
  .build()

describe('Client', () => {
  it('gets tips', async () => {
    const tips = await client.getTips()
    assert.strictEqual(Array.isArray(tips), true)
    assert.strictEqual(tips.length, 2)
    assertMessageId(tips[0])
    assertMessageId(tips[1])
  })

  it('finds addresses', () => {
    const addresses = client.findAddresses(seed)
      .path("m/0'/0'")
      .range(0, 5)
      .get()
    assert.strictEqual(Array.isArray(addresses), true)
    assert.strictEqual(addresses.length, 5)
    addresses.forEach(assertAddress)
  })

  it('get milestone and message', async () => {
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

  it('get address outputs', async () => {
    const outputs = await client.getAddressOutputs('6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92')
    assert.strictEqual(Array.isArray(outputs), true)
    assert.strictEqual(outputs.length > 0, true)
    assert.strictEqual(typeof outputs[0], 'string')
    assert.strictEqual(outputs[0].length, 68)

    const output = await client.getOutput(outputs[0])
    assert.strictEqual(typeof output, 'object')
    assert.strict('messageId' in output, true)
    assertMessageId(output.messageId)
  })

  it('submits an indexation message and reads it', async () => {
    const tips = await client.getTips()
    const indexation = {
      index: 'IOTA.RS BINDING - NODE.JS',
      data: []
    }
    const messageId = await client.postMessage({
      network_id: 0,
      parent1: tips[0],
      parent2: tips[1],
      payload: {
        Indexation: indexation
      },
      nonce: 0
    })
    assertMessageId(messageId)

    const message = await client.getMessage().data(messageId)
    assertMessage(message)
    assert.strictEqual(typeof message.payload.Indexation, 'object')
    assert.deepStrictEqual(message.payload.Indexation, indexation)
  })

  it('gets info', async () => {
    const info = await client.getInfo()
    assert.strictEqual(typeof info, 'object')
    assert.strictEqual('name' in info, true)
    assert.strictEqual(info.name, 'HORNET')
  })

})
