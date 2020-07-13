const assert = require('assert')
const client = require('../node')

const seed = 'RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA'
const uri = 'https://nodes.comnet.thetangle.org'
const bundleHash = 'MKQKKUKBRQTJEQZRSJCPOABSBEHRMDLRKFHHYYIGZPNKKCDTXHJQBORAX9KEFDBDBZDEWZFOKOCICAUBC'

describe('core', () => {
    it('should get the node info', () => {
        client.addNode(uri)
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
        client.addNode(uri)
        client.addNeighbors('udp://148.148.148.148:14265')
            .then(assertFn)

        client.addNeighbors(['udp://148.148.148.148:14265'])
            .then(assertFn)
    })

    it('should send transfers', () => {
        client.addNode(uri)

        client.sendTransfers(seed, [{
                value: 0
            }], 10)
            .then(transactions => {
                assert.equal(Array.isArray(transactions), true)
                assert.equal(transactions.length, 1)
                assert.equal(typeof transactions[0].isTail, 'boolean')
            })
    })

    it('should traverse bundle', () => {
        client.addNode(uri)

        client.traverseBundle('SVHIDTVSJRHLNFXIFUVYPIWBV9IZGCSMLUZCFOEQMCXMUTHRQCESOIHHKKEVXOUGGOYOSF9ATDMBFK999')
            .then(transactions => {
                assert.equal(Array.isArray(transactions), true)
                assert.equal(transactions.length, 1)
            })
    })

    it('should find transactions', () => {
        client.addNode(uri)
        client.findTransactions([bundleHash]).then(transaction => {
            assert.equal(transaction.hashes.length, 0)
        })
    })

    it('should get transaction trytes', () => {
        client.addNode(uri)
        client.getTrytes([
            'SVHIDTVSJRHLNFXIFUVYPIWBV9IZGCSMLUZCFOEQMCXMUTHRQCESOIHHKKEVXOUGGOYOSF9ATDMBFK999',
            'IITL9EALLVZEGFIFBCCAHUOKHFBIIKQACBCEVVNZUEQLUJTOPXRICFRZKJDQGSVHARJANFDDAHMERS999'
        ]).then(trytes => console.log(trytes))
    })
})