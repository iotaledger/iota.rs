// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const {
  Client,
  ClientBuilder,
  TopicSubscriber,
  MessageGetter,
  MessageSender,
  UnspentAddressGetter,
  AddressGetter,
  BalanceGetter
} = require('../build/Release')

function promisify(fn, parse = true) {
  return function () {
    return new Promise((resolve, reject) => fn.apply(this, [...Array.from(arguments), (err, data) => {
      if (err) {
        reject(err)
      } else {
        resolve(parse && data ? JSON.parse(data) : data)
      }
    }]))
  }
}

function poll(instance, cb) {
  instance.poll((err, data) => {
    cb(err, err || !data ? null : JSON.parse(data))
    poll(instance, cb)
  })
}

const subscribe = TopicSubscriber.prototype.subscribe
TopicSubscriber.prototype.subscribe = function (cb) {
  subscribe.call(this, function (err, _) {
    if (err) {
      cb(err)
    }
  })
  poll(this, cb)
}
const unsubscribe = TopicSubscriber.prototype.unsubscribe
TopicSubscriber.prototype.unsubscribe = function (cb) {
  return unsubscribe.call(this, cb || function () { })
}

const brokerOptionsFn = ClientBuilder.prototype.brokerOptions
ClientBuilder.prototype.brokerOptions = function (options) {
  const opt = { ...options }
  if (options.timeout !== undefined) {
    opt.timeout = { secs: options.timeout, nanos: 0 }
  }
  return brokerOptionsFn.apply(this, [JSON.stringify(opt)])
}

const nodeAuthFn = ClientBuilder.prototype.nodeAuth
ClientBuilder.prototype.nodeAuth = function (url, authOptions) {
  return nodeAuthFn.apply(this, [url, JSON.stringify(authOptions)])
}

const primaryNodeFn = ClientBuilder.prototype.primaryNode
ClientBuilder.prototype.primaryNode = function (url, authOptions) {
  if (authOptions !== undefined) {
    return primaryNodeFn.apply(this, [url, JSON.stringify(authOptions)])
  }
  return primaryNodeFn.apply(this, [url])
}

const primaryPowNodeFn = ClientBuilder.prototype.primaryPowNode
ClientBuilder.prototype.primaryPowNode = function (url, authOptions) {
  if (authOptions !== undefined) {
    return primaryPowNodeFn.apply(this, [url, JSON.stringify(authOptions)])
  }
  return primaryPowNodeFn.apply(this, [url])
}

Client.prototype.networkInfo = promisify(Client.prototype.networkInfo)

Client.prototype.findMessages = promisify(Client.prototype.findMessages)
Client.prototype.getAddressBalances = promisify(Client.prototype.getAddressBalances)
Client.prototype.getInfo = promisify(Client.prototype.getInfo)
Client.prototype.getPeers = promisify(Client.prototype.getPeers)
Client.prototype.getTips = promisify(Client.prototype.getTips)
const postMessage = Client.prototype.postMessage
Client.prototype.postMessage = function (message) {
  if (message && message.payload) {
    if ('index' in message.payload) {
      if (typeof message.payload.index === 'string') {
        message.payload.index = new TextEncoder().encode(message.payload.index)
      }
      message.payload.index = Array.from(message.payload.index)
    }

    if ('data' in message.payload) {
      if (typeof message.payload.data === 'string') {
        message.payload.data = new TextEncoder().encode(message.payload.data)
      }
      message.payload.data = Array.from(message.payload.data)
    }
  }
  return promisify(postMessage).apply(this, [JSON.stringify(message)])
}
Client.prototype.getOutput = promisify(Client.prototype.getOutput)
Client.prototype.findOutputs = promisify(Client.prototype.findOutputs)
const getAddressOutputs = Client.prototype.getAddressOutputs
Client.prototype.getAddressOutputs = function (address, options) {
  if (typeof options == 'undefined') {
    options = {
      includeSpent: false
    }
  }
  return promisify(getAddressOutputs).apply(this, [address, JSON.stringify(options)])
}
Client.prototype.getAddressBalance = promisify(Client.prototype.getAddressBalance)
Client.prototype.getMilestone = promisify(Client.prototype.getMilestone)
Client.prototype.getMilestoneUtxoChanges = promisify(Client.prototype.getMilestoneUtxoChanges)
Client.prototype.getReceipts = promisify(Client.prototype.getReceipts)
Client.prototype.getReceiptsMigratedAt = promisify(Client.prototype.getReceiptsMigratedAt)
Client.prototype.getTreasury = promisify(Client.prototype.getTreasury)
Client.prototype.getIncludedMessage = promisify(Client.prototype.getIncludedMessage)

Client.prototype.retry = promisify(Client.prototype.retry)
const retryUntilIncluded = Client.prototype.retryUntilIncluded
Client.prototype.retryUntilIncluded = function (msg_id, interval, maxAttempts) {
  if (typeof interval == 'undefined') {
    interval = 5
  }
  if (typeof maxAttempts == 'undefined') {
    maxAttempts = 10
  }
  return promisify(retryUntilIncluded).apply(this, [msg_id, interval, maxAttempts])
}
Client.prototype.consolidateFunds = promisify(Client.prototype.consolidateFunds)
Client.prototype.reattach = promisify(Client.prototype.reattach)
Client.prototype.promote = promisify(Client.prototype.promote)
Client.prototype.hexToBech32 = promisify(Client.prototype.hexToBech32)

const messageGetterIndexSetter = promisify(MessageGetter.prototype.index)
MessageGetter.prototype.index = function (index) {
  if (typeof index === 'string') {
    index = new TextEncoder().encode(index)
  }
  return messageGetterIndexSetter.apply(this, [Array.from(index)])
}
MessageGetter.prototype.data = promisify(MessageGetter.prototype.data)
MessageGetter.prototype.raw = promisify(MessageGetter.prototype.raw, false)
MessageGetter.prototype.children = promisify(MessageGetter.prototype.children)
MessageGetter.prototype.metadata = promisify(MessageGetter.prototype.metadata)

MessageSender.prototype.submit = promisify(MessageSender.prototype.submit)
const messageSenderDataSetter = MessageSender.prototype.data
MessageSender.prototype.data = function (data) {
  if (typeof data === 'string') {
    data = new TextEncoder().encode(data)
  }
  return messageSenderDataSetter.apply(this, [Array.from(data)])
}
const messageSenderIndexSetter = MessageSender.prototype.index
MessageSender.prototype.index = function (index) {
  if (typeof index === 'string') {
    index = new TextEncoder().encode(index)
  }
  return messageSenderIndexSetter.apply(this, [Array.from(index)])
}

UnspentAddressGetter.prototype.get = promisify(UnspentAddressGetter.prototype.get)

AddressGetter.prototype.get = promisify(AddressGetter.prototype.get)

BalanceGetter.prototype.get = promisify(BalanceGetter.prototype.get)

module.exports = {
  ClientBuilder
}
