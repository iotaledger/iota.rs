const {
  Client,
  ClientBuilder,
  TopicSubscriber,
  MessageGetter,
  ValueTransactionSender,
  UnspentAddressGetter,
  AddressFinder,
  BalanceGetter
} = require('../native')

function promisify (fn, parse = true) {
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

const brokerOptionsFn = ClientBuilder.prototype.brokerOptions
ClientBuilder.prototype.brokerOptions = function (options) {
  const opt = { ...options }
  if (options.timeout !== undefined) {
    opt.timeout = { secs: options.timeout, nanos: 0 }
  }
  return brokerOptionsFn.apply(this, [JSON.stringify(opt)])
}

Client.prototype.findMessages = promisify(Client.prototype.findMessages)
Client.prototype.getInfo = promisify(Client.prototype.getInfo)
Client.prototype.getTips = promisify(Client.prototype.getTips)
const postMessage = Client.prototype.postMessage
Client.prototype.postMessage = function (message) {
  return promisify(postMessage).apply(this, [JSON.stringify(message)])
}
Client.prototype.getOutput = promisify(Client.prototype.getOutput)
Client.prototype.findOutputs = promisify(Client.prototype.findOutputs)
Client.prototype.getAddressOutputs = promisify(Client.prototype.getAddressOutputs)
Client.prototype.getAddressBalance = promisify(Client.prototype.getAddressBalance)
Client.prototype.getMilestone = promisify(Client.prototype.getMilestone)
Client.prototype.retry = promisify(Client.prototype.retry)
Client.prototype.reattach = promisify(Client.prototype.reattach)
Client.prototype.promote = promisify(Client.prototype.promote)

MessageGetter.prototype.index = promisify(MessageGetter.prototype.index)
MessageGetter.prototype.data = promisify(MessageGetter.prototype.data)
MessageGetter.prototype.raw = promisify(MessageGetter.prototype.raw, false)
MessageGetter.prototype.children = promisify(MessageGetter.prototype.children)
MessageGetter.prototype.metadata = promisify(MessageGetter.prototype.metadata)

ValueTransactionSender.prototype.send = promisify(ValueTransactionSender.prototype.send, false)

UnspentAddressGetter.prototype.get = promisify(UnspentAddressGetter.prototype.get)

const findAddressesGetter = AddressFinder.prototype.get
AddressFinder.prototype.get = function () {
  return JSON.parse(findAddressesGetter.apply(this))
}

BalanceGetter.prototype.get = promisify(BalanceGetter.prototype.get)

module.exports = {
  ClientBuilder
}
