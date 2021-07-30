const assert = require('assert')

function assertMessageWrapper(message) {
  assert.strictEqual(typeof message, 'object')
  assert.strictEqual('parentMessageIds' in message.message, true)
  assert.strictEqual('messageId' in message, true)
  assertMessageId(message.message.parentMessageIds[0])
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
  assertMessageWrapper,
  assertMessageId,
  assertAddress
}