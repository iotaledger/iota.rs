## Classes

<dl>
<dt><a href="#Client">Client</a></dt>
<dd><p>The Client to interact with nodes.</p></dd>
<dt><a href="#MessageHandler">MessageHandler</a></dt>
<dd><p>The MessageHandler which sends the commands to the Rust side.</p></dd>
</dl>

## Members

<dl>
<dt><a href="#SHIMMER_TESTNET_BECH32_HRP">SHIMMER_TESTNET_BECH32_HRP</a></dt>
<dd><p>BIP44 Coin Types for IOTA and Shimmer.</p></dd>
<dt><a href="#utf8ToBytes">utf8ToBytes</a></dt>
<dd><p>Convert hex encoded string to UTF8 string</p></dd>
<dt><a href="#hexToUtf8">hexToUtf8</a></dt>
<dd><p>Convert UTF8 string to hex encoded string</p></dd>
</dl>

## Functions

<dl>
<dt><a href="#initLogger">initLogger()</a></dt>
<dd><p>Initialize logger, if no arguments are provided a default config will be used.</p></dd>
<dt><a href="#utf8ToBytes">utf8ToBytes()</a></dt>
<dd><p>Convert UTF8 string to an array of bytes</p></dd>
</dl>

<a name="Client"></a>

## Client
<p>The Client to interact with nodes.</p>

**Kind**: global class  

