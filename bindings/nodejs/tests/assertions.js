const assert = require('assert')

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
  assertMessageId,
  assertAddress
}