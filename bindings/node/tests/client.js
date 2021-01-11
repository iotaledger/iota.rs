const { ClientBuilder } = require('../lib')
const { assertAddress, assertMessageId, assertMessage } = require('./assertions')
const assert = require('assert')

const seed = 'b3a9bf35521157aa9c4508ab3a9266e210ae297ff5a4584234c4d9e7d01712e3'

const client = new ClientBuilder()
  .node('http://localhost:14265')
  .brokerOptions({ timeout: 50 })
  .localPow(false)
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
      .accountIndex(0)
      .range(0, 5)
      .get()
    assert.strictEqual(Array.isArray(addresses), true)
    assert.strictEqual(addresses.length, 10)
    addresses.forEach(([address, _internal]) => assertAddress(address))
  })

  it('sends an indexation message with the high level API', async () => {
    const messageId = await client
      .send()
      .with_index('IOTA.RS TEST')
      .data(new TextEncoder().encode('MESSAGE'))
      .submit()
    assertMessageId(messageId)
  })

  it('sends a value transaction and checks output balance', async () => {
    const depositAddress = 'iot1q9jyad2efwyq7ldg9u6eqg5krxdqawgcdxvhjlmxrveylrt4fgaqj30s9qj'
    const messageId = await client
      .send()
      .with_seed(seed)
      .accountIndex(0)
      .output(depositAddress, 2)
      .submit()
    assertMessageId(messageId)

    while (true) {
      const metadata = await client.getMessage().metadata(messageId)
      if (metadata.ledgerInclusionState) {
        assert.strictEqual(metadata.ledgerInclusionState, 'included')
        break
      } else {
        await new Promise(resolve => setTimeout(resolve, 2000))
      }
    }

    const depositBalance = await client.getAddressBalance(depositAddress)
    assert.strictEqual(depositBalance >= 2, true)
  })

  it('gets an unspent address', async () => {
    const res = await client.getUnspentAddress(seed).initialAddressIndex(5).accountIndex(0).get()
    assert.strictEqual(Array.isArray(res), true)
    assert.strictEqual(res.length, 2)
    const [address, index] = res
    assertAddress(address)
    assert.strictEqual(index, 5)
  })

  it('gets seed balance', async () => {
    const balance = await client.getBalance(seed).accountIndex(0).initialAddressIndex(50000).get()
    assert.strictEqual(balance, 0)
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
    const outputs = await client.getAddressOutputs('iot1q95jpvtk7cf7c7l9ne50c684jl4n8ya0srm5clpak7qes9ratu0eyf5eyz5')
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
    const indexation = {
      index: 'IOTA.RS BINDING - NODE.JS',
      data: new TextEncoder().encode('INDEXATION DATA')
    }
    const messageId = await client.postMessage({
      payload: indexation
    })
    assertMessageId(messageId)

    const message = await client.getMessage().data(messageId)
    assertMessage(message)
    assert.strictEqual(message.payload.type, 'Indexation')
    assert.strictEqual(typeof message.payload.data, 'object')
    assert.deepStrictEqual(message.payload.data, indexation)
  })

  it('gets info', async () => {
    const info = await client.getInfo()
    assert.strictEqual(typeof info, 'object')
    assert.strictEqual('name' in info, true)
    assert.strictEqual(info.name, 'HORNET')
  })
})
