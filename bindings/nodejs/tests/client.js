const { ClientBuilder } = require('../lib')
const { assertAddress, assertMessageId, assertMessage } = require('./assertions')
const assert = require('assert')

const seed = '256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2'

const client = new ClientBuilder()
  .node('http://api.hornet-1.testnet.chrysalis2.com')
  .disableNodeSync()
  .brokerOptions({ timeout: 50 })
  .localPow(true)
  .build()

describe('Client', () => {
  it('gets network info', async () => {
    const info = await client.networkInfo()
    assert.strictEqual(typeof info, 'object')
    assert.strictEqual(info.localPow, true)
    assert.strictEqual(info.bech32HRP, 'atoi')
    assert.strictEqual(info.minPowScore, 4000)
  })

  it('gets tips', async () => {
    const tips = await client.getTips()
    assert.strictEqual(Array.isArray(tips), true)
    assertMessageId(tips[0])
  })

  it('get addresses', async () => {
    const addresses = await client.getAddresses(seed)
      .accountIndex(0)
      .range(0, 5)
      .get()
    assert.strictEqual(Array.isArray(addresses), true)
    assert.strictEqual(addresses.length, 5)
    addresses.forEach(assertAddress)
  })

  it('sends an indexation message with the high level API', async () => {
    const message = await client
      .message()
      .index('IOTA.RS TEST')
      .data(new TextEncoder().encode('MESSAGE'))
      .submit()
    assertMessage(message)
  })

  it('sends a value transaction and checks output balance', async () => {
    const depositAddress = 'atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r'
    const message = await client
      .message()
      .seed(seed)
      .accountIndex(0)
      .output(depositAddress, 1000000)
      .submit()
    assertMessage(message)

    while (true) {
      const metadata = await client.getMessage().metadata(message.messageId)
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
    const milestone = await client.getMilestone(750)
    assert.strictEqual(typeof milestone, 'object')
    assert.strictEqual('message_id' in milestone, true)
    assertMessageId(milestone.message_id)

    const message = await client.getMessage().data(milestone.message_id)
    assertMessage(message)


    const children = await client.getMessage().children(milestone.message_id)
    assert.strictEqual(Array.isArray(children), true)

    const metadata = await client.getMessage().metadata(milestone.message_id)
    assert.strictEqual(typeof metadata, 'object')
    assert.strictEqual('messageId' in metadata, true)
    assertMessageId(metadata.messageId)
    assert.strictEqual(metadata.messageId, milestone.message_id)

    const raw = await client.getMessage().raw(milestone.message_id)
    assert.strictEqual(typeof raw, 'string')
  })

  it('get address outputs', async () => {
    const outputs = await client.getAddressOutputs('atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r')
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
    assert.strictEqual(message.message.payload.type, 'Indexation')
    assert.strictEqual(typeof message.message.payload.data, 'object')
    assert.deepStrictEqual(message.message.payload.data, indexation)
  })

  it('gets info', async () => {
    const info = await client.getInfo()
    assert.strictEqual(typeof info, 'object')
    assert.strictEqual('name' in info, true)
    assert.strictEqual(info.name, 'HORNET')
  })
})
