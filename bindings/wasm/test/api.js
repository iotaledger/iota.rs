const assert = require('assert')
const { Client } = require('../node')

describe('core', () => {
    it('should get the node info', () => {
        const client = new Client('https://nodes.comnet.thetangle.org')
        client.getNodeInfo().then(info => {
            assert.equal(typeof info, 'object')
            assert.equal(info.appName, 'IRI Comnet')
        })
    })
})
