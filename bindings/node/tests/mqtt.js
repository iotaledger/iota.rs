const { ClientBuilder } = require('../lib')
const assert = require('assert')

const client = new ClientBuilder().node('http://localhost:14265').build()

describe('subscribes to MQTT topics', () => {
  return new Promise(resolve => {
    client.subscriber().topic('milestones/solid').subscribe((err, data) => {
      assert.strictEqual(err, null)
      assert.strictEqual(typeof data, 'object')
      assert.strictEqual('topic' in data, true)
      assert.strictEqual(typeof data.topic, 'string')
      assert.strictEqual('payload' in data, true)
      assert.strictEqual(typeof data.payload, 'string')
      resolve()
    })
  })
})
