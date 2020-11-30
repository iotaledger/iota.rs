const assert = require('assert')

function assertMessage(message) {
  assert.strictEqual(typeof message, 'object')
  assert.strictEqual('parent1' in message, true)
  assert.strictEqual('parent2' in message, true)
  assertMessageId(message.parent1)
  assertMessageId(message.parent2)
}

function assertMessageId(messageId) {
  assert.strictEqual(typeof messageId, 'string')
  assert.strictEqual(messageId.length, 64)
}

function assertAddress(address) {
  assert.strictEqual(typeof address, 'object')
  assert.strictEqual('type' in address, true)
  assert.strictEqual(typeof address.type, 'string')
  assert.strictEqual(address.type, 'Ed25519')

  assert.strictEqual('data' in address, true)
  assert.strictEqual(typeof address.data, 'string')
  assert.strictEqual(address.data.length, 64)
}

module.exports = {
  assertMessage,
  assertMessageId,
  assertAddress
}