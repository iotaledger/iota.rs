// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const {
  Client,
  ClientBuilder,
  TopicSubscriber,
  MessageGetter,
  MessageSender,
  UnspentAddressGetter,
  AddressFinder,
  BalanceGetter
} = require('../native')

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

const getNetworkInfo = Client.prototype.networkInfo
Client.prototype.networkInfo = function () {
  return JSON.parse(getNetworkInfo.apply(this, []))
}

Client.prototype.findMessages = promisify(Client.prototype.findMessages)
Client.prototype.getAddressBalances = promisify(Client.prototype.getAddressBalances)
Client.prototype.getInfo = promisify(Client.prototype.getInfo)
Client.prototype.getPeers = promisify(Client.prototype.getPeers)
Client.prototype.getTips = promisify(Client.prototype.getTips)
const postMessage = Client.prototype.postMessage
Client.prototype.postMessage = function (message) {
  if (message && message.payload && message.payload.data instanceof Uint8Array) {
    message.payload.data = Array.from(message.payload.data)
  }
  return promisify(postMessage).apply(this, [JSON.stringify(message)])
}
Client.prototype.getOutput = promisify(Client.prototype.getOutput)
Client.prototype.findOutputs = promisify(Client.prototype.findOutputs)
Client.prototype.getAddressOutputs = promisify(Client.prototype.getAddressOutputs)
Client.prototype.getAddressBalance = promisify(Client.prototype.getAddressBalance)
Client.prototype.getMilestone = promisify(Client.prototype.getMilestone)
Client.prototype.getMilestoneUTXOChanges = promisify(Client.prototype.getMilestoneUTXOChanges)
Client.prototype.retry = promisify(Client.prototype.retry)
Client.prototype.reattach = promisify(Client.prototype.reattach)
Client.prototype.promote = promisify(Client.prototype.promote)

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

const findAddressesGetter = AddressFinder.prototype.get
AddressFinder.prototype.get = function () {
  return JSON.parse(findAddressesGetter.apply(this))
}

BalanceGetter.prototype.get = promisify(BalanceGetter.prototype.get)

module.exports = {
  ClientBuilder
}