* [Client](#Client)
    * [.getInfo()](#Client+getInfo) ⇒ <code>Promise.&lt;INodeInfoWrapper&gt;</code>
    * [.getNetworkInfo()](#Client+getNetworkInfo)
    * [.basicOutputIds()](#Client+basicOutputIds)
    * [.getOutput()](#Client+getOutput)
    * [.getOutputs()](#Client+getOutputs)
    * [.generateMnemonic()](#Client+generateMnemonic)
    * [.mnemonicToHexSeed()](#Client+mnemonicToHexSeed)
    * [.generateAddresses()](#Client+generateAddresses)
    * [.generateBlock()](#Client+generateBlock)
    * [.getTips()](#Client+getTips)
    * [.postBlock()](#Client+postBlock)
    * [.getBlock()](#Client+getBlock)
    * [.getBlockMetadata()](#Client+getBlockMetadata)
    * [.findInputs()](#Client+findInputs)
    * [.findOutputs()](#Client+findOutputs)
    * [.prepareTransaction()](#Client+prepareTransaction)
    * [.storeMnemonic()](#Client+storeMnemonic)
    * [.signTransaction()](#Client+signTransaction)
    * [.submitPayload()](#Client+submitPayload)
    * [.parseBech32Address()](#Client+parseBech32Address)
    * [.blockId()](#Client+blockId)
    * [.getNode()](#Client+getNode)
    * [.getNetworkId()](#Client+getNetworkId)
    * [.getBech32Hrp()](#Client+getBech32Hrp)
    * [.getMinPowScore()](#Client+getMinPowScore)
    * [.getTipsInterval()](#Client+getTipsInterval)
    * [.getLocalPow()](#Client+getLocalPow)
    * [.getFallbackToLocalPow()](#Client+getFallbackToLocalPow)
    * [.getHealth()](#Client+getHealth)
    * [.getNodeInfo()](#Client+getNodeInfo)
    * [.getPeers()](#Client+getPeers)
    * [.postBlockRaw()](#Client+postBlockRaw)
    * [.getBlockRaw()](#Client+getBlockRaw)
    * [.getMilestoneById()](#Client+getMilestoneById)
    * [.getUtxoChangesById()](#Client+getUtxoChangesById)
    * [.getMilestoneByIndex()](#Client+getMilestoneByIndex)
    * [.getUtxoChangesByIndex()](#Client+getUtxoChangesByIndex)
    * [.getReceipts()](#Client+getReceipts)
    * [.getReceiptsMigratedAt()](#Client+getReceiptsMigratedAt)
    * [.getTreasury()](#Client+getTreasury)
    * [.getIncludedBlock()](#Client+getIncludedBlock)
    * [.bech32ToHex()](#Client+bech32ToHex)
    * [.hexToBech32()](#Client+hexToBech32)
    * [.hexPublicKeyToBech32Address()](#Client+hexPublicKeyToBech32Address)
    * [.isAddressValid()](#Client+isAddressValid)
    * [.aliasOutputIds()](#Client+aliasOutputIds)
    * [.aliasOutputId()](#Client+aliasOutputId)
    * [.nftOutputIds()](#Client+nftOutputIds)
    * [.nftOutputId()](#Client+nftOutputId)
    * [.foundryOutputIds()](#Client+foundryOutputIds)
    * [.foundryOutputId()](#Client+foundryOutputId)
    * [.tryGetOutputs()](#Client+tryGetOutputs)
    * [.findBlocks()](#Client+findBlocks)
    * [.retry()](#Client+retry)
    * [.retryUntilIncluded()](#Client+retryUntilIncluded)
    * [.consolidateFunds()](#Client+consolidateFunds)
    * [.reattach()](#Client+reattach)
    * [.reattachUnchecked()](#Client+reattachUnchecked)
    * [.promote()](#Client+promote)
    * [.promoteUnchecked()](#Client+promoteUnchecked)
    * [.unsyncedNodes()](#Client+unsyncedNodes)
    * [.buildBasicOutput()](#Client+buildBasicOutput)
    * [.buildAliasOutput()](#Client+buildAliasOutput)
    * [.buildFoundryOutput()](#Client+buildFoundryOutput)
    * [.buildNftOutput()](#Client+buildNftOutput)

<a name="Client+getInfo"></a>

### client.getInfo() ⇒ <code>Promise.&lt;INodeInfoWrapper&gt;</code>
<p>Returns the node information together with the url of the used node</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
**Returns**: <code>Promise.&lt;INodeInfoWrapper&gt;</code> - <p>.</p>  
<a name="Client+getNetworkInfo"></a>

### client.getNetworkInfo()
<p>Gets the network related information such as network_id and min_pow_score</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+basicOutputIds"></a>

### client.basicOutputIds()
<p>Fetch basic output IDs based on query parameters</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getOutput"></a>

### client.getOutput()
<p>Get output from a known outputID</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getOutputs"></a>

### client.getOutputs()
<p>Fetch OutputResponse from provided OutputIds (requests are sent in parallel)</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+generateMnemonic"></a>

### client.generateMnemonic()
<p>Generates a new mnemonic.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+mnemonicToHexSeed"></a>

### client.mnemonicToHexSeed()
<p>Returns a hex encoded seed for a mnemonic.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+generateAddresses"></a>

### client.generateAddresses()
<p>Generate addresses</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+generateBlock"></a>

### client.generateBlock()
<p>Generate client block</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getTips"></a>

### client.getTips()
<p>Returns tips that are ideal for attaching a block.
The tips can be considered as non-lazy and are therefore ideal for attaching a block.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+postBlock"></a>

### client.postBlock()
<p>Post block in JSON format, returns the block ID.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getBlock"></a>

### client.getBlock()
<p>Get block as JSON.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getBlockMetadata"></a>

### client.getBlockMetadata()
<p>Get block metadata.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+findInputs"></a>

### client.findInputs()
<p>Find inputs from addresses for a provided amount (useful for offline signing)</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+findOutputs"></a>

### client.findOutputs()
<p>Find all outputs based on the requests criteria. This method will try to query multiple nodes if
the request amount exceeds individual node limit.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+prepareTransaction"></a>

### client.prepareTransaction()
<p>Prepare a transaction for signing</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+storeMnemonic"></a>

### client.storeMnemonic()
<p>Store a mnemonic in the Stronghold vault</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+signTransaction"></a>

### client.signTransaction()
<p>Sign a transaction</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+submitPayload"></a>

### client.submitPayload()
<p>Submit a payload in a block</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+parseBech32Address"></a>

### client.parseBech32Address()
<p>Returns a valid Address parsed from a String.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+blockId"></a>

### client.blockId()
<p>Returns a block ID (Blake2b256 hash of the block bytes)</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getNode"></a>

### client.getNode()
<p>Get a node candidate from the synced node pool.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getNetworkId"></a>

### client.getNetworkId()
<p>Get the network id of the node we're connecting to.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getBech32Hrp"></a>

### client.getBech32Hrp()
<p>Returns the bech32_hrp.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getMinPowScore"></a>

### client.getMinPowScore()
<p>Returns the min PoW score.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getTipsInterval"></a>

### client.getTipsInterval()
<p>Returns the tips interval.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getLocalPow"></a>

### client.getLocalPow()
<p>Returns if local pow should be used or not.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getFallbackToLocalPow"></a>

### client.getFallbackToLocalPow()
<p>Get fallback to local proof of work timeout.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getHealth"></a>

### client.getHealth()
<p>Get health of node by input url.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getNodeInfo"></a>

### client.getNodeInfo()
<p>Get info of node with input url.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getPeers"></a>

### client.getPeers()
<p>Get peers.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+postBlockRaw"></a>

### client.postBlockRaw()
<p>Post block as raw bytes, returns the block ID.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getBlockRaw"></a>

### client.getBlockRaw()
<p>Get block as raw bytes.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getMilestoneById"></a>

### client.getMilestoneById()
<p>Look up a milestone by a given milestone index.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getUtxoChangesById"></a>

### client.getUtxoChangesById()
<p>Returns all UTXO changes that happened at a specific milestone.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getMilestoneByIndex"></a>

### client.getMilestoneByIndex()
<p>Look up a milestone by a given milestone index.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getUtxoChangesByIndex"></a>

### client.getUtxoChangesByIndex()
<p>Returns all UTXO changes that happened at a specific milestone.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getReceipts"></a>

### client.getReceipts()
<p>Get receipts.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getReceiptsMigratedAt"></a>

### client.getReceiptsMigratedAt()
<p>Get the receipts by the given milestone index.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getTreasury"></a>

### client.getTreasury()
<p>Get the treasury output.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getIncludedBlock"></a>

### client.getIncludedBlock()
<p>Returns the included block of the transaction.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+bech32ToHex"></a>

### client.bech32ToHex()
<p>Transforms bech32 to hex.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+hexToBech32"></a>

### client.hexToBech32()
<p>Transforms a hex encoded address to a bech32 encoded address.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+hexPublicKeyToBech32Address"></a>

### client.hexPublicKeyToBech32Address()
<p>Transforms a hex encoded public key to a bech32 encoded address.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+isAddressValid"></a>

### client.isAddressValid()
<p>Checks if a String is a valid bech32 encoded address.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+aliasOutputIds"></a>

### client.aliasOutputIds()
<p>Fetch alias output IDs</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+aliasOutputId"></a>

### client.aliasOutputId()
<p>Fetch alias output ID</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+nftOutputIds"></a>

### client.nftOutputIds()
<p>Fetch NFT output IDs</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+nftOutputId"></a>

### client.nftOutputId()
<p>Fetch NFT output ID</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+foundryOutputIds"></a>

### client.foundryOutputIds()
<p>Fetch Foundry Output IDs</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+foundryOutputId"></a>

### client.foundryOutputId()
<p>Fetch Foundry Output ID</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+tryGetOutputs"></a>

### client.tryGetOutputs()
<p>Try to get OutputResponse from provided OutputIds (requests are sent
in parallel and errors are ignored, can be useful for spent outputs)</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+findBlocks"></a>

### client.findBlocks()
<p>Find all blocks by provided block IDs.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+retry"></a>

### client.retry()
<p>Retries (promotes or reattaches) a block for provided block id. Block should be
retried only if they are valid and haven't been confirmed for a while.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+retryUntilIncluded"></a>

### client.retryUntilIncluded()
<p>Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
position and additional reattached blocks</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+consolidateFunds"></a>

### client.consolidateFunds()
<p>Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
Returns the address to which the funds got consolidated, if any were available</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+reattach"></a>

### client.reattach()
<p>Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
confirmed for a while.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+reattachUnchecked"></a>

### client.reattachUnchecked()
<p>Reattach a block without checking if it should be reattached</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+promote"></a>

### client.promote()
<p>Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
method should error out and should not allow unnecessary promotions.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+promoteUnchecked"></a>

### client.promoteUnchecked()
<p>Promote a block without checking if it should be promoted</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+unsyncedNodes"></a>

### client.unsyncedNodes()
<p>Returns the unsynced nodes.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+buildBasicOutput"></a>

### client.buildBasicOutput()
<p>Build a Basic Output.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+buildAliasOutput"></a>

### client.buildAliasOutput()
<p>Build an Alias Output.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+buildFoundryOutput"></a>

### client.buildFoundryOutput()
<p>Build a Foundry Output.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+buildNftOutput"></a>

### client.buildNftOutput()
<p>Build an Nft Output.</p>

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="MessageHandler"></a>

## MessageHandler
<p>The MessageHandler which sends the commands to the Rust side.</p>

**Kind**: global class  
<a name="SHIMMER_TESTNET_BECH32_HRP"></a>

## SHIMMER\_TESTNET\_BECH32\_HRP
<p>BIP44 Coin Types for IOTA and Shimmer.</p>

**Kind**: global variable  
<a name="utf8ToBytes"></a>

## utf8ToBytes
<p>Convert hex encoded string to UTF8 string</p>

**Kind**: global variable  
<a name="hexToUtf8"></a>

## hexToUtf8
<p>Convert UTF8 string to hex encoded string</p>

**Kind**: global variable  
<a name="initLogger"></a>

## initLogger()
<p>Initialize logger, if no arguments are provided a default config will be used.</p>

**Kind**: global function  
<a name="utf8ToBytes"></a>

## utf8ToBytes()
<p>Convert UTF8 string to an array of bytes</p>

**Kind**: global function  
