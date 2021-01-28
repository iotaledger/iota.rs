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
  assert.strictEqual(typeof address, 'string')
  assert.strictEqual(address.length, 63)
  assert.strictEqual(address.startsWith('iot1'), true)
}

module.exports = {
  assertMessage,
  assertMessageId,
  assertAddress
}