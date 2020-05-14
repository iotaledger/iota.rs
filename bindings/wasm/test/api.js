const assert = require('assert')
const { Client } = require('../node')

const seed = 'RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA'
const uri = 'https://nodes.comnet.thetangle.org'

describe('core', () => {
    it('should get the node info', () => {
        const client = new Client(uri)
        client.getNodeInfo().then(info => {
            assert.equal(typeof info, 'object')
            assert.equal(info.appName, 'IRI Comnet')
        })
    })

    it('should add neighbors', () => {
        const assertFn = response => {
            assert.equal(typeof response, 'object')
            console.log(response)
            assert.equal(response.addedNeighbors, null)
        }
        const client = new Client(uri)
        client.addNeighbors('udp://148.148.148.148:14265')
            .then(assertFn)

        client.addNeighbors(['udp://148.148.148.148:14265'])
            .then(assertFn)
    })

    it('should send transfers', () => {
        const client = new Client(uri)

        client.sendTransfers(seed, [{ value: 0 }], 10)
            .then(transactions => {
                assert.equal(Array.isArray(transactions), true)
                assert.equal(transactions.length, 1)
                assert.equal(typeof transactions[0].isTail, 'boolean')
            })
    })
})
