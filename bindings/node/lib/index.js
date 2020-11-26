const { Client, TopicSubscriber, MessageFinder } = require('../native')

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

Client.prototype.getInfo = promisify(Client.prototype.getInfo)
Client.prototype.getTips = promisify(Client.prototype.getTips)
Client.prototype.postMessage = function (message) {
  return promisify(Client.prototype.postMessage).apply(this, [JSON.stringify(message)])
}
Client.prototype.getOutput = promisify(Client.prototype.getOutput)
Client.prototype.findOutputs = promisify(Client.prototype.findOutputs)
Client.prototype.getAddressOutputs = promisify(Client.prototype.getAddressOutputs)
Client.prototype.getAddressBalance = promisify(Client.prototype.getAddressBalance)
Client.prototype.getMilestone = promisify(Client.prototype.getMilestone)
Client.prototype.retry = promisify(Client.prototype.retry)
Client.prototype.reattach = promisify(Client.prototype.reattach)
Client.prototype.promote = promisify(Client.prototype.promote)
MessageFinder.prototype.index = promisify(MessageFinder.prototype.index)
MessageFinder.prototype.data = promisify(MessageFinder.prototype.data)
MessageFinder.prototype.raw = promisify(MessageFinder.prototype.raw, false)
MessageFinder.prototype.children = promisify(MessageFinder.prototype.children)
MessageFinder.prototype.metadata = promisify(MessageFinder.prototype.metadata)

module.exports = {
  Client
}
