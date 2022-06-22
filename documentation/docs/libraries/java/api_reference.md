---
description: Official IOTA Client Library Java API reference.
image: /img/logo/iota_mark_light.png
keywords:
- api
- Java
- param
- type
- endpoint
- builder
---
# API Reference - IOTA Client Library - Java binding

### Api

The Api enum contains a list of node endpoints we can call.
It is used in setting a timeout for a specific api call during the building of a client using `withApiTimeout(api, timeout)` .  

GET_HEALTH
GET_INFO
GET_PEERS
GET_TIPS
POST_MESSAGE
POST_MESSAGE_WITH_REMOTE_POW
GET_OUTPUT
GET_MILESTONE
GET_MESSAGE
GET_BALANCE

### ClientBuilder

#### new(): ClientBuilder

Construct the ClientBuilder instance, with which you can create a Client to connect to nodes

#### withNode(node): ClientBuilder

Adds an IOTA node to the client pool.

| Param | Type     | Description |
| ----- | -------- | ----------- |
| node  | `String` | A node URL  |

#### withNodes(nodes): ClientBuilder

Adds an array of IOTA nodes to the client pool.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| nodes | `String[]` | A node URL  |

#### withNodeAuth(node, jwt, username, password): ClientBuilder

Adds an IOTA node by its URL with optional jwt and or basic authentication

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| nodes | `String` | A node URL  |
| jwt | `String` | jwt or null  |
| username | `String` | username or null  |
| password | `String` | password or null  |

#### withPrimaryNode(node, jwt, username, password): ClientBuilder

Adds an IOTA node by its URL to be used as primary node, with optional jwt and or basic authentication

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| node | `String` | A node URL  |
| jwt | `String` | jwt or null  |
| username | `String` | username or null  |
| password | `String` | password or null  |

#### withPrimaryPowNode(node, jwt, username, password): ClientBuilder

Adds an IOTA node by its URL to be used as primary PoW node (for remote PoW), with optional jwt and or basic authentication

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| node | `String` | A node URL  |
| jwt | `String` | jwt or null  |
| username | `String` | username or null  |
| password | `String` | password or null  |

#### withPermanode(node, jwt, username, password): ClientBuilder

Adds an IOTA permanode by its URL, with optional jwt and or basic authentication

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| node | `String` | A node URL  |
| jwt | `String` | jwt or null  |
| username | `String` | username or null  |
| password | `String` | password or null  |

#### withNodePoolUrls(nodes): ClientBuilder

Add a list of nodes to use in a node pool

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| nodes | `String[]` | urls for the node pool  |

#### withOfflineMode(): ClientBuilder

Allows creating the client without nodes for offline address generation or signing

#### withNetwork(network): ClientBuilder

Add a list of nodes to use in a node pool

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| network | `String` | name of the network  |

#### withNodeSyncInterval(node_sync_interval): ClientBuilder

Set the node sync interval

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| node_sync_interval | `float` | the interval in seconds  |

#### withNodeSyncDisabled(): ClientBuilder

Disables the node syncing process.
Every node will be considered healthy and ready to use.

#### withQuorum(quorum): ClientBuilder

Set if quorum should be used or not

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| quorum | `boolean` | true for quorum on, false for off |

#### withQuorumSize(min_quorum_size): ClientBuilder

Set amount of nodes which should be used for quorum

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| min_quorum_size | `long` | size of the quorum |

#### withQuorumThreshold(threshold): ClientBuilder

Set quorum threshold percentage (0-100)

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| threshold | `long` | threshold for reaching a quorum decision |

#### withMqttBrokerOptions(options): ClientBuilder

Sets the MQTT broker options.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| options | `BrokerOptions` | the options to use in MQTT |

#### withLocalPow(local): ClientBuilder

Sets whether the PoW should be done locally or remotely.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| local | `boolean` | true for local PoW, false for remote |

#### withTipsInterval(tips): ClientBuilder

Sets after how many seconds new tips will be requested during PoW

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| tips | `long` | interval in seconds |

#### withRequestTimeout(timeout): ClientBuilder

Sets the default request timeout in seconds.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| timeout | `float` | the timeout in seconds |

#### withApiTimeout(api, timeout): ClientBuilder

Sets the request timeout in seconds for a specific API usage.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| api | `Api` | The api call to set the timeout for |
| timeout | `float` | the timeout in seconds |

#### withRequestTimeout(timeout): ClientBuilder

Sets the default request timeout in seconds.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| timeout | `float` | the timeout in seconds |

#### finish(): Client

Build the Client instance.

### Client

**Static methods**

#### Builder(): ClientBuilder

Static method to create a ClientBuilder

#### generateMnemonic(): String

Generates a new mnemonic.

#### mnemonicToHexSeed(): String

Returns a hex encoded seed for a mnemonic.

#### bech32ToHex(bech32): String

Transforms bech32 to hex

#### isAddressValid(address): boolean

Checks if a String address is valid.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `String` | The address to check validity for |

#### parseBech32Address(address): Address

Returns a valid Address parsed from a String.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `String` | The address to parse |

**instance methods**

#### getHealth(): boolean

GET /health endpoint

#### getNodeHealth(node): boolean

GET /health endpoint for a specific node

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| node | `String` | node to call get health on |

#### getInfo(): NodeInfoWrapper

GET /api/core/v2/info endpoint

#### getPeers(): PeerDto[]

GET /api/core/v2/peers endpoint

#### getTips(): ClientBuilder

GET /api/core/v2/tips endpoint

#### getOutput(output_id): OutputResponse

GET /api/core/v2/outputs/{outputId} endpoint
Find an output by its transaction_id and corresponding output_index.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| output_id | `String` | the id of the output |

#### getAddress(): GetAddressBuilder

GET /api/plugins/indexer/v1/outputs/basic{query} endpoint
Returns a builder ith which to construct the exact needs.

#### getAddressBalance(address): BalanceAddressResponse

Return the balance in iota for the given address; No seed or security level needed to do this
since we are only checking and already know the address.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `String` | the address to look up |

#### getAddressesBalances(addresses): BalanceAddressResponse[]

Return the balance in iota for the given addresses; No seed or security level needed to do this
since we are only checking and already know the addresses.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| addresses | `String[]` | the addresses to look up |

#### findOutputs(output_ids, addresses): OutputResponse[]

Find all outputs based on the requests criteria. This method will try to query multiple nodes if
the request amount exceeds individual node limit.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| output_ids | `String[]` | the output ids to look up |
| addresses | `String[]` | the addresses to look up |

#### getMilestone(index): MilestoneResponse

GET /api/core/v2/milestones/{index} endpoint
Get the milestone by the given index.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `long` | the milestone index |

#### getMilestoneUtxoChanges(index): MilestoneUtxoChangesResponse

GET /api/core/v2/milestones/{index}/utxo-changes endpoint
Gets the utxo changes by the given milestone index.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `long` | the milestone index |

#### getReceipts(): ReceiptDto[]

GET /api/core/v2/receipts/{migratedAt} endpoint
Get the receipts by the given milestone index.

#### getReceiptsMigratedAt(index): MilestoneUtxoChangesResponse

GET /api/core/v2/receipts/{migratedAt} endpoint
Get the receipts by the given milestone index.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `long` | the milestone index |

#### getTreasury(): TreasuryResponse[]

GET /api/core/v2/treasury endpoint
Get the treasury output.

#### getIncludedMessage(transaction_id): Message

GET /api/core/v2/transactions/{transactionId}/included-message
Returns the included message of the transaction.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| transaction_id | `TransactionId` | the transaction id (has a `fromString` constructor) |

#### postMessage(msg): MessageId

POST /api/core/v2/messages endpoint

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| msg | `Message` | The message to post to the node|

#### reattach(message_id): MessageWrap

Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
confirmed for a while.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| message_id | `MessageId` | The message to re-post to the node |

#### reattachUnchecked(message_id): MessageWrap

Reattach a message without checking if it should be reattached

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| message_id | `MessageId` | The message to re-post to the node |

#### promote(message_id): MessageWrap

Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
method should error out and should not allow unnecessary promotions.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| message_id | `MessageId` | The message to promote to the node |

#### promoteUnchecked(message_id): MessageWrap

Promote a message without checking if it should be promoted

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| message_id | `MessageId` | The message to promote to the node |

#### getBalance(seed): GetBalanceBuilderApi

Return the balance for a provided seed and its wallet chain account index.
Addresses with balance must be consecutive, so this method will return once it encounters a zero
balance address.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| seed | `String` | the seed we use to generate addresses |

#### message(): ClientMessageBuilder

A generic send function for easily sending transaction or indexation messages.

#### getMessage(): GetMessageBuilder

GET /api/core/v2/messages/{messageId} endpoint

#### getAddresses(seed): GetAddressesBuilder

Return a list of addresses from the seed regardless of their validity.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| seed | `String` | the seed we use to generate addresses |

#### retryUntilIncluded(message_id, interval, max_attempts): MessageWrap[]

Retries (promotes or reattaches) a message for provided message id until it's included (referenced by a
milestone). Default interval is 5 seconds and max attempts is 10. Returns reattached messages. Set to -1 for defaults.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| message_id | `MessageId` | the id of the message we wish to have included |
| interval | `long` | delay in between retries in seconds |
| max_attempts | `long` | maximum amount of retries before we error out |

#### findInputs(addresses, amount): UtxoInput[]

Function to find inputs from addresses for a provided amount (useful for offline signing)

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| addresses | `String[]` | A list of addresses to check for inputs |
| amount | `long` | The value we need these inputs to contain in total |

### ClientMessageBuilder

#### withSeed(seed): ClientMessageBuilder

Sets the seed.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| seed | `String` | the seed we use to sign messages |

#### withAccountIndex(account_index): ClientMessageBuilder

Sets the account index.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| account_index | `long` | The account index we use |

#### withInitialAddressIndex(initial_address_index): ClientMessageBuilder

Sets the index of the address to start looking for balance.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| initial_address_index | `long` | The starting index |

#### withInput(initial_address_index): ClientMessageBuilder

Set a custom input(transaction output)

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| input | `UtxoInput` | custom input |

#### withInputRange(low, high): ClientMessageBuilder

Set a custom range in which to search for addresses for custom inputs. Default: 0..100

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| low | `long` | lower end of the range (must be 0 or higher) |
| high | `long` | higher end of the range (must be higher than `low`) |

#### withOutput(address, amount): ClientMessageBuilder

Set a transfer to the builder, tries to parse address

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `String` | The address we transfer the `amount` to |
| amount | `long` | the amount of iota to send to the `address` |

#### withOutputHex(address, amount): ClientMessageBuilder

Set a transfer to the builder, address needs to be hex encoded

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `String` | The address we transfer the `amount` to |
| amount | `long` | the amount of iota to send to the `address` |

#### withDustAllowanceOutput(address, amount): ClientMessageBuilder

Set a dust allowance transfer to the builder, address needs to be Bech32 encoded

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `String` | The address we transfer the dust `amount` to |
| amount | `long` | the amount of iota to send to the `address`. Must be more than 1.000.000 |

#### withIndexVec(index): ClientMessageBuilder

Set indexation to the builder

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `byte[]` | index in `bytes` |

#### withIndexString(index): ClientMessageBuilder

Set indexation to the builder

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `String` | The index |

#### withData(data): ClientMessageBuilder

Set the data part of a message to the builder

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| data | `byte[]` | data in `bytes` |

#### withDataString(data): ClientMessageBuilder

Set the data part of a message to the builder

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| data | `String` | The data |

#### prepareTransaction(): PreparedTransactionData

Prepare a transaction. This consumes the builder.

#### signTransaction(prepared_transaction_data, seed, inputs_range_low, inputs_range_high): MessagePayload

Sign the transaction. Set inputsRangeLow and high to 0 for not using an input range. This consumes the builder.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| prepared_transaction_data | `PreparedTransactionData` | The prepared data (from `prepareTransaction`) |
| seed | `String` | The seed we use for address finding and signing |
| inputs_range_low | `long` | The 1 lower range (minimum 0) |
| inputs_range_high | `long` | The the inputs higher range (minimum lower + 1) |

#### finish(payload): Message

Consume the builder and return the message made with the generic message payload

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| payload | `MessagePayload` | The data |

#### finishTransaction(payload): Message

Consume the builder and return the message made with a TransactionPayload

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| payload | `TransactionPayload` | The data |

#### finishMilestone(payload): Message

Consume the builder and return the message made with a MilestonePayload

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| payload | `MilestonePayload` | The data |

#### finishIndex(payload): Message

Consume the builder and return the message made with an TaggedPayload
| Param | Type       | Description |
| ----- | ---------- | ----------- |
| payload | `TaggedPayload` | The data |

#### finishReceipt(payload): Message

Consume the builder and return the message made with a ReceiptPayload

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| payload | `ReceiptPayload` | The data |

#### finishTreasury(payload): Message

Consume the builder and return the message made with a TreasuryPayload

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| payload | `TreasuryPayload` | The data |

#### finish(): Message

Consume the builder and return the message

### GetMessageBuilder

#### indexString(index): MessageId[]

GET /api/core/v2/messages?index={Index} endpoint
Consume the builder and search for messages matching the index

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `byte[]` | index in `bytes` |

#### indexVec(index): MessageId[]

GET /api/core/v2/messages?index={Index} endpoint
Consume the builder and search for messages matching the index

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `String` | The index |

#### data(message_id): Message

GET /api/core/v2/messages/{messageID} endpoint
Consume the builder and find a message by its identifier. This method returns the given message object if it exists.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| message_id | `MessageId` | The id of the message |

#### metadata(message_id): MessageMetadata

GET /api/core/v2/messages/{messageID}/metadata endpoint
Consume the builder and find a message by its identifier. This method returns the given message metadata if it exists.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| message_id | `MessageId` | The id of the message |

#### raw(message_id): String

GET /api/core/v2/messages/{MessageId} endpoint
Consume the builder and find a message by its identifier. This method returns the given message raw data if it exists.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| message_id | `MessageId` | The id of the message |

### GetAddressesBuilder

#### balance(address): BalanceAddressResponse

Consume the builder and get the balance of a given Bech32 encoded address.
If count equals maxResults, then there might be more outputs available but those were skipped for performance
reasons. User should sweep the address to reduce the amount of outputs.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `String` | The address were looking up the balance for |

#### outputs(address, options): BalanceAddressResponse

Consume the builder and get all outputs that use a given address.
If count equals maxResults, then there might be more outputs available but those were skipped for performance
reasons. User should sweep the address to reduce the amount of outputs.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `String` | The address were looking up the balance for |
| options | `OutputsOptions` | The options for which outputs to show |

### OutputsOptions

#### includeSpent(include_spent): void

Whether the query should include spent outputs or not.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| include_spent | `boolean` | true for including spent addresses, false for excluding |

#### outputType(output_type): void

On what type of output are we filtering. can be
SIGNATURE_LOCKED_SINGLE, SIGNATURE_LOCKED_DUST_ALLOWANCE or TREASURY

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| output_type | `OutputKind` | The output type filter. |

### Message

Represent the object that nodes gossip around the network.

#### builder(): MessageBuilder

Creates a new `MessageBuilder` to construct an instance of a `Message`.

#### networkId(): long

Returns the network id of a `Message`.

#### nonce(): long

Returns the nonce of a `Message`.

#### parents(): MessageId []

Returns the parents of a `Message`.

#### payload(): Optional&lt;MessagePayload&gt;

Returns the optional payload of a `Message`.

### MessageBuilder

A builder to build a `Message`.

#### new(): MessageBuilder

A builder to build a `Message`.

#### networkId(network_id): MessageBuilder

Adds a network id to a `MessageBuilder`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| network_id | `long` | The network id of the message |

#### parents(parents): MessageBuilder

Adds parents to a `MessageBuilder`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| parents | `MessageId[]` | The parents of this message |

#### payload(payload): MessageBuilder

Adds a payload to a `MessageBuilder`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| payload | `MessagePayload` | The payload of the message |

### MessagePayload

#### deserialize(serialised_data): MessagePayload

Turns a serialized message payload string back into its class

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| serialised_data | `String` | The serialised Json MessagePayload string |

#### serialize(): String

Serializes the message payload into a json string.

#### payloadType(): MessagePayloadType

Get the type of message this contains (used to select the correct getter). Possible types are TRANSACTION, MILESTONE, INDEXATION, RECEIPT and TREASURY_TRANSACTION.

#### getAsIndexation(): Optional&lt;TaggedPayload&gt;

Get this Payload as a Indexation payload type

#### getAsTransaction(): Optional&lt;TransactionPayload&gt;

Get this Payload as a TransactionPayload type

#### getAsTreasury(): Optional&lt;TreasuryPayload&gt;

Get this Payload as a TreasuryPayload type

#### getAsMilestone(): Optional&lt;MilestonePayload&gt;

Get this Payload as a MilestonePayload type

#### getAsReceipt(): Optional&lt;ReceiptPayload&gt;

Get this Payload as a ReceiptPayload type

### TaggedPayload

#### fromBytes(index, data): void

Creates a new `TaggedPayload`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `byte []` | index bytes |
| data | `byte []` | data bytes |

#### fromStrings(index, data): void

Creates a new `TaggedPayload` from strings

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `String` | index string |
| data | `String` | data string |

#### index(): byte []

Returns the index of an `TaggedPayload`.

#### data(): byte []

Returns the data of an `TaggedPayload`.

### TransactionPayload

#### builder(): TransactionPayloadBuilder

Return a new `TransactionPayloadBuilder` to build a `TransactionPayload`.

#### essence(): Essence

Return the essence of a `TransactionPayload`.

#### unlockBlocks(): UnlockBlock []

Return unlock blocks of a `TransactionPayload`.

#### id(): TransactionId

Computes the identifier of a `TransactionPayload`.

### TransactionPayloadBuilder

#### new(): TransactionPayloadBuilder

Creates a new `TransactionPayloadBuilder`.

#### withEssence(essence): TransactionPayloadBuilder

Adds an essence to a `TransactionPayloadBuilder`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| essence | `Essence` | index bytes |

#### withUnlockBlocks(unlock_blocks): TransactionPayloadBuilder

Adds unlock blocks to a `TransactionPayloadBuilder`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| unlock_blocks | `UnlockBlocks` | index bytes |

#### finish(): TransactionPayload

Finishes a `TransactionPayloadBuilder` into a `TransactionPayload`.

### TreasuryPayload

#### new(input, output): TreasuryPayload

Creates a new `TreasuryPayload`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| input | `TreasuryInput` | The input information of this payload |
| output | `TreasuryOutput` | the output information of this payload |

#### input(): TreasuryInput

Return the TreasuryInput

#### output(): TreasuryOutput

Returns the TreasuryOutput

### MilestonePayload

#### essence(): MilestonePayloadEssence

Returns the essence of a `MilestonePayload`.

#### signatures(): MilestoneSignature[]

Returns the signatures of a `MilestonePayload`.

#### id(): long

Computes the identifier of a `MilestonePayload`.

#### validate(applicable_public_keys, min_threshold): void

Semantically validate a `MilestonePayload`. Throws an error if the milestone is considered invalid.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| applicable_public_keys | `String[]` | hex encoded list of public keys used by the milestone |
| min_threshold | `long` | Minimum amount of signatures we need to verify on either side |

### ReceiptPayload

#### from(migrated_at, last, funds, transaction): void

Creates a new `ReceiptPayload`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| migrated_at | `long` | The milestone index at which the funds of a `ReceiptPayload` were migrated |
| last | `boolean` | whether a `ReceiptPayload` is the final one for a given migrated at index. |
| funds | `MigratedFundsEntry[]` | The funds which are migrated  |
| transaction | `MessagePayload` | The `TreasuryTransaction` used to fund the funds. Must be a Treasury type  |

#### migratedAt(): long

Returns the milestone index at which the funds of a `ReceiptPayload` were migrated at in the legacy network.

#### last(): boolean

Returns whether a `ReceiptPayload` is the final one for a given migrated at index.

#### funds(): MigratedFundsEntry[]

The funds which were migrated with a `ReceiptPayload`.

#### amount(): long

Returns the sum of all `MigratedFundsEntry` items within a `ReceiptPayload`.

#### transaction(): TreasuryPayload

The `TreasuryTransaction` used to fund the funds of a `ReceiptPayload`.

### Essence

#### getAsRegular(): Optional&lt;RegularEssence&gt;

Get this Essence as a RegularEssence type

### RegularEssence

#### inputs(): Input[] 

Gets the transaction inputs.

#### outputs(): Output[]

Gets the transaction outputs.

#### payload(): Optional&lt;MessagePayload&gt;

Gets the optional payload attached to this transaction

### MilestonePayloadEssence

#### index(): long

Returns the index of a `MilestonePayloadEssence`.

#### timestamp(): long

Returns the timestamp of a `MilestonePayloadEssence`.

#### parents():  MessageId[]

Returns the parents of a `MilestonePayloadEssence`.

#### merkleProof(): byte[]

Returns the merkle proof of a `MilestonePayloadEssence`.

#### nextPowScore(): long

Returns the next proof of work score of a `MilestonePayloadEssence`.

#### nextPowScoreMilestone(): long

Returns the next proof of work index of a `MilestonePayloadEssence`.

#### publicKeys(): PublicKey[]

Returns the public keys of a `MilestonePayloadEssence`.

#### receipt(): Optional&lt;ReceiptPayload&gt;

Returns the optional receipt of a `MilestonePayloadEssence`.

#### hash(): byte[]

Hashes the `MilestonePayloadEssence to be signed.`

### UnlockBocks

#### from(unlock_blocks): UnlockBocks

Constructs an UnlockBlocks type from an array of blocks

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| unlock_blocks | `UnlockBock[]` | The UnlockBocks to add |

#### get(index) Optional&lt;UnlockBlock&gt;

Gets a clone of an `UnlockBlock` from `UnlockBlocks`.
Returns the referenced unlock block if the requested unlock block was a reference.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `long` | The UnlockBock to get |

### UnlockBock

#### kind(): UnlockBlockKind

Get the type of message this contains (used to select the correct getter). Possible types are ED25519 and REFERENCE.

#### getAsReference(): Optional&lt;ReferenceUnlock&gt;

Get this UnlockBock as a Reference Unlock type

#### getAsSignature(): Optional&lt;SignatureUnlock&gt;

Get this UnlockBock as a SignatureUnlock type

### ReferenceUnlock

#### from(index): ReferenceUnlock

Creates a new `ReferenceUnlock` by index

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| index | `int` | The index this Reference is actually using |

#### index(): long

Return the index of a `ReferenceUnlock`.

### SignatureUnlock

#### new(public_key, signature): SignatureUnlock

Create a new SignatureUnlock

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| public_key | `byte[]` | The public_key associated with the signature |
| signature | `byte[]` | The signature of this block |


### MigratedFundsEntry

#### from(hash, output): MigratedFundsEntry

Creates a new `MigratedFundsEntry`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| hash | `String` | The tail transaction hash of the entry |
| output | `SignatureLockedSingleOutput` | The output used in this entry |

#### tailTransactionHash(): String

Returns the tail transaction hash of a `MigratedFundsEntry`.

#### output(): SignatureLockedSingleOutput

Returns the output of a `MigratedFundsEntry`.

### SignatureLockedSingleOutput

Describes a deposit to a single address which is unlocked via a signature.

#### from(address, amount): SignatureLockedSingleOutput

Creates a new `SignatureLockedSingleOutput`.

| Param | Type       | Description |
| ----- | ---------- | ----------- |
| address | `Address` | The address of the output |
| amount | `long` | The amount of the output |

#### address(): Address

Returns the address of a `SignatureLockedSingleOutput`.

#### amount(): long

Returns the amount of a `SignatureLockedDustAllowanceOutput`.