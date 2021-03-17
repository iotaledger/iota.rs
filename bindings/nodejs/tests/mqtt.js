const { ClientBuilder } = require('../lib')
const assert = require('assert')

const client = new ClientBuilder().node('http://localhost:14265').build()

describe('MQTT', () => {
  // TODO: test not exiting
  /* it('subscribes and unsubscribes to the milestones topic', () => {
    return new Promise(resolve => {
      client.subscriber().topic('milestones/confirmed').subscribe((err, data) => {
        assert.strictEqual(err, null)
        assert.strictEqual(typeof data, 'object')
        assert.strictEqual('topic' in data, true)
        assert.strictEqual(typeof data.topic, 'string')
        assert.strictEqual('payload' in data, true)
        assert.strictEqual(typeof data.payload, 'string')
        client.subscriber().topic('milestones/confirmed').unsubscribe((err) => {
          assert.strictEqual(err, null)
          resolve()
        })
      })
    })
  }) */
})
