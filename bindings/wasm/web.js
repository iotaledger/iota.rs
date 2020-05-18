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
    /**
     * @constructor
     * @param {String} uri URI of IRI connection
     */
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

    /**
     * gets the node info
     */
    getNodeInfo() {
        return this.__getClient().then(client => client.getNodeInfo())
    }

    /**
     * generates a new address and validates it on the IRI node
     * @param {String} seed
     */
    getNewAddress(seed) {
        return new AddressGenerator((seed, index, security) => {
            return this.__getClient().then(client => client.getNewAddress(seed, index, security))
        }, seed)
    }

    /**
     * Add a list of neighbors to your node. It should be noted that
     * this is only temporary, and the added neighbors will be removed
     * from your set of neighbors after you relaunch IRI.
     * 
     * @param {String|String[]} uris tcp:// or udp:// URIs to add
     */
    addNeighbors(uris) {
        if (typeof uris === 'string') {
            uris = [uris]
        } else if (!Array.isArray(uris)) {
            return Promise.reject(new Error('uris must be an array'))
        } else if (uris.some(uri => typeof uri !== 'string')) {
            return Promise.reject(new Error('Every uri must be a string'))
        }

        return this.__getClient().then(client => client.addNeighbors(uris))
    }

    /**
     * Does proof of work for the given transaction trytes.
     * The `branch_transaction` and `trunk_transaction` parameters are returned
     * from the `getTransactionsToApprove` method.
     * 
     * @param {Hash} trunkTransactionHash
     * @param {Hash} branchTransactionHash
     * @param {Array<byte[]>} transactionTrytes
     * @param {Number} [minWeightMagnitude]
     */
    attachToTangle(trunkTransactionHash, branchTransactionHash, minWeightMagnitude = null, transactions = null) {
        return this.__getClient()
            .then(
                client => client.attachToTangle(trunkTransactionHash, branchTransactionHash, minWeightMagnitude, transactions)
            )
    }

    /**
     * Re-broadcasts all transactions in a bundle given the tail transaction hash. It might be useful
     * when transactions did not properly propagate, particularly in the case of large bundles.
     * 
     * @param {Hash} tailTransactionHash
     */
    broadcastBundle(tailTransactionHash) {
        return this.__getClient().then(client => client.broadcastBundle(tailTransactionHash))
    }

    /**
     * Checks the consistency of transactions. A consistent transaction is one where the following statements are true:
     * The node isn't missing the transaction's branch or trunk transactions
     * The transaction's bundle is valid
     * The transaction's branch and trunk transactions are valid
     * 
     * @param {Hash[]} tailTransactionHashes
     */
    checkConsistency(tailTransactionHashes) {
        return this.__getClient().then(client => client.checkConsistency(tailTransactionHashes))
    }

    /**
     * Calls PrepareTransfers and then sends off the bundle via SendTrytes.
     * @param {String} seed
     * @param {Object[]} transfers
     * @param {Number} [transfers.value]
     * @param {Number} [minWeightMagnitude]
     */
    sendTransfers(seed, transfers, minWeightMagnitude = null) {
        return this.__getClient().then(client => client.sendTransfers(seed, transfers, minWeightMagnitude))
    }
}

export { Client }
