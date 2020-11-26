const { Client, TopicSubscriber, MessageFinder } = require('../native')

function promisify (fn) {
  return function () {
    return new Promise((resolve, reject) => fn.apply(this, [...Array.from(arguments), (err, data) => {
      if (err) {
        reject(err)
      } else {
        resolve(data)
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
MessageFinder.prototype.index = promisify(MessageFinder.prototype.index)
MessageFinder.prototype.data = promisify(MessageFinder.prototype.data)
MessageFinder.prototype.raw = promisify(MessageFinder.prototype.raw)
MessageFinder.prototype.children = promisify(MessageFinder.prototype.children)
MessageFinder.prototype.metadata = promisify(MessageFinder.prototype.metadata)

module.exports = {
  Client
}
