import initWasm, * as wasm from './wasm-web/iota_wasm'
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
        this.__index = index
        return this
    }

    security(security) {
        this.__security = security
        return this
    }
}

function __getClient() {
    if (__initializedIotaWasm) {
        return Promise.resolve(wasm)
    }
    return initWasm('iota_client.wasm').then(() => {
        __initializedIotaWasm = true
        return wasm
    })
}

/**
 * @param {String} uri URI of IRI connection
 */
function addNode(uri) {
    return __getClient().then(client => client.addNode(uri))
}

/**
 * gets the node info
 */
function getNodeInfo() {
    return __getClient().then(client => client.getNodeInfo())
}

/**
 * generates a new address and validates it on the IRI node
 * @param {String} seed
 */
function getNewAddress(seed) {
    return new AddressGenerator((seed, index, security) => {
        return __getClient().then(client => client.getNewAddress(seed, index, security))
    }, seed)
}

/**
 * Add a list of neighbors to your node. It should be noted that
 * this is only temporary, and the added neighbors will be removed
 * from your set of neighbors after you relaunch IRI.
 * 
 * @param {String|String[]} uris tcp:// or udp:// URIs to add
 */
function addNeighbors(uris) {
    if (typeof uris === 'string') {
        uris = [uris]
    } else if (!Array.isArray(uris)) {
        return Promise.reject(new Error('uris must be an array'))
    } else if (uris.some(uri => typeof uri !== 'string')) {
        return Promise.reject(new Error('Every uri must be a string'))
    }

    return __getClient().then(client => client.addNeighbors(uris))
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
function attachToTangle(trunkTransactionHash, branchTransactionHash, minWeightMagnitude = null, transactions = null) {
    return __getClient()
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
function broadcastBundle(tailTransactionHash) {
    return __getClient().then(client => client.broadcastBundle(tailTransactionHash))
}

/**
 * Sends transaction trytes to a node.
 * The input trytes for this call are provided by `attach_to_tangle`.
 * Response only contains errors and exceptions, it would be `None` if the call success.
 * 
 * @param {String|String[]} trytes Transaction trytes
 */
function broadcastTransactions(trytes) {
    if (typeof trytes === 'string') {
        trytes = [trytes]
    } else if (!Array.isArray(trytes)) {
        return Promise.reject(new Error('uris must be an array'))
    } else if (trytes.some(tryte => typeof tryte !== 'string')) {
        return Promise.reject(new Error('Every uri must be a string'))
    }

    return __getClient().then(client => client.broadcastTransactions(trytes))
}

/**
 * Checks the consistency of transactions. A consistent transaction is one where the following statements are true:
 * The node isn't missing the transaction's branch or trunk transactions
 * The transaction's bundle is valid
 * The transaction's branch and trunk transactions are valid
 * 
 * @param {Hash[]} tailTransactionHashes
 */
function checkConsistency(tailTransactionHashes) {
    return __getClient().then(client => client.checkConsistency(tailTransactionHashes))
}

/**
 * Calls PrepareTransfers and then sends off the bundle via SendTrytes.
 * @param {String} seed
 * @param {Object[]} transfers
 * @param {Number} [transfers.value]
 * @param {Number} [minWeightMagnitude]
 */
function sendTransfers(seed, transfers, minWeightMagnitude = null) {
    return __getClient().then(client => client.sendTransfers(seed, transfers, minWeightMagnitude))
}

/**
 * Store and broadcast transactions to the node.
 * The input trytes for this call are provided by `attach_to_tangle`.
 * Response only contains errors and exceptions, it would be `None` if the call success.
 * 
 * @param {String|String[]} trytes Transaction trytes
 */
function storeAndBroadCast(trytes) {
    if (typeof trytes === 'string') {
        trytes = [trytes]
    } else if (!Array.isArray(trytes)) {
        return Promise.reject(new Error('uris must be an array'))
    } else if (trytes.some(tryte => typeof tryte !== 'string')) {
        return Promise.reject(new Error('Every uri must be a string'))
    }

    return __getClient().then(client => client.storeAndBroadCast(trytes))
}

/**
 * Store transactions into the local storage.
 * The input trytes for this call are provided by `attach_to_tangle`.
 * Response only contains errors and exceptions, it would be `None` if the call success.
 * 
 * @param {String|String[]} trytes Transaction trytes
 */
function storeTransactions(trytes) {
    if (typeof trytes === 'string') {
        trytes = [trytes]
    } else if (!Array.isArray(trytes)) {
        return Promise.reject(new Error('uris must be an array'))
    } else if (trytes.some(tryte => typeof tryte !== 'string')) {
        return Promise.reject(new Error('Every uri must be a string'))
    }

    return __getClient().then(client => client.storeTransactions(trytes))
}

/**
 * Fetches the bundle of a given the tail transaction hash, by traversing through trunk transaction.
 * It does not validate the bundle. Use [`get_bundle`] instead to get validated bundle.
 *
 * @param {Hash} tailTransactionHash Tail transaction hash (current_index == 0)
 */
function traverseBundle(tailTransactionHash) {
    return __getClient().then(client => client.traverseBundle(tailTransactionHash))
}

/**
 * Finds the transactions with the specified filters
 *
 * @param {Hash[]} bundleHashes bundle hashes
 * @param {String[]} tags transaction tags
 * @param {Hash[]} approveesHashes transaction approvees hashes
 * @param {String[]} addresses transaction addresses
 */
function findTransactions(bundleHashes, tags, approveesHashes, addresses) {
    return __getClient().then(client => client.findTransactions(bundleHashes, tags, approveesHashes, addresses))
}

/**
 * Gets the trytes of the transaction with the given hashes
 *
 * @param {Hash[]} transactionHashes the transaction hashes
 *
 * @return the transaction trytes
 */
function getTrytes(transactionHashes) {
    return __getClient().then(client => client.getTrytes(transactionHashes))
}

export {
    addNode,
    getNodeInfo,
    getNewAddress,
    addNeighbors,
    attachToTangle,
    broadcastBundle,
    broadcastTransactions,
    checkConsistency,
    sendTransfers,
    storeAndBroadcast,
    storeTransactions,
    traverseBundle,
    findTransactions,
    getTrytes
}