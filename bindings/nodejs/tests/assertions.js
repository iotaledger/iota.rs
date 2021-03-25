const assert = require('assert')

function assertMessage(message) {
  assert.strictEqual(typeof message, 'object')
  assert.strictEqual('parents' in message.message, true)
  assert.strictEqual('messageId' in message, true)
  assertMessageId(message.message.parents[0])
}

function assertMessageId(messageId) {
  assert.strictEqual(typeof messageId, 'string')
  assert.strictEqual(messageId.length, 64)
}

function assertAddress(address) {
  assert.strictEqual(typeof address, 'string')
  assert.strictEqual(address.length, 64)
  assert.strictEqual(address.startsWith('atoi'), true)
}

module.exports = {
  assertMessage,
  assertMessageId,
  assertAddress
}