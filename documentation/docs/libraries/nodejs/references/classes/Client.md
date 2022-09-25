# Class: Client

The Client to interact with nodes.

## Table of contents

### Methods

- [getInfo](Client.md#getinfo)
- [getNetworkInfo](Client.md#getnetworkinfo)
- [basicOutputIds](Client.md#basicoutputids)
- [getOutput](Client.md#getoutput)
- [getOutputs](Client.md#getoutputs)
- [generateMnemonic](Client.md#generatemnemonic)
- [mnemonicToHexSeed](Client.md#mnemonictohexseed)
- [generateAddresses](Client.md#generateaddresses)
- [buildAndPostBlock](Client.md#buildandpostblock)
- [getTips](Client.md#gettips)
- [postBlock](Client.md#postblock)
- [getBlock](Client.md#getblock)
- [getBlockMetadata](Client.md#getblockmetadata)
- [findInputs](Client.md#findinputs)
- [findOutputs](Client.md#findoutputs)
- [getLedgerNanoStatus](Client.md#getledgernanostatus)
- [prepareTransaction](Client.md#preparetransaction)
- [storeMnemonic](Client.md#storemnemonic)
- [signTransaction](Client.md#signtransaction)
- [postblockpayload](Client.md#postblockpayload)
- [parseBech32Address](Client.md#parsebech32address)
- [blockId](Client.md#blockid)
- [getNode](Client.md#getnode)
- [getNetworkId](Client.md#getnetworkid)
- [getBech32Hrp](Client.md#getbech32hrp)
- [getMinPowScore](Client.md#getminpowscore)
- [getTipsInterval](Client.md#gettipsinterval)
- [getLocalPow](Client.md#getlocalpow)
- [getFallbackToLocalPow](Client.md#getfallbacktolocalpow)
- [getHealth](Client.md#gethealth)
- [getNodeInfo](Client.md#getnodeinfo)
- [getPeers](Client.md#getpeers)
- [postBlockRaw](Client.md#postblockraw)
- [getBlockRaw](Client.md#getblockraw)
- [getMilestoneById](Client.md#getmilestonebyid)
- [getUtxoChangesById](Client.md#getutxochangesbyid)
- [getMilestoneByIndex](Client.md#getmilestonebyindex)
- [getUtxoChangesByIndex](Client.md#getutxochangesbyindex)
- [getReceipts](Client.md#getreceipts)
- [getReceiptsMigratedAt](Client.md#getreceiptsmigratedat)
- [getTreasury](Client.md#gettreasury)
- [getIncludedBlock](Client.md#getincludedblock)
- [bech32ToHex](Client.md#bech32tohex)
- [hexToBech32](Client.md#hextobech32)
- [hexPublicKeyToBech32Address](Client.md#hexpublickeytobech32address)
- [isAddressValid](Client.md#isaddressvalid)
- [aliasOutputIds](Client.md#aliasoutputids)
- [aliasOutputId](Client.md#aliasoutputid)
- [nftOutputIds](Client.md#nftoutputids)
- [nftOutputId](Client.md#nftoutputid)
- [foundryOutputIds](Client.md#foundryoutputids)
- [foundryOutputId](Client.md#foundryoutputid)
- [tryGetOutputs](Client.md#trygetoutputs)
- [findBlocks](Client.md#findblocks)
- [retry](Client.md#retry)
- [retryUntilIncluded](Client.md#retryuntilincluded)
- [consolidateFunds](Client.md#consolidatefunds)
- [reattach](Client.md#reattach)
- [reattachUnchecked](Client.md#reattachunchecked)
- [promote](Client.md#promote)
- [promoteUnchecked](Client.md#promoteunchecked)
- [unhealthyNodes](Client.md#unhealthynodes)
- [buildBasicOutput](Client.md#buildbasicoutput)
- [buildAliasOutput](Client.md#buildaliasoutput)
- [buildFoundryOutput](Client.md#buildfoundryoutput)
- [buildNftOutput](Client.md#buildnftoutput)

## Methods

### getInfo

▸ **getInfo**(): `Promise`<[`INodeInfoWrapper`](../interfaces/INodeInfoWrapper.md)\>

Returns the node information together with the url of the used node

#### Returns

`Promise`<[`INodeInfoWrapper`](../interfaces/INodeInfoWrapper.md)\>

.

___

### getNetworkInfo

▸ **getNetworkInfo**(): `Promise`<[`INetworkInfo`](../interfaces/INetworkInfo.md)\>

Gets the network related information such as network_id and min_pow_score

#### Returns

`Promise`<[`INetworkInfo`](../interfaces/INetworkInfo.md)\>

___

### basicOutputIds

▸ **basicOutputIds**(`queryParameters`): `Promise`<`string`[]\>

Fetch basic output IDs based on query parameters

#### Parameters

| Name | Type |
| :------ | :------ |
| `queryParameters` | [`QueryParameter`](../api_ref.md#queryparameter)[] |

#### Returns

`Promise`<`string`[]\>

___

### getOutput

▸ **getOutput**(`outputId`): `Promise`<`IOutputResponse`\>

Get output from a known outputID

#### Parameters

| Name | Type |
| :------ | :------ |
| `outputId` | `string` |

#### Returns

`Promise`<`IOutputResponse`\>

___

### getOutputs

▸ **getOutputs**(`outputIds`): `Promise`<`IOutputResponse`[]\>

Fetch OutputResponse from provided OutputIds (requests are sent in parallel)

#### Parameters

| Name | Type |
| :------ | :------ |
| `outputIds` | `string`[] |

#### Returns

`Promise`<`IOutputResponse`[]\>

___

### generateMnemonic

▸ **generateMnemonic**(): `Promise`<`string`\>

Generates a new mnemonic.

#### Returns

`Promise`<`string`\>

___

### mnemonicToHexSeed

▸ **mnemonicToHexSeed**(`mnemonic`): `Promise`<`string`\>

Returns a hex encoded seed for a mnemonic.

#### Parameters

| Name | Type |
| :------ | :------ |
| `mnemonic` | `string` |

#### Returns

`Promise`<`string`\>

___

### generateAddresses

▸ **generateAddresses**(`secretManager`, `generateAddressesOptions`): `Promise`<`string`[]\>

Generate addresses

#### Parameters

| Name | Type |
| :------ | :------ |
| `secretManager` | [`SecretManager`](../api_ref.md#secretmanager) |
| `generateAddressesOptions` | [`IGenerateAddressesOptions`](../interfaces/IGenerateAddressesOptions.md) |

#### Returns

`Promise`<`string`[]\>

___

### buildAndPostBlock

▸ **buildAndPostBlock**(`secretManager?`, `options?`): `Promise`<[`string`, `IBlock`]\>

Build and post a block

#### Parameters

| Name | Type |
| :------ | :------ |
| `secretManager?` | [`SecretManager`](../api_ref.md#secretmanager) |
| `options?` | [`IBuildBlockOptions`](../interfaces/IBuildBlockOptions.md) |

#### Returns

`Promise`<[`string`, `IBlock`]\>

___

### getTips

▸ **getTips**(): `Promise`<`string`[]\>

Returns tips that are ideal for attaching a block.
The tips can be considered as non-lazy and are therefore ideal for attaching a block.

#### Returns

`Promise`<`string`[]\>

___

### postBlock

▸ **postBlock**(`block`): `Promise`<`string`\>

Post block in JSON format, returns the block ID.

#### Parameters

| Name | Type |
| :------ | :------ |
| `block` | `IBlock` |

#### Returns

`Promise`<`string`\>

___

### getBlock

▸ **getBlock**(`blockId`): `Promise`<`IBlock`\>

Get block as JSON.

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |

#### Returns

`Promise`<`IBlock`\>

___

### getBlockMetadata

▸ **getBlockMetadata**(`blockId`): `Promise`<`IBlockMetadata`\>

Get block metadata.

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |

#### Returns

`Promise`<`IBlockMetadata`\>

___

### findInputs

▸ **findInputs**(`addresses`, `amount`): `Promise`<`IUTXOInput`[]\>

Find inputs from addresses for a provided amount (useful for offline signing)

#### Parameters

| Name | Type |
| :------ | :------ |
| `addresses` | `string`[] |
| `amount` | `number` |

#### Returns

`Promise`<`IUTXOInput`[]\>

___

### findOutputs

▸ **findOutputs**(`outputIds`, `addresses`): `Promise`<`IOutputResponse`[]\>

Find all outputs based on the requests criteria. This method will try to query multiple nodes if
the request amount exceeds individual node limit.

#### Parameters

| Name | Type |
| :------ | :------ |
| `outputIds` | `string`[] |
| `addresses` | `string`[] |

#### Returns

`Promise`<`IOutputResponse`[]\>

___

### getLedgerNanoStatus

▸ **getLedgerNanoStatus**(`isSimulator`): `Promise`<[`LedgerNanoStatus`](../interfaces/LedgerNanoStatus.md)\>

Get the status of a Ledger Nano

#### Parameters

| Name | Type |
| :------ | :------ |
| `isSimulator` | `boolean` |

#### Returns

`Promise`<[`LedgerNanoStatus`](../interfaces/LedgerNanoStatus.md)\>

___

### prepareTransaction

▸ **prepareTransaction**(`secretManager?`, `options?`): `Promise`<[`IPreparedTransactionData`](../interfaces/IPreparedTransactionData.md)\>

Prepare a transaction for signing

#### Parameters

| Name | Type |
| :------ | :------ |
| `secretManager?` | [`SecretManager`](../api_ref.md#secretmanager) |
| `options?` | [`IBuildBlockOptions`](../interfaces/IBuildBlockOptions.md) |

#### Returns

`Promise`<[`IPreparedTransactionData`](../interfaces/IPreparedTransactionData.md)\>

___

### storeMnemonic

▸ **storeMnemonic**(`secretManager`, `mnemonic`): `Promise`<`void`\>

Store a mnemonic in the Stronghold vault

#### Parameters

| Name | Type |
| :------ | :------ |
| `secretManager` | [`SecretManager`](../api_ref.md#secretmanager) |
| `mnemonic` | `string` |

#### Returns

`Promise`<`void`\>

___

### signTransaction

▸ **signTransaction**(`secretManager`, `preparedTransactionData`): `Promise`<`PayloadTypes`\>

Sign a transaction

#### Parameters

| Name | Type |
| :------ | :------ |
| `secretManager` | [`SecretManager`](../api_ref.md#secretmanager) |
| `preparedTransactionData` | [`IPreparedTransactionData`](../interfaces/IPreparedTransactionData.md) |

#### Returns

`Promise`<`PayloadTypes`\>

___

### postBlockPayload

▸ **postBlockPayload**(`payload`): `Promise`<`IBlock`\>

Submit a payload in a block

#### Parameters

| Name | Type |
| :------ | :------ |
| `payload` | `PayloadTypes` |

#### Returns

`Promise`<`IBlock`\>

___

### parseBech32Address

▸ **parseBech32Address**(`address`): `Promise`<`AddressTypes`\>

Returns a valid Address parsed from a String.

#### Parameters

| Name | Type |
| :------ | :------ |
| `address` | `string` |

#### Returns

`Promise`<`AddressTypes`\>

___

### blockId

▸ **blockId**(`block`): `Promise`<`string`\>

Returns a block ID (Blake2b256 hash of the block bytes)

#### Parameters

| Name | Type |
| :------ | :------ |
| `block` | `IBlock` |

#### Returns

`Promise`<`string`\>

___

### getNode

▸ **getNode**(): `Promise`<[`INode`](../interfaces/INode.md)\>

Get a node candidate from the healthy node pool.

#### Returns

`Promise`<[`INode`](../interfaces/INode.md)\>

___

### getNetworkId

▸ **getNetworkId**(): `Promise`<`number`\>

Get the network id of the node we're connecting to.

#### Returns

`Promise`<`number`\>

___

### getBech32Hrp

▸ **getBech32Hrp**(): `Promise`<`string`\>

Returns the bech32_hrp.

#### Returns

`Promise`<`string`\>

___

### getMinPowScore

▸ **getMinPowScore**(): `Promise`<`number`\>

Returns the min PoW score.

#### Returns

`Promise`<`number`\>

___

### getTipsInterval

▸ **getTipsInterval**(): `Promise`<`number`\>

Returns the tips interval.

#### Returns

`Promise`<`number`\>

___

### getLocalPow

▸ **getLocalPow**(): `Promise`<`boolean`\>

Returns if local pow should be used or not.

#### Returns

`Promise`<`boolean`\>

___

### getFallbackToLocalPow

▸ **getFallbackToLocalPow**(): `Promise`<`boolean`\>

Get fallback to local proof of work timeout.

#### Returns

`Promise`<`boolean`\>

___

### getHealth

▸ **getHealth**(`url`): `Promise`<`boolean`\>

Get health of node by input url.

#### Parameters

| Name | Type |
| :------ | :------ |
| `url` | `string` |

#### Returns

`Promise`<`boolean`\>

___

### getNodeInfo

▸ **getNodeInfo**(`url`, `auth?`): `Promise`<`INodeInfo`\>

Get info of node with input url.

#### Parameters

| Name | Type |
| :------ | :------ |
| `url` | `string` |
| `auth?` | [`IAuth`](../interfaces/IAuth.md) |

#### Returns

`Promise`<`INodeInfo`\>

___

### getPeers

▸ **getPeers**(): `Promise`<`IPeer`[]\>

Get peers.

#### Returns

`Promise`<`IPeer`[]\>

___

### postBlockRaw

▸ **postBlockRaw**(`block`): `Promise`<`string`\>

Post block as raw bytes, returns the block ID.

#### Parameters

| Name | Type |
| :------ | :------ |
| `block` | `IBlock` |

#### Returns

`Promise`<`string`\>

___

### getBlockRaw

▸ **getBlockRaw**(`blockId`): `Promise`<`Uint8Array`\>

Get block as raw bytes.

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |

#### Returns

`Promise`<`Uint8Array`\>

___

### getMilestoneById

▸ **getMilestoneById**(`milestoneId`): `Promise`<`IMilestonePayload`\>

Look up a milestone by a given milestone index.

#### Parameters

| Name | Type |
| :------ | :------ |
| `milestoneId` | `string` |

#### Returns

`Promise`<`IMilestonePayload`\>

___

### getUtxoChangesById

▸ **getUtxoChangesById**(`milestoneId`): `Promise`<`IMilestoneUtxoChangesResponse`\>

Returns all UTXO changes that happened at a specific milestone.

#### Parameters

| Name | Type |
| :------ | :------ |
| `milestoneId` | `string` |

#### Returns

`Promise`<`IMilestoneUtxoChangesResponse`\>

___

### getMilestoneByIndex

▸ **getMilestoneByIndex**(`index`): `Promise`<`IMilestonePayload`\>

Look up a milestone by a given milestone index.

#### Parameters

| Name | Type |
| :------ | :------ |
| `index` | `number` |

#### Returns

`Promise`<`IMilestonePayload`\>

___

### getUtxoChangesByIndex

▸ **getUtxoChangesByIndex**(`index`): `Promise`<`IMilestoneUtxoChangesResponse`\>

Returns all UTXO changes that happened at a specific milestone.

#### Parameters

| Name | Type |
| :------ | :------ |
| `index` | `number` |

#### Returns

`Promise`<`IMilestoneUtxoChangesResponse`\>

___

### getReceipts

▸ **getReceipts**(): `Promise`<`IReceiptsResponse`\>

Get receipts.

#### Returns

`Promise`<`IReceiptsResponse`\>

___

### getReceiptsMigratedAt

▸ **getReceiptsMigratedAt**(`milestoneIndex`): `Promise`<`IReceiptsResponse`[]\>

Get the receipts by the given milestone index.

#### Parameters

| Name | Type |
| :------ | :------ |
| `milestoneIndex` | `number` |

#### Returns

`Promise`<`IReceiptsResponse`[]\>

___

### getTreasury

▸ **getTreasury**(): `Promise`<`ITreasury`\>

Get the treasury output.

#### Returns

`Promise`<`ITreasury`\>

___

### getIncludedBlock

▸ **getIncludedBlock**(`transactionId`): `Promise`<`IBlock`\>

Returns the included block of the transaction.

#### Parameters

| Name | Type |
| :------ | :------ |
| `transactionId` | `string` |

#### Returns

`Promise`<`IBlock`\>

___

### bech32ToHex

▸ **bech32ToHex**(`bech32`): `Promise`<`string`\>

Transforms bech32 to hex.

#### Parameters

| Name | Type |
| :------ | :------ |
| `bech32` | `string` |

#### Returns

`Promise`<`string`\>

___

### hexToBech32

▸ **hexToBech32**(`hex`, `bech32Hrp?`): `Promise`<`string`\>

Transforms a hex encoded address to a bech32 encoded address.

#### Parameters

| Name | Type |
| :------ | :------ |
| `hex` | `string` |
| `bech32Hrp?` | `string` |

#### Returns

`Promise`<`string`\>

___

### hexPublicKeyToBech32Address

▸ **hexPublicKeyToBech32Address**(`hex`, `bech32Hrp?`): `Promise`<`string`\>

Transforms a hex encoded public key to a bech32 encoded address.

#### Parameters

| Name | Type |
| :------ | :------ |
| `hex` | `string` |
| `bech32Hrp?` | `string` |

#### Returns

`Promise`<`string`\>

___

### isAddressValid

▸ **isAddressValid**(`address`): `Promise`<`boolean`\>

Checks if a String is a valid bech32 encoded address.

#### Parameters

| Name | Type |
| :------ | :------ |
| `address` | `string` |

#### Returns

`Promise`<`boolean`\>

___

### aliasOutputIds

▸ **aliasOutputIds**(`queryParameters`): `Promise`<`string`[]\>

Fetch alias output IDs

#### Parameters

| Name | Type |
| :------ | :------ |
| `queryParameters` | [`AliasQueryParameter`](../api_ref.md#aliasqueryparameter)[] |

#### Returns

`Promise`<`string`[]\>

___

### aliasOutputId

▸ **aliasOutputId**(`aliasId`): `Promise`<`string`\>

Fetch alias output ID

#### Parameters

| Name | Type |
| :------ | :------ |
| `aliasId` | `string` |

#### Returns

`Promise`<`string`\>

___

### nftOutputIds

▸ **nftOutputIds**(`queryParameters`): `Promise`<`string`[]\>

Fetch NFT output IDs

#### Parameters

| Name | Type |
| :------ | :------ |
| `queryParameters` | [`NftQueryParameter`](../api_ref.md#nftqueryparameter)[] |

#### Returns

`Promise`<`string`[]\>

___

### nftOutputId

▸ **nftOutputId**(`nftId`): `Promise`<`string`\>

Fetch NFT output ID

#### Parameters

| Name | Type |
| :------ | :------ |
| `nftId` | `string` |

#### Returns

`Promise`<`string`\>

___

### foundryOutputIds

▸ **foundryOutputIds**(`queryParameters`): `Promise`<`string`[]\>

Fetch Foundry Output IDs

#### Parameters

| Name | Type |
| :------ | :------ |
| `queryParameters` | [`FoundryQueryParameter`](../api_ref.md#foundryqueryparameter)[] |

#### Returns

`Promise`<`string`[]\>

___

### foundryOutputId

▸ **foundryOutputId**(`foundryId`): `Promise`<`string`\>

Fetch Foundry Output ID

#### Parameters

| Name | Type |
| :------ | :------ |
| `foundryId` | `string` |

#### Returns

`Promise`<`string`\>

___

### tryGetOutputs

▸ **tryGetOutputs**(`outputIds`): `Promise`<`IOutputResponse`[]\>

Try to get OutputResponse from provided OutputIds (requests are sent
in parallel and errors are ignored, can be useful for spent outputs)

#### Parameters

| Name | Type |
| :------ | :------ |
| `outputIds` | `string`[] |

#### Returns

`Promise`<`IOutputResponse`[]\>

___

### findBlocks

▸ **findBlocks**(`blockIds`): `Promise`<`IBlock`[]\>

Find all blocks by provided block IDs.

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockIds` | `string`[] |

#### Returns

`Promise`<`IBlock`[]\>

___

### retry

▸ **retry**(`blockId`): `Promise`<[`string`, `IBlock`]\>

Retries (promotes or reattaches) a block for provided block id. Block should be
retried only if they are valid and haven't been confirmed for a while.

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |

#### Returns

`Promise`<[`string`, `IBlock`]\>

___

### retryUntilIncluded

▸ **retryUntilIncluded**(`blockId`, `interval?`, `maxAttempts?`): `Promise`<[`string`, `IBlock`][]\>

Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
position and additional reattached blocks

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |
| `interval?` | `number` |
| `maxAttempts?` | `number` |

#### Returns

`Promise`<[`string`, `IBlock`][]\>

___

### consolidateFunds

▸ **consolidateFunds**(`secretManager`, `generateAddressesOptions`): `Promise`<`string`\>

Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
Returns the address to which the funds got consolidated, if any were available

#### Parameters

| Name | Type |
| :------ | :------ |
| `secretManager` | [`SecretManager`](../api_ref.md#secretmanager) |
| `generateAddressesOptions` | [`IGenerateAddressesOptions`](../interfaces/IGenerateAddressesOptions.md) |

#### Returns

`Promise`<`string`\>

___

### reattach

▸ **reattach**(`blockId`): `Promise`<[`string`, `IBlock`]\>

Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
confirmed for a while.

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |

#### Returns

`Promise`<[`string`, `IBlock`]\>

___

### reattachUnchecked

▸ **reattachUnchecked**(`blockId`): `Promise`<[`string`, `IBlock`]\>

Reattach a block without checking if it should be reattached

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |

#### Returns

`Promise`<[`string`, `IBlock`]\>

___

### promote

▸ **promote**(`blockId`): `Promise`<[`string`, `IBlock`]\>

Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
method should error out and should not allow unnecessary promotions.

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |

#### Returns

`Promise`<[`string`, `IBlock`]\>

___

### promoteUnchecked

▸ **promoteUnchecked**(`blockId`): `Promise`<[`string`, `IBlock`]\>

Promote a block without checking if it should be promoted

#### Parameters

| Name | Type |
| :------ | :------ |
| `blockId` | `string` |

#### Returns

`Promise`<[`string`, `IBlock`]\>

___

### unhealthyNodes

▸ **unhealthyNodes**(): `Promise`<`Set`<[`INode`](../interfaces/INode.md)\>\>

Returns the unhealthy nodes.

#### Returns

`Promise`<`Set`<[`INode`](../interfaces/INode.md)\>\>

___

### buildBasicOutput

▸ **buildBasicOutput**(`options`): `Promise`<`IBasicOutput`\>

Build a Basic Output.

#### Parameters

| Name | Type |
| :------ | :------ |
| `options` | [`IBasicOutputBuilderOptions`](../interfaces/IBasicOutputBuilderOptions.md) |

#### Returns

`Promise`<`IBasicOutput`\>

___

### buildAliasOutput

▸ **buildAliasOutput**(`options`): `Promise`<`IAliasOutput`\>

Build an Alias Output.

#### Parameters

| Name | Type |
| :------ | :------ |
| `options` | [`IAliasOutputBuilderOptions`](../interfaces/IAliasOutputBuilderOptions.md) |

#### Returns

`Promise`<`IAliasOutput`\>

___

### buildFoundryOutput

▸ **buildFoundryOutput**(`options`): `Promise`<`IFoundryOutput`\>

Build a Foundry Output.

#### Parameters

| Name | Type |
| :------ | :------ |
| `options` | [`IFoundryOutputBuilderOptions`](../interfaces/IFoundryOutputBuilderOptions.md) |

#### Returns

`Promise`<`IFoundryOutput`\>

___

### buildNftOutput

▸ **buildNftOutput**(`options`): `Promise`<`INftOutput`\>

Build an Nft Output.

#### Parameters

| Name | Type |
| :------ | :------ |
| `options` | [`INftOutputBuilderOptions`](../interfaces/INftOutputBuilderOptions.md) |

#### Returns

`Promise`<`INftOutput`\>
