import initWasm, { Client as WasmClient } from './wasm-web/iota_wasm'
let __initializedIotaWasm = false

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
        if (__initializedIotaWasm) {
            return Promise.resolve(new WasmClient(this.uri))
        }
        return initWasm('iota_client.wasm').then(() => {
            __initializedIotaWasm = true
            return new WasmClient(this.uri)
        })
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

export { Client }
