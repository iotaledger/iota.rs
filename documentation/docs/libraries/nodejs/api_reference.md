---
description: Official IOTA Client Library Node.js API reference.
image: /img/logo/libraries.png
keywords:
- api
- nodejs
- param
- type
- client builder
---
# API Reference - IOTA Client Library - Node.js binding

Node.js binding to the IOTA client library.

## Installation

- Using NPM:

```bash
$ npm i @iota/client
```

- Using yarn:

```bash
$ yarn add @iota/client
```

## Requirements

One of the following Node.js version: '12.x', '14.x', '16.x'

If there is no prebuilt binary available for your system you need `Rust` and `Cargo`, to build it yourself. Install them [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Getting Started

After you linked the library, you can create a `Client` instance and interface with it.

```javascript
const { ClientBuilder } = require('@iota/client')
const client = new ClientBuilder()
    .node('https://api.lb-0.h.chrysalis-devnet.iota.cafe')
    .build()
client.getInfo().then(console.log).catch(console.error)
```

Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.

## API Reference

### ClientBuilder

#### node(url): ClientBuilder

Adds an IOTA node to the client pool.

| Param | Type     | Description |
| ----- | -------- | ----------- |
| url   | `string` | A node URL  |

**Returns** the client builder instance for chained calls.

#### nodeAuth(url, authOptions): ClientBuilder

Adds an IOTA node with authentication to the client pool.

| Param       | Type                                  | Description                     |
| ----------- | ------------------------------------- | ------------------------------- |
| url         | `string`                              | A node URL                      |
| authOptions | `[NodeAuthOptions](#nodeauthoptions)` | Options for node authentication |

**Returns** the client builder instance for chained calls.

#### primaryNode(url, [, authOptions]): ClientBuilder

Add a node to always connect first to with optional authentication.

| Param       | Type                                  | Description                     |
| ----------- | ------------------------------------- | ------------------------------- |
| url         | `string`                              | A node URL                      |
| authOptions | `[NodeAuthOptions](#nodeauthoptions)` | Options for node authentication |

**Returns** the client builder instance for chained calls.

#### primaryPowNode(url, [, authOptions]): ClientBuilder

Add a node to always connect first to when using remote PoW with optional authentication. Will overwrite the primary node for this case.

| Param       | Type                                  | Description                     |
| ----------- | ------------------------------------- | ------------------------------- |
| url         | `string`                              | A node URL                      |
| authOptions | `[NodeAuthOptions](#nodeauthoptions)` | Options for node authentication |

**Returns** the client builder instance for chained calls.

#### permanode(url, [, authOptions]): ClientBuilder

Add a permanode.

| Param       | Type                                  | Description                     |
| ----------- | ------------------------------------- | ------------------------------- |
| url         | `string`                              | A node URL                      |
| authOptions | `[NodeAuthOptions](#nodeauthoptions)` | Options for node authentication |

**Returns** the client builder instance for chained calls.

#### nodes(urls): ClientBuilder

Adds a list of IOTA nodes to the client pool.

| Param | Type       | Description           |
| ----- | ---------- | --------------------- |
| url   | `string[]` | An array of node URLs |

**Returns** the client builder instance for chained calls.

#### nodePoolUrls(urls): ClientBuilder

Adds a list of IOTA nodes from node pool URLs to the client pool.

| Param | Type       | Description                |
| ----- | ---------- | -------------------------- |
| url   | `string[]` | An array of node pool URLs |

**Returns** the client builder instance for chained calls.

#### network(networkName): ClientBuilder

Set a network to get default nodes for it. Can be "testnet" or "mainnet".
Nodes that don't belong to this network are ignored.

| Param       | Type     | Description |
| ----------- | -------- | ----------- |
| networkName | `string` | The network |

**Returns** the client builder instance for chained calls.

#### quorum(enabled): ClientBuilder

Defines how many of nodes will be queried at the same time to check for quorum.

| Param   | Type      | Description                            |
| ------- | --------- | -------------------------------------- |
| enabled | `boolean` | Define if quourm should be used or not |

**Returns** the client builder instance for chained calls.

#### quorumSize(size): ClientBuilder

Defines how many of nodes will be queried at the same time to check for quorum.

| Param | Type     | Description                              |
| ----- | -------- | ---------------------------------------- |
| size  | `number` | The number of nodes that will be queried |

**Returns** the client builder instance for chained calls.

#### quorumThreshold(threshold): ClientBuilder

Defines the minimum amount of nodes from the quorum pool that need to agree if we want to consider the result true.

| Param     | Type     | Description             |
| --------- | -------- | ----------------------- |
| threshold | `number` | Minimum amount of nodes |

**Returns** the client builder instance for chained calls.

#### brokerOptions(options): ClientBuilder

Sets the options for the MQTT connection with the node.

| Param   | Type                              | Description             |
| ------- | --------------------------------- | ----------------------- |
| options | `[BrokerOptions](#brokeroptions)` | The MQTT broker options |

**Returns** the client builder instance for chained calls.

#### nodeSyncInterval(interval): ClientBuilder

Sets the node syncing interval.

| Param    | Type     | Description                               |
| -------- | -------- | ----------------------------------------- |
| interval | `number` | The interval for the node syncing process |

**Returns** the client builder instance for chained calls.

#### disableNodeSync(): ClientBuilder

Disables the node syncing process. Every node will be considered healthy and ready to use.

**Returns** the client builder instance for chained calls.

#### requestTimeout(timeoutMs): ClientBuilder

Sets the default HTTP request timeout.

| Param   | Type     | Description                 |
| ------- | -------- | --------------------------- |
| timeout | `number` | The timeout in milliseconds |

**Returns** the client builder instance for chained calls.

#### apiTimeout(api, timeoutMs): ClientBuilder

Sets the HTTP request timeout for the specified API.

| Param   | Type                                                                                      | Description                        |
| ------- | ----------------------------------------------------------------------------------------- | ---------------------------------- |
| api     | `GetHealth` \| `GetInfo` \| `GetTips` \| `PostMessage` \| `GetOutput` \| `GetMilestone` | The API to set the request timeout |
| timeout | `number`                                                                                  | The timeout in milliseconds        |

**Returns** the client builder instance for chained calls.

#### localPow(local): ClientBuilder

Sets the PoW type.

| Param | Type      | Description                                                |
| ----- | --------- | ---------------------------------------------------------- |
| local | `boolean` | Flag determining if PoW should be done locally or remotely |

**Returns** the client builder instance for chained calls.

#### build(): Client

Builds the client instance.

**Returns** a [Client](#client) instance.

### Client

#### networkInfo(): NetworkInfo

Gets the cached network info.

**Returns** a [NetworkInfo](#networkinfo) instance.

#### subscriber(): TopicSubscriber

Gets a handle to the MQTT topic subscriber.

**Returns** a [TopicSubscriber](#topicsubscriber) instance.

#### message(): MessageSender

Initiates the builder to send messages.

**Returns** a [MessageSender](#messagesender) instance.

#### getUnspentAddress(seed): UnspentAddressGetter

Get a valid unspent address.

| Param | Type     | Description                    |
| ----- | -------- | ------------------------------ |
| seed  | `string` | The hex-encoded seed to search |

**Returns** a [UnspentAddressGetter](#unspentaddressgetter) instance.

#### getAddresses(seed): AddressGetter

Find addresses from the seed regardless of their validity.

| Param | Type     | Description                    |
| ----- | -------- | ------------------------------ |
| seed  | `string` | The hex-encoded seed to search |

**Returns** a [AddressGetter](#addressgetter) instance.

#### findMessages(indexationKeys, messageIds): Promise<Message[]>

Finds all messages associated with the given indexation keys and message ids.

| Param          | Type       | Description                             |
| -------------- | ---------- | --------------------------------------- |
| indexationKeys | `string[]` | The list of indexations keys too search |
| messageIds     | `string[]` | The list of message ids to search       |

**Returns** a promise resolving to the list of the found messages.

#### getBalance(seed: string): BalanceGetter

Get balance on a given seed

| Param | Type     | Description                    |
| ----- | -------- | ------------------------------ |
| seed  | `string` | The hex-encoded seed to search |

**Returns** a [BalanceGetter](#balancegetter) instance.

#### getAddressBalances(addresses): Promise<AddressBalance[]>

Get the balance in iotas for the given addresses.

| Param     | Type       | Description                     |
| --------- | ---------- | ------------------------------- |
| addresses | `string[]` | The list of addresses to search |

**Returns** A promise resolving to the list of `{ address, balance }` pairs.

#### generateMnemonic()

Returns a random generated Bip39 mnemonic with the English word list.

**Returns** A String

#### mnemonicToHexSeed(mnemonic)

Returns the seed hex encoded.

| Param    | Type     | Description                                           |
| -------- | -------- | ----------------------------------------------------- |
| mnemonic | `string` | Bip39 mnemonic with words from the English word list. |

**Returns** A String

#### bech32ToHex(bech32)

Returns a parsed hex String from bech32.

| Param  | Type     | Description               |
| ------ | -------- | ------------------------- |
| bech32 | `string` | The address Bech32 string |

**Returns** A String

#### hexToBech32(hex, bech32_hrp (optional))

Returns a parsed bech32 String from hex.

| Param      | Type     | Description               |
| ---------- | -------- | ------------------------- |
| bech32     | `string` | The address Bech32 string |
| bech32_hrp | `string` | The Bech32 hrp string     |

**Returns** A String

#### isAddressValid(address: string): boolean

Checks if a given address is valid.

| Param   | Type     | Description               |
| ------- | -------- | ------------------------- |
| address | `string` | The address Bech32 string |

**Returns** A boolean.

#### getMessageId(message: string): boolean

Returns the message id from a message.

| Param   | Type     | Description    |
| ------- | -------- | -------------- |
| message | `string` | The message id |

**Returns** the message id.

#### retry(messageId: string): Promise<Message/>

Retries (promotes or reattaches) the message associated with the given id.

| Param     | Type     | Description                    |
| --------- | -------- | ------------------------------ |
| messageId | `string` | The id of the message to retry |

**Returns** A promise resolving to the new [Message](#message) instance.

#### retryUntilIncluded(messageId: string[, interval: int, maxAttempts: int]): Promise<Message/>

Retries (promotes or reattaches) the message associated with the given id until it's included in the Tangle.
Default interval is 5 seconds and max_attempts is 10.

| Param                 | Type     | Description                                            |
| --------------------- | -------- | ------------------------------------------------------ |
| messageId             | `string` | The id of the message to retry                         |
| [options.interval]    | `int`    | The interval in seconds in which we retry the message. |
| [options.maxAttempts] | `int`    | The maximum of attempts we retry the message.          |

**Returns** the message ids and [Message](#message) of reattached messages.

#### consolidateFunds(seed: string, accountIndex: int, startIndex: int, endIndex: int): Promise<string/>

Function to consolidate all funds from a range of addresses to the address with the lowest index in that range

| Param        | Type     | Description                                                           |
| ------------ | -------- | --------------------------------------------------------------------- |
| seed         | `string` | The seed                                                              |
| accountIndex | `int`    | The account index.                                                    |
| startIndex   | `int`    | The lowest address index, funds will be consolidated to this address. |
| endIndex     | `int`    | The address index until which funds will be consolidated              |

**Returns** the address to which the funds got consolidated, if any were available.

#### getInfo(): Promise<Wrapper/>

Gets information about the node.

**Returns** a promise resolving to the [NodeInfoWrapper](#nodeinfowrapper) object.

#### getTips(): Promise<[string, string]>

Gets two non-lazy tips.

**Returns** a promise resolving to an array of length 2 containing the message ids of the tips.

#### postMessage(message): Promise<string/>

Submits a message.

| Param   | Type                        | Description           |
| ------- | --------------------------- | --------------------- |
| message | `[MessageDto](#messagedto)` | The message to submit |

**Returns** the message identifier.

#### getMessage(): MessageFinder

Gets a message from its identifier.

**Returns** an instance of the [MessageFinder](#messagefinder) for choices of response.

#### getOutput(outputId): Promise<Metadata/>

Gets the UTXO outputs associated with the given output id.

| Param    | Type     | Description                    |
| -------- | -------- | ------------------------------ |
| outputId | `string` | The id of the output to search |

**Returns** a promise resolving to the associated [OutputMetadata](#outputmetadata).

#### findOutputs(outputIds, addresses): Promise<OutputMetadata[]>

Gets the UTXO outputs associated with the given output ids and addresses.

| Param     | Type       | Description                      |
| --------- | ---------- | -------------------------------- |
| addresses | `string[]` | The list of addresses to search  |
| outputIds | `string[]` | The list of output ids to search |

**Returns** a promise resolving to a list of [OutputMetadata](#outputmetadata).

#### getAddressOutputs(address[, options]): Promise<string[]>

Gets the UTXO outputs associated with the given address.

| Param                  | Type                                                        | Description                                           |
| ---------------------- | ----------------------------------------------------------- | ----------------------------------------------------- |
| address                | `string`                                                    | The address Bech32 string                             |
| [options.includeSpent] | `boolean`                                                   | Whether the query should include spent outputs or not |
| [options.outputType]   | `SignatureLockedSingle` \| `SignatureLockedDustAllowance`  | The output type filter                                |

**Returns** a promise resolving to a list of output ids.

#### getAddressBalance(address): Promise<Balance/>

Gets the balance of the given address.

| Param   | Type     | Description               |
| ------- | -------- | ------------------------- |
| address | `string` | The address Bech32 string |

#### getMilestone(index): Promise<Metadata/>

Gets the milestone by the given index.

| Param | Type     | Description                |
| ----- | -------- | -------------------------- |
| index | `number` | The index of the milestone |

**Returns** a promise resolving to the [MilestoneMetadata](#milestonemetadata).

#### getMilestoneUtxoChanges(index): Promise<UTXOChanges/>

Gets the utxo changes by the given milestone index.

| Param | Type     | Description                |
| ----- | -------- | -------------------------- |
| index | `number` | The index of the milestone |

**Returns** a promise resolving to the [MilestoneUTXOChanges](#MilestoneUTXOChanges).

#### getReceipts(): Promise<Receipts[]>

Get all receipts.

**Returns** a promise resolving to the [Receipts](#Receipts).

#### getReceiptsMigratedAt(index): Promise<Receipts[]>

Get all receipts for a given milestone index

| Param | Type     | Description                |
| ----- | -------- | -------------------------- |
| index | `number` | The index of the milestone |

**Returns** a promise resolving to the [Receipts](#Receipts).

#### getTreasury(): Promise<Treasury/>

Get the treasury amount.

**Returns** a promise resolving to the [Treasury](#Treasury).

#### getIncludedMessage(): Promise<Message/>

Get the included message of a transaction.

| Param | Type     | Description               |
| ----- | -------- | ------------------------- |
| index | `string` | The id of the transaction |

**Returns** A promise resolving to the new [Message](#message) instance.

#### reattach(messageId): Promise<Message/>

Reattaches the message associated with the given id.

| Param     | Type     | Description                       |
| --------- | -------- | --------------------------------- |
| messageId | `string` | The id of the message to reattach |

**Returns** A promise resolving to the new [Message](#message) instance.

#### promote(messageId): Promise<Message/>

Promotes the message associated with the given id.

| Param     | Type     | Description                      |
| --------- | -------- | -------------------------------- |
| messageId | `string` | The id of the message to promote |

**Returns** A promise resolving to the new [Message](#message) instance.

### NetworkInfo

| Field       | Type      | Description                           |
| ----------- | --------- | ------------------------------------- |
| network     | `string`  | The network                           |
| networkId   | `number`  | The network hashed                    |
| bech32HRP   | `string`  | Bech32 HRP for this network           |
| minPoWScore | `number`  | The network's minimum score for PoW   |
| localPow    | `boolean` | Whether we are using local PoW or not |

### TopicSubscriber

Possible topics:

```bash
milestones/latest
milestones/confirmed

messages
messages/referenced
messages/indexation/{index}
messages/{messageId}/metadata
transactions/{transactionId}/included-message

outputs/{outputId}

addresses/{address}/outputs
addresses/ed25519/{address}/outputs
```

#### topic(topic): TopicSubscriber

Adds a topic to this manager instance.

| Param | Type     | Description  |
| ----- | -------- | ------------ |
| topic | `string` | A MQTT topic |

**Returns** the topic subscriber instance for chained calls.

#### topics(topic): TopicSubscriber

Adds a list of topics to this manager instance.

| Param  | Type       | Description             |
| ------ | ---------- | ----------------------- |
| topics | `string[]` | An array of MQTT topics |

**Returns** the topic subscriber instance for chained calls.

#### subscribe(cb): TopicSubscriber

Subscribe to the provided topics.

| Param | Type       | Description                                                      |
| ----- | ---------- | ---------------------------------------------------------------- |
| cb    | `function` | The topic handler callback in the form of `(err, message) => {}` |

**Returns** the topic subscriber instance for chained calls.

#### unsubscribe(cb: Callback): TopicSubscriber

Unsubscribes from the provided topics.

| Param | Type       | Description                                                                                |
| ----- | ---------- | ------------------------------------------------------------------------------------------ |
| cb    | `function` | A callback executed when the unsubscribe is finished in the form of `(err, message) => {}` |

**Returns** the topic subscriber instance for chained calls.

### MessageSender

Builder to create and submit messages to the Tangle.

#### index(index): MessageSender

Sets the message indexation. This field is required for indexation payloads.

| Param | Type                               | Description    |
| ----- | ---------------------------------- | -------------- |
| index | `string` \| `number[]` \| `Uint8Array` | The indexation |

**Returns** the message submit instance for chained calls.

#### data(data): MessageSender

Sets the indexation data.

| Param | Type                   | Description        |
| ----- | ---------------------- | ------------------ |
| data  | `string` \| `Uint8Array` | The message's data |

**Returns** the message submit instance for chained calls.

#### seed(seed): MessageSender

Sets the transaction account seed. This field is required for transaction payloads.

| Param | Type     | Description                                  |
| ----- | -------- | -------------------------------------------- |
| seed  | `string` | The hex-encoded seed of the account to spend |

**Returns** the message submit instance for chained calls.

#### parents(messageId): MessageSender

Sets 1-8 custom parent message ids.

| Param     | Type       | Description             |
| --------- | ---------- | ----------------------- |
| messageId | `string[]` | The parents message ids |

**Returns** the message submit instance for chained calls.

#### accountIndex(index): MessageSender

Sets the account index. This field is required for transactions.

| Param | Type     | Description       |
| ----- | -------- | ----------------- |
| index | `number` | The account index |

**Returns** the message submit instance for chained calls.

#### input(transactionId, index): MessageSender

Adds an output to the transaction.

| Param         | Type     | Description        |
| ------------- | -------- | ------------------ |
| transactionId | `string` | The transaction id |
| index         | `number` | The input index    |

**Returns** the message submit instance for chained calls.

#### inputRange(start, end): MessageSender

Defines the range in which to search for addresses fro custom inputs.

| Param | Type     | Description     |
| ----- | -------- | --------------- |
| start | `number` | The start index |
| end   | `number` | The end index   |

**Returns** the message submit instance for chained calls.

#### output(address, amount): MessageSender

Adds an output to the transaction.

| Param   | Type     | Description        |
| ------- | -------- | ------------------ |
| address | `string` | The output address |
| amount  | `number` | The output amount  |

**Returns** the message submit instance for chained calls.

#### dustAllowanceOutput(address, amount): MessageSender

Adds a dust allowance output to the transaction.

| Param   | Type     | Description        |
| ------- | -------- | ------------------ |
| address | `string` | The output address |
| amount  | `number` | The output amount  |

**Returns** the message submit instance for chained calls.

#### initialAddressIndex(index): MessageSender

Sets the initial address index to search for balance. Defaults to 0 if the function isn't called.

| Param | Type     | Description               |
| ----- | -------- | ------------------------- |
| index | `number` | The initial address index |

**Returns** the message submit instance for chained calls.

#### submit(): Promise<Wrapper/>

Submits the message.

**Returns** a promise resolving to the message identifier.

### UnspentAddressGetter

Gets a valid unspent address associated with the seed.

#### accountIndex(index): UnspentAddressGetter

Sets the account index. This field is required.

| Param | Type     | Description       |
| ----- | -------- | ----------------- |
| index | `number` | The account index |

**Returns** the address getter instance for chained calls.

#### initialAddressIndex(index): UnspentAddressGetter

Sets the initial address index. Defaults to 0 if the function isn't called.

| Param | Type     | Description               |
| ----- | -------- | ------------------------- |
| index | `number` | The initial address index |

**Returns** the address getter instance for chained calls.

#### get(): Promise<[Address, number]>

Performs the operation.

**Returns** a promise resolving to the [Address](#address) instance and its index.

### AddressGetter

Generates addresses with a given seed.

#### accountIndex(index): AddressGetter

Sets the account index. This field is required.

| Param | Type     | Description       |
| ----- | -------- | ----------------- |
| index | `number` | The account index |

**Returns** the address finder instance for chained calls.

#### range(start, end): AddressGetter

Defines the range of addresses to get. Defaults to `0..20` if the function isn't called.

| Param | Type     | Description             |
| ----- | -------- | ----------------------- |
| start | `number` | The first address index |
| end   | `number` | The last address index  |

**Returns** the address finder instance for chained calls.

#### includeInternal(): AddressGetter

Defines that public and internal address will be returned instead of only public addresses.

**Returns** the address finder instance for chained calls.

#### bech32Hrp(bech32Hrp): AddressGetter

Defines the bech32Hrp for the bech32 encoded addresses, required when generating addresses offline(with disableNodeSync()).

| Param     | Type     | Description                     |
| --------- | -------- | ------------------------------- |
| bech32Hrp | `string` | The bech32Hrp for the addresses |

**Returns** the address finder instance for chained calls.

#### get(): Address[] | [Address, bool][]

Performs the operation.

**Returns** an array of public [Address](#address) instances or an array of arrays with an Address and a bool,
where the bool defines whether it's an internal address or not.

### BalanceGetter

Gets balance on a given seed.

#### accountIndex(index): BalanceGetter

Sets the account index. This field is required.

| Param | Type     | Description       |
| ----- | -------- | ----------------- |
| index | `number` | The account index |

**Returns** the balance getter instance for chained calls.

#### initialAddressIndex(index): BalanceGetter

Sets the initial address index. Defaults to 0 if the function isn't called.

| Param | Type     | Description               |
| ----- | -------- | ------------------------- |
| index | `number` | The initial address index |

**Returns** the balance getter instance for chained calls.

#### gapLimit(amount): BalanceGetter

Sets the gapLimit to specify how many addresses will be checked each round.
If gapLimit amount of addresses in a row have no balance the BalanceGetter will return. Defaults to 20 if the function isn't called.

| Param    | Type     | Description               |
| -------- | -------- | ------------------------- |
| gapLimit | `number` | The initial address index |

**Returns** the balance getter instance for chained calls.

#### get(): Promise<number/>

Performs the operation.

**Returns** a promise resolving to the account balance.

### MessageFinder

Gets a message by indexation key or identifier.

#### initialAddressIndex(index): Promise<string[]/>

| Param | Type     | Description        |
| ----- | -------- | ------------------ |
| index | `string` | The indexation key |

Gets a list of message identifiers associated with the given indexation key.

**Returns** a promise resolving to the list of associated ids.

#### data(id): Promise<Message/>

Gets the message object associated with the given identifier.

| Param | Type     | Description            |
| ----- | -------- | ---------------------- |
| id    | `string` | The message identifier |

**Returns** a [Message](#message) object.

#### raw(id): Promise<string/>

Gets the message raw data.

| Param | Type     | Description            |
| ----- | -------- | ---------------------- |
| id    | `string` | The message identifier |

**Returns** the message raw data as string.

#### children(id): Promise<string[]>

Gets the children of the given message.

| Param | Type     | Description            |
| ----- | -------- | ---------------------- |
| id    | `string` | The message identifier |

**Returns** the list of message ids of the message children.

#### metadata(id): Promise<Metadata/>

Gets the metadata of the given message.

| Param | Type     | Description            |
| ----- | -------- | ---------------------- |
| id    | `string` | The message identifier |

**Returns** a [MessageMetadata](#messagemetadata) object.

### BrokerOptions

All fields are optional.

| Field                   | Type      | Description                                                                                           |
| ----------------------- | --------- | ----------------------------------------------------------------------------------------------------- |
| automaticDisconnect     | `boolean` | Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not. |
| timeout                 | `number`  | MQTT connection timeout in secods                                                                     |
| useWs                   | `boolean` | Defines if websockets should be used (true) or TCP (false)                                            |
| maxReconnectionAttempts | `number`  | Defines the maximum reconnection attempts before it returns an error                                  |
| port                    | `number`  | Defines the port to be used for the MQTT connection                                                   |

### NodeAuthOptions

| Field             | Type     | Description                                |
| ----------------- | -------- | ------------------------------------------ |
| jwt               | `string` | Optional JSON Web Token.                   |
| basicAuthName     | `string` | Optional name for basic authentication     |
| basicAuthPassword | `string` | Optional password for basic authentication |

### Address

| Field | Type     | Description                |
| ----- | -------- | -------------------------- |
| data  | `string` | Address as a Bech32 string |

### Message

| Field     | Type                  | Description                           |
| --------- | --------------------- | ------------------------------------- |
| networkId | `number`              | Network identifier                    |
| parents   | `string[]`            | Message ids of the message references |
| payload   | `[Payload](#payload)` | Message payload                       |
| nonce     | `number`              | Message nonce                         |

### MessageWrapper

| Field     | Type      | Description    |
| --------- | --------- | -------------- |
| message   | `Message` | Message        |
| messageId | `string`  | The message id |

#### Payload

| Field | Type                                                                                                                                                   | Description  |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------ |
| data  | `{ type: 'Transaction', data: TransactionPayload }` \| `{ type: 'Indexation', data: IndexationPayload }` \| `{ type: 'Milestone', data: MilestonePayload}` | Payload data |

##### TransactionPayload

| Field         | Type                        | Description         |
| ------------- | --------------------------- | ------------------- |
| essence       | `TransactionPayloadEssence` | Transaction essence |
| unlock_blocks | `UnlockBlock[]`             | Unlock blocks       |

- TransactionPayloadEssence

| Field   | Type                   | Description          |
| ------- | ---------------------- | -------------------- |
| inputs  | `Input[]`              | Inputs               |
| outputs | `Output[]`             | Outputs              |
| payload | `Payload` \| `undefined` | Payload for chaining |

- Input

| Field | Type     | Description              |
| ----- | -------- | ------------------------ |
| type  | `'UTXO'` | Input type identifier    |
| data  | `string` | The associated output id |

- Output

| Field | Type                                  | Description            |
| ----- | ------------------------------------- | ---------------------- |
| type  | `SignatureLockedSingle`             | Output type identifier |
| data  | `{ address: string, amount: number }` | The output definition  |

- UnlockBlock

| Field | Type                                    | Description                                           |
| ----- | --------------------------------------- | ----------------------------------------------------- |
| type  | `'Signature' \| 'Reference'`            | Unlock block type identifier                          |
| data  | `Ed25519SignatureUnlockBlock` \| `number` | Unlock block data (signature type or reference index) |

- Ed25519SignatureUnlockBlock

| Field      | Type       | Description        |
| ---------- | ---------- | ------------------ |
| public_key | `number[]` | Ed25519 public key |
| signature  | `number[]` | Ed25519 signature  |

##### IndexationPayload

| Field | Type       | Description                   |
| ----- | ---------- | ----------------------------- |
| index | `string`   | Indexation key                |
| data  | `number[]` | Indexation data as byte array |

##### MilestonePayload

| Field      | Type               | Description          |
| ---------- | ------------------ | -------------------- |
| essence    | `MilestoneEssence` | Milestone essence    |
| signatures | `number[][]`       | Milestone signatures |

- MilestoneEssence

| Field                      | Type         | Description                                            |
| -------------------------- | ------------ | ------------------------------------------------------ |
| index                      | `number`     | Milestone index                                        |
| timestamp                  | `number`     | Timestamp                                              |
| parents                    | `string[]`   | Message ids of the messages the milestone references   |
| merkleProof                | `number[]`   | Merkle proof                                           |
| nextPoWScore               | `number`     | Next PoW score                                         |
| nextPoWScoreMilestoneIndex | `number`     | Milestone index at which the nextPoWScore will be used |
| publicKeys                 | `number[][]` | public keys                                            |

### MessageDto

| Field   | Type                        | Description                                                             |
| ------- | --------------------------- | ----------------------------------------------------------------------- |
| parents | `string[]` \| `undefined`     | Message ids of the messages it references. `getTips` is used by default |
| payload | `[PayloadDto](#payloaddto)` | Message payload                                                         |

#### PayloadDto

| Field | Type                                            | Description  |
| ----- | ----------------------------------------------- | ------------ |
| data  | `TransactionPayloadDto` \| `IndexationPayloadDto` | Payload data |

##### TransactionPayloadDto

| Field        | Type                           | Description         |
| ------------ | ------------------------------ | ------------------- |
| essence      | `TransactionPayloadEssenceDto` | Transaction essence |
| unlockBlocks | `UnlockBlockDto[]`             | Unlock blocks       |

- TransactionPayloadEssenceDto

| Field   | Type                      | Description          |
| ------- | ------------------------- | -------------------- |
| inputs  | `string[]`                | Inputs               |
| outputs | `Output[]`                | Outputs              |
| payload | `PayloadDto` \| `undefined` | Payload for chaining |

- OutputDto

| Field   | Type     | Description    |
| ------- | -------- | -------------- |
| address | `string` | Output address |
| amount  | `amount` | Output amount  |

- UnlockBlockDto

| Field | Type                                       | Description                                           |
| ----- | ------------------------------------------ | ----------------------------------------------------- |
| data  | `Ed25519SignatureUnlockBlockDto` \| `number` | Unlock block data (signature type or reference index) |

- Ed25519SignatureUnlockBlockDto

| Field     | Type       | Description        |
| --------- | ---------- | ------------------ |
| publicKey | `number[]` | Ed25519 public key |
| signature | `number[]` | Ed25519 signature  |

##### IndexationPayloadDto

| Field | Type         | Description     |
| ----- | ------------ | --------------- |
| index | `string`     | Indexation key  |
| data  | `Uint8Array` | Indexation data |

##### AddressBalance

| Field       | Type      | Description            |
| ----------- | --------- | ---------------------- |
| address     | `string`  | Bech32 encoded address |
| balance     | `number`  | Address balance        |
| dustAllowed | `boolean` | Dust allowed           |

### MessageMetadata

| Field                      | Type                   | Description                                               |
| -------------------------- | ---------------------- | --------------------------------------------------------- |
| messageId                  | `string`               | Message identifier                                        |
| parents                    | `string[]`             | Message id of the messages it references                  |
| isSolid                    | `boolean`              | Message solid state                                       |
| shouldPromote              | `boolean` \| `undefined` | Indicates whether the message should be promoted or not   |
| shouldReattach             | `boolean` \| `undefined` | Indicates whether the message should be reattached or not |
| referencedByMilestoneIndex | `number` \| `undefined`  | Index of the milestone that references this message       |
| ledgerInclusionState       | `string` \| `undefined`  | Ledger inclusion state                                    |

### NodeInfoWrapper

| Field    | Type       | Description |
| -------- | ---------- | ----------- |
| url      | `string`   | Node url    |
| nodeinfo | `NodeInfo` | NodeInfo    |

### NodeInfo

| Field                       | Type       | Description                                       |
| --------------------------- | ---------- | ------------------------------------------------- |
| name                        | `string`   | Node name                                         |
| version                     | `string`   | Node version                                      |
| isHealthy                   | `boolean`  | Node health status                                |
| networkId                   | `string`   | Node network identifier                           |
| bech32HRP                   | `string`   | Bech32 HRP for this network                       |
| minPoWScore                 | `number`   | Min PoW score                                     |
| messagesPerSecond           | `number`   | Network stats: Messages per second in the network |
| referencedMessagesPerSecond | `number`   | Network stats: Referenced messages per second     |
| referencedRate              | `number`   | Network stats: referenced rate                    |
| latestMilestoneTimestamp    | `number`   | Timestamp of the latest milestone                 |
| latestMilestoneIndex        | `number`   | Index of the latest milestone                     |
| confirmedMilestoneIndex     | `number`   | Index of the confirmed milestone                  |
| pruningIndex                | `number`   | Pruning index                                     |
| features                    | `string[]` | List of node features                             |

### OutputMetadata

| Field         | Type      | Description                                      |
| ------------- | --------- | ------------------------------------------------ |
| messageId     | `string`  | Id of the message associated with the output     |
| transactionId | `string`  | Id of the transaction associated with the output |
| outputIndex   | `number`  | Output index                                     |
| isSpent       | `boolean` | Output spent state                               |
| address       | `string`  | Output address                                   |
| amount        | `number`  | Output amount                                    |

### MilestoneMetadata

| Field          | Type     | Description                                     |
| -------------- | -------- | ----------------------------------------------- |
| milestoneIndex | `number` | Milestone index                                 |
| messageId      | `string` | Id of the message associated with the milestone |
| timestamp      | `number` | Milestone timestamp                             |

### MilestoneUTXOChanges

| Field           | Type       | Description                        |
| --------------- | ---------- | ---------------------------------- |
| index           | `number`   | Milestone index                    |
| createdOutputs  | `string[]` | OutputIds from new created outputs |
| consumedOutputs | `string[]` | OutputIds from consumed outputs    |

### Receipts

| Field          | Type      | Description     |
| -------------- | --------- | --------------- |
| receipt        | `receipt` | Receipt         |
| milestoneIndex | `number`  | Milestone index |

### Treasury

| Field       | Type     | Description  |
| ----------- | -------- | ------------ |
| milestoneId | `string` | Milestone id |
| amount      | `number` | Amount       |
