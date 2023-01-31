const { ClientBuilder } = require('../node/client_wasm')
const { assertAddress, assertMessageId, assertMessageWrapper } = require('./assertions')
const assert = require('assert')
const TestVectors = require('../../../tests/fixtures/test_vectors.json')

const seed = '256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2'

async function test() {

  const client = await new ClientBuilder()
    .node('https://api.lb-1.h.chrysalis-devnet.iota.cafe/')
    .build()

  describe('Client', () => {
    it('gets network info', async () => {
      const info = await client.networkInfo()
      assert.strictEqual(typeof info, 'object')
      assert.strictEqual(info.localPow, false)
      assert.strictEqual(info.bech32HRP, 'atoi')
      // 4000 in mainnet, 2000 in devnet
      assert.strictEqual(info.minPoWScore, 2000)
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

    it('convert address', async () => {
      const address = "atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf"
      let hexAddress = client.bech32ToHex(address)
      let bech32Address = await client.hexToBech32(hexAddress, "atoi")
      assert.strictEqual(address, bech32Address)
    })

    it('sends an indexation message with the high level API', async () => {
      const messageWrapper = await client
        .message()
        .index(new TextEncoder().encode("INDEX"))
        .data(new TextEncoder().encode('MESSAGE'))
        .submit()
      assertMessageWrapper(messageWrapper)
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
      const info = await client.getInfo()
      const milestone = await client.getMilestone(info.nodeinfo.confirmedMilestoneIndex)
      assert.strictEqual(typeof milestone, 'object')
      assert.strictEqual('messageId' in milestone, true)
      assertMessageId(milestone.messageId)
      const message = await client.getMessage().data(milestone.messageId)
      assertMessageWrapper(message)


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
      const outputs = await client.getAddress().outputs('atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf', { includeSpent: false })
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
        type: 2,
        index: Buffer.from(new TextEncoder().encode('iota.rs binding - wasm')).toString('hex'),
        data: Buffer.from(new TextEncoder().encode('indexation data')).toString('hex')
      }
      const messageId = await client.postMessage({
        payload: indexation
      })
      assertMessageId(messageId)

      const message = await client.getMessage().data(messageId)
      assertMessageWrapper(message)
      assert.strictEqual(message.message.payload.type, 2)
      assert.strictEqual(typeof message.message.payload.data, 'string')
      const decodedIndex = Buffer.from(message.message.payload.index, 'hex').toString("utf8");
      assert.deepStrictEqual(decodedIndex, 'iota.rs binding - wasm')
      const decodedData = Buffer.from(message.message.payload.data, 'hex').toString("utf8");
      assert.deepStrictEqual(decodedData, 'indexation data')
    })

    it('gets info', async () => {
      const info = await client.getInfo()
      assert.strictEqual(typeof info.nodeinfo, 'object')
      assert.strictEqual('name' in info.nodeinfo, true)
      assert.strictEqual(info.nodeinfo.name, 'HORNET')
    })

    it('public key to address', async () => {
      const address = await client.hexPublicKeyToBech32Address("2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a")
      assert.strictEqual(address, 'atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r')
    })

    it('transaction id', async () => {
      const transaction_id = await client.getTransactionId(JSON.stringify({
        "type": 0,
        "essence": {
          "type": 0,
          "inputs": [
            {
              "type": 0,
              "transactionId": "fd9a483662c27877825f227fcebdc4f43b3897e6445573610bf0217f0c1d2f43",
              "transactionOutputIndex": 0
            }
          ],
          "outputs": [
            {
              "type": 0,
              "address": {
                "type": 0,
                "address": "2aacd2b0fb5d641e5369fd70696750706ffc1c759302d6cacc33249b7e487f5b"
              },
              "amount": 30000000
            },
            {
              "type": 0,
              "address": {
                "type": 0,
                "address": "96f9de0989e77d0e150e850a5a600e83045fa57419eaf3b20225b763d4e23813"
              },
              "amount": 8000000
            }
          ],
          "payload": null
        },
        "unlockBlocks": [
          {
            "type": 0,
            "signature": {
              "type": 0,
              "publicKey": "2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a",
              "signature": "65fe3702699418bbb973265a2a4d8de0b5f8a1e70a7c3932b2679aac5176c289248e1d5f48f82760c2fead97deed59d68ed74577bf00442eddd9093648f1a60e"
            }
          }
        ]
      }))
      assert.strictEqual(transaction_id, 'dda4d34eb0138eecd58f3a4cade9f35ea593866e69f657910cebc63297e5898c')
    })

    it('essence_hash', () => {
      const transaction_essence = {
        "type": "Regular",
        "data": {
          "inputs": [
            {
              "type": "Utxo",
              "data": "738ef491bdb2ac1f28368760272773644d53a8a4dbf27e34f271fb5f3e99e2780000"
            }
          ],
          "outputs": [
            {
              "type": "SignatureLockedSingle",
              "data": {
                "address": {
                  "type": "Ed25519",
                  "data": "96f9de0989e77d0e150e850a5a600e83045fa57419eaf3b20225b763d4e23813"
                },
                "amount": 880000177
              }
            },
            {
              "type": "SignatureLockedSingle",
              "data": {
                "address": {
                  "type": "Ed25519",
                  "data": "f8208cdcda8b1afc710fbcb2e822fe70661c3213172189b845d49a64dd52a7a4"
                },
                "amount": 1000000
              }
            }
          ],
          "payload": null
        }
      };
      const essence_hash = client.getEssenceHash(JSON.stringify(transaction_essence))
      assert.strictEqual(essence_hash, "509d0ba50e38b37d5c4e446f54c07378ad6811b32343388e7b3b02b7b86c1b09")
    })

    it('mnemonic to address conversion', async () => {
      const mnemonic = TestVectors['general']['MNEMNONIC'];
      const address = TestVectors['general']['MNEMNONIC_ADDRESS'];
  
      const seed = await client.mnemonicToHexSeed(mnemonic)
  
      const generatedAddresses = await client.getAddresses(seed)
        .accountIndex(0)
        .bech32Hrp('iota')
        .range(0, 1)
        .get()
  
      assert.strictEqual(address, generatedAddresses[0])
    })

  })

  // transaction tests disabled for workflows, because they fail if we don't have funds
  // it('sends a value transaction and checks output balance', async () => {
  //   const depositAddress = 'atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf'
  //   const message = await client
  //     .message()
  //     .seed(seed)
  //     .accountIndex(0)
  //     .output(depositAddress, BigInt(1000000))
  //     .submit()
  //   assertMessageWrapper(message)

  //   while (true) {
  //     const metadata = await client.getMessage().metadata(message.messageId)
  //     if (metadata.ledgerInclusionState) {
  //       assert.strictEqual(metadata.ledgerInclusionState, 'included')
  //       break
  //     } else {
  //       await new Promise(resolve => setTimeout(resolve, 2000))
  //     }
  //   }

  //   const addressBalanceObject = await client.getAddress().balance(depositAddress)
  //   assert.strictEqual(addressBalanceObject.balance >= 1000000, true)
  // })

  // it('offline transaction', async () => {
  //   const seed = '256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2'
  //   const addresses = await client.getAddresses(seed)
  //     .bech32Hrp("atoi")
  //     .accountIndex(0)
  //     .range(0, 2)
  //     .get();
  //   let inputs = [];
  //   try {
  //     inputs = await client.findInputs(addresses, BigInt(1000000));
  //   } catch (e) { console.log };
  //   // only try to send a transaction if we have inputs
  //   if (inputs.length > 0) {
  //     const prepared_transaction = await client
  //       .message()
  //       .input(inputs[0])
  //       .output('atoi1qz4sfmp605vnj6fxt0sf0cwclffw5hpxjqkf6fthyd74r9nmmu337m3lwl2', BigInt(1000000))
  //       .prepareTransaction();
  //     const signed_transaction = await client
  //       .message()
  //       .signTransaction(prepared_transaction, seed);
  //     const message = await client
  //       .message()
  //       .finishMessage(signed_transaction);
  //   }
  // })
}
test()
