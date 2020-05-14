const { Client: WasmClient } = require('./wasm-node/iota_wasm')

const fetch = require('node-fetch')
global.Headers = fetch.Headers
global.Request = fetch.Request
global.Response = fetch.Response
global.Window = Object
global.fetch = fetch

class AddressGenerator {
    constructor(generator, seed) {
        this.generator = generator
        this.__seed = seed
    }

    generate() {
        return this.generator(this.__seed, this.__index, this.__security)
    }

    index(index) {
        this.__index = BigInt(index)
        return this
    }

    security(security) {
        this.__security = security
        return this
    }
}

class Client {
    constructor(uri) {
        this.uri = uri
    }

    __getClient() {
        return Promise.resolve(new WasmClient(this.uri))
    }

    getNodeInfo() {
        return this.__getClient().then(client => client.getNodeInfo())
    }

    getNewAddress(seed) {
        return new AddressGenerator((seed, index, security) => {
            return this.__getClient().then(client => client.getNewAddress(seed, index, security))
        }, seed)
    }

    addNeighbors(uris) {
        if (typeof uris === 'string') {
            uris = [uris]
        } else if (!Array.isArray(uris)) {
            throw new Error('uris must be an array')
        } else if (uris.some(uri => typeof uri !== 'string')) {
            throw new Error('Every uri must be a string')
        }

        return this.__getClient().then(client => client.addNeighbors(uris))
    }

    sendTransfers(seed, transfers, minWeightMagnitude = null) {
        return this.__getClient().then(client => client.sendTransfers(seed, transfers, minWeightMagnitude))
    }
}

module.exports = { Client }
