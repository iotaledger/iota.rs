const { Client, TopicSubscriber } = require('../native')

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

module.exports = {
  Client
}
