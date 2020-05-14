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
}

module.exports = { Client }
