# IOTA Client Library - Node.js binding

Node.js binding to the IOTA client library.

## Requirements

`Rust` and `Cargo` are required. Install them [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Ensure you have installed the required dependencies for the library [here](https://github.com/iotaledger/iota.rs/blob/dev/README.md).

## Installation

Currently the package isn't published so you'd need to link it to your project using `npm` or `yarn`.

- Using NPM:
```
$ git clone https://github.com/iotaledger/iota.rs
$ cd iota.rs/bindings/nodejs
$ npm link
$ cd /path/to/nodejs/project/
$ npm link iota-client
```
- Using yarn: 
```
$ git clone https://github.com/iotaledger/iota.rs
$ cd iota.rs/bindings/nodejs
$ yarn link
$ cd /path/to/nodejs/project/
$ yarn link iota-client
```

## Getting Started

After you linked the library, you can create a `Client` instance and interface with it.

```javascript
const { ClientBuilder } = require('iota-client')
const client = new ClientBuilder()
  .node('http://localhost:14265')
  .build()
client.getTips().then(console.log).catch(console.error)
```

## API Reference

### ClientBuilder

#### node(url): ClientBuilder

Adds an IOTA node to the client pool.

| Param | Type                | Description |
| ----- | ------------------- | ----------- |
| url   | <code>string</code> | A node URL  |

**Returns** the client builder instance for chained calls.

#### nodes(urls): ClientBuilder

Adds a list of IOTA nodes to the client pool.

| Param | Type                  | Description           |
| ----- | --------------------- | --------------------- |
| url   | <code>string[]</code> | An array of node URLs |

**Returns** the client builder instance for chained calls.

#### nodePoolUrls(urls): ClientBuilder

Adds a list of IOTA nodes from node pool URLs to the client pool.

| Param | Type                  | Description                |
| ----- | --------------------- | -------------------------- |
| url   | <code>string[]</code> | An array of node pool URLs |

**Returns** the client builder instance for chained calls.

#### network(networkName): ClientBuilder

Set a network to get default nodes for it. Can be "testnet" or "mainnet".
Nodes that don't belong to this network are ignored.

| Param       | Type                  | Description      |
| ----------- | --------------------- | ---------------- |
| networkName | <code>string</code>   | The network |

**Returns** the client builder instance for chained calls.

#### quorumSize(size): ClientBuilder

Defines how many of nodes will be queried at the same time to check for quorum.

| Param | Type                | Description                              |
| ----- | ------------------- | ---------------------------------------- |
| size  | <code>number</code> | The number of nodes that will be queried |

**Returns** the client builder instance for chained calls.

#### quorumThreshold(threshold): ClientBuilder

Defines the minimum amount of nodes from the quorum pool that need to agree if we want to consider the result true.

| Param     | Type                | Description             |
| --------- | ------------------- | ----------------------- |
| threshold | <code>number</code> | Minimum amount of nodes |

**Returns** the client builder instance for chained calls.

#### brokerOptions(options): ClientBuilder

Sets the options for the MQTT connection with the node.

| Param   | Type                                         | Description             |
| ------- | -------------------------------------------- | ----------------------- |
| options | <code>[BrokerOptions](#brokeroptions)</code> | The MQTT broker options |

**Returns** the client builder instance for chained calls.

#### nodeSyncInterval(interval): ClientBuilder

Sets the node syncing interval.

| Param    | Type                | Description                               |
| -------- | ------------------- | ----------------------------------------- |
| interval | <code>number</code> | The interval for the node syncing process |

**Returns** the client builder instance for chained calls.

#### disableNodeSync(): ClientBuilder

Disables the node syncing process. Every node will be considered healthy and ready to use.

**Returns** the client builder instance for chained calls.

#### defaultTimeout(timeoutMs): ClientBuilder

Sets the default HTTP request timeout.

| Param   | Type                | Description                 |
| ------- | ------------------- | --------------------------- |
| timeout | <code>number</code> | The timeout in milliseconds |

**Returns** the client builder instance for chained calls.

#### apiTimeout(api, timeoutMs): ClientBuilder

Sets the HTTP request timeout for the specified API.

| Param   | Type                                                                                                 | Description                        |
| ------- | ---------------------------------------------------------------------------------------------------- | ---------------------------------- |
| api     | <code>'GetHealth' \| 'GetInfo' \| 'GetTips' \| 'PostMessage' \| 'GetOutput' \| 'GetMilestone'</code> | The API to set the request timeout |
| timeout | <code>number</code>                                                                                  | The timeout in milliseconds        |

**Returns** the client builder instance for chained calls.

#### localPow(local): ClientBuilder

Sets the PoW type.

| Param | Type                 | Description                                                |
| ----- | -------------------- | ---------------------------------------------------------- |
| local | <code>boolean</code> | Flag determining if PoW should be done locally or remotely |

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

| Param | Type                | Description                    |
| ----- | ------------------- | ------------------------------ |
| seed  | <code>string</code> | The hex-encoded seed to search |

**Returns** a [UnspentAddressGetter](#unspentaddressgetter) instance.

#### getAddresses(seed): AddressGetter

Find addresses from the seed regardless of their validity.

| Param | Type                | Description                    |
| ----- | ------------------- | ------------------------------ |
| seed  | <code>string</code> | The hex-encoded seed to search |

**Returns** a [AddressGetter](#addressgetter) instance.

#### findMessages(indexationKeys, messageIds): Promise<Message[]>

Finds all messages associated with the given indexation keys and message ids.

| Param          | Type                  | Description                             |
| -------------- | --------------------- | --------------------------------------- |
| indexationKeys | <code>string[]</code> | The list of indexations keys too search |
| messageIds     | <code>string[]</code> | The list of message ids to search       |

**Returns** a promise resolving to the list of the found messages.

#### getBalance(seed: string): BalanceGetter

Get balance on a given seed and its wallet account index.

| Param | Type                | Description                    |
| ----- | ------------------- | ------------------------------ |
| seed  | <code>string</code> | The hex-encoded seed to search |

**Returns** a [BalanceGetter](#balancegetter) instance.

#### getAddressBalances(addresses): Promise<AddressBalance[]>

Get the balance in iotas for the given addresses.

| Param     | Type                  | Description                     |
| --------- | --------------------- | ------------------------------- |
| addresses | <code>string[]</code> | The list of addresses to search |

**Returns** A promise resolving to the list of `{ address, balance }` pairs.

#### retry(messageId: string): Promise<Message>

Retries (promotes or reattaches) the message associated with the given id.

| Param     | Type                | Description                    |
| --------- | ------------------- | ------------------------------ |
| messageId | <code>string</code> | The id of the message to retry |

**Returns** A promise resolving to the new [Message](#message) instance.

#### getInfo(): Promise<NodeInfo>

Gets information about the node.

**Returns** a promise resolving to the [NodeInfo](#nodeinfo) object.

#### getTips(): Promise<[string, string]>

Gets two non-lazy tips.

**Returns** a promise resolving to an array of length 2 containing the message ids of the tips.

#### postMessage(message): Promise<string>

Submits a message.

| Param   | Type                                   | Description           |
| ------- | -------------------------------------- | --------------------- |
| message | <code>[MessageDto](#messagedto)</code> | The message to submit |

**Returns** the message identifier.

#### getMessage(): MessageFinder

Gets a message from its identifier.

**Returns** an instance of the [MessageFinder](#messagefinder) for choices of response.

#### getOutput(outputId): Promise<OutputMetadata>

Gets the UTXO outputs associated with the given output id.

| Param    | Type                | Description                    |
| -------- | ------------------- | ------------------------------ |
| outputId | <code>string</code> | The id of the output to search |

**Returns** a promise resolving to the associated [OutputMetadata](#outputmetadata).

#### findOutputs(outputIds, addresses): Promise<OutputMetadata[]>

Gets the UTXO outputs associated with the given output ids and addresses.

| Param     | Type                  | Description                      |
| --------- | --------------------- | -------------------------------- |
| addresses | <code>string[]</code> | The list of addresses to search  |
| outputIds | <code>string[]</code> | The list of output ids to search |

**Returns** a promise resolving to a list of [OutputMetadata](#outputmetadata).

#### getAddressOutputs(address): Promise<string[]>

Gets the UTXO outputs associated with the given address.

| Param   | Type                | Description               |
| ------- | ------------------- | ------------------------- |
| address | <code>string</code> | The address Bech32 string |

**Returns** a promise resolving to a list of output ids.

#### getAddressBalance(address): Promise<number>

Gets the balance of the given address.

| Param   | Type                | Description               |
| ------- | ------------------- | ------------------------- |
| address | <code>string</code> | The address Bech32 string |

#### getMilestone(index): Promise<MilestoneMetadata>

Gets the milestone by the given index.

| Param | Type                | Description                |
| ----- | ------------------- | -------------------------- |
| index | <code>number</code> | The index of the milestone |

**Returns** a promise resolving to the [MilestoneMetadata](#milestonemetadata).

#### getMilestoneUTXOChanges(index): Promise<MilestoneUTXOChanges>

Gets the utxo changes by the given milestone index.

| Param | Type                | Description                |
| ----- | ------------------- | -------------------------- |
| index | <code>number</code> | The index of the milestone |

**Returns** a promise resolving to the [MilestoneUTXOChanges](#MilestoneUTXOChanges).

#### reattach(messageId): Promise<Message>

Reattaches the message associated with the given id.

| Param     | Type                | Description                       |
| --------- | ------------------- | --------------------------------- |
| messageId | <code>string</code> | The id of the message to reattach |

**Returns** A promise resolving to the new [Message](#message) instance.

#### promote(messageId): Promise<Message>

Promotes the message associated with the given id.

| Param     | Type                | Description                      |
| --------- | ------------------- | -------------------------------- |
| messageId | <code>string</code> | The id of the message to promote |

**Returns** A promise resolving to the new [Message](#message) instance.

### NetworkInfo

| Field       | Type                                          | Description                           |
| ----------- | --------------------------------------------- | ------------------------------------- |
| network     | <code>string</code>                           | The network                           |
| networkId   | <code>number</code>                           | The network hashed                    |
| bech32HRP   | <code>string</code>                           | Bech32 HRP for this network           |
| minPowScore | <code>number</code>                           | The network's minimum score for PoW   |
| localPow    | <code>boolean</code>                          | Whether we are using local PoW or not |

### TopicSubscriber

#### topic(topic): TopicSubscriber

Adds a topic to this manager instance.

| Param | Type                | Description  |
| ----- | ------------------- | ------------ |
| topic | <code>string</code> | A MQTT topic |

**Returns** the topic subscriber instance for chained calls.

#### topics(topic): TopicSubscriber

Adds a list of topics to this manager instance.

| Param  | Type                  | Description             |
| ------ | --------------------- | ----------------------- |
| topics | <code>string[]</code> | An array of MQTT topics |

**Returns** the topic subscriber instance for chained calls.

#### subscribe(cb): TopicSubscriber

Subscribe to the provided topics.

| Param | Type                  | Description                                                      |
| ----- | --------------------- | ---------------------------------------------------------------- |
| cb    | <code>function</code> | The topic handler callback in the form of `(err, message) => {}` |

**Returns** the topic subscriber instance for chained calls.

#### unsubscribe(cb: Callback): TopicSubscriber

Unsubscribes from the provided topics.

| Param | Type                  | Description                                                                                |
| ----- | --------------------- | ------------------------------------------------------------------------------------------ |
| cb    | <code>function</code> | A callback executed when the unsubscribe is finished in the form of `(err, message) => {}` |

**Returns** the topic subscriber instance for chained calls.

### MessageSender

Builder to create and submit messages to the Tangle.

#### index(index): MessageSender

Sets the message indexation. This field is required for indexation payloads.

| Param | Type                | Description    |
| ----- | ------------------- | -------------- |
| index | <code>string</code> | The indexation |

**Returns** the message submit instance for chained calls.

#### seed(seed): MessageSender

Sets the transaction account seed. This field is required for transaction payloads.

| Param | Type                | Description                                  |
| ----- | ------------------- | -------------------------------------------- |
| seed  | <code>string</code> | The hex-encoded seed of the account to spend |

**Returns** the message submit instance for chained calls.

#### data(data): MessageSender

Sets the indexation data.

| Param | Type                    | Description        |
| ----- | ----------------------- | ------------------ |
| data  | <code>Uint8Array</code> | The message's data |

**Returns** the message submit instance for chained calls.

#### parents(messageId): MessageSender

Sets 1-8 custom parent message ids.

| Param     | Type                  | Description             |
| --------- | --------------------- | ----------------------- |
| messageId | <code>string[]</code> | The parents message ids |

**Returns** the message submit instance for chained calls.

#### accountIndex(index): MessageSender

Sets the account index. This field is required for transactions.

| Param | Type                | Description       |
| ----- | ------------------- | ----------------- |
| index | <code>number</code> | The account index |

**Returns** the message submit instance for chained calls.

#### input(transactionId, index): MessageSender

Adds an output to the transaction.

| Param         | Type                | Description        |
| ------------- | ------------------- | ------------------ |
| transactionId | <code>string</code> | The transaction id |
| index         | <code>number</code> | The input index    |

**Returns** the message submit instance for chained calls.

#### inputRange(start, end): MessageSender

Defines the range in which to search for addresses fro custom inputs.

| Param         | Type                | Description        |
| ------------- | ------------------- | ------------------ |
| start         | <code>number</code> | The start index |
| end           | <code>number</code> | The end index    |

**Returns** the message submit instance for chained calls.

#### output(address, amount): MessageSender

Adds an output to the transaction.

| Param   | Type                | Description        |
| ------- | ------------------- | ------------------ |
| address | <code>string</code> | The output address |
| amount  | <code>number</code> | The output amount  |

**Returns** the message submit instance for chained calls.

#### dustAllowanceOutput(address, amount): MessageSender

Adds a dust allowance output to the transaction.

| Param   | Type                | Description        |
| ------- | ------------------- | ------------------ |
| address | <code>string</code> | The output address |
| amount  | <code>number</code> | The output amount  |

**Returns** the message submit instance for chained calls.

#### initialAddressIndex(index): MessageSender

Sets the initial address index to search for balance. Defaults to 0 if the function isn't called.

| Param | Type                | Description               |
| ----- | ------------------- | ------------------------- |
| index | <code>number</code> | The initial address index |

**Returns** the message submit instance for chained calls.

#### submit(): Promise<string>

Submits the message.

**Returns** a promise resolving to the message identifier.

### UnspentAddressGetter

Gets a valid unspent address associated with the seed.

#### accountIndex(index): UnspentAddressGetter

Sets the account index. This field is required.

| Param | Type                | Description       |
| ----- | ------------------- | ----------------- |
| index | <code>number</code> | The account index |

**Returns** the address getter instance for chained calls.

#### initialAddressIndex(index): UnspentAddressGetter

Sets the initial address index. Defaults to 0 if the function isn't called.

| Param | Type                | Description               |
| ----- | ------------------- | ------------------------- |
| index | <code>number</code> | The initial address index |

**Returns** the address getter instance for chained calls.

#### get(): Promise<[Address, number]>

Performs the operation.

**Returns** a promise resolving to the [Address](#address) instance and its index.

### AddressGetter

Generates addresses with a given seed.

#### accountIndex(index): AddressGetter

Sets the account index. This field is required.

| Param | Type                | Description       |
| ----- | ------------------- | ----------------- |
| index | <code>number</code> | The account index |

**Returns** the address finder instance for chained calls.

#### range(start, end): AddressGetter

Defines the range of addresses to get. Defaults to `0..20` if the function isn't called.

| Param | Type                | Description             |
| ----- | ------------------- | ----------------------- |
| start | <code>number</code> | The first address index |
| end   | <code>number</code> | The last address index  |

**Returns** the address finder instance for chained calls.

#### get(): Address[]

Performs the operation.

**Returns** an array of [Address](#address) instances.

### BalanceGetter

Gets balance on a given seed.

#### accountIndex(index): BalanceGetter

Sets the account index. This field is required.

| Param | Type                | Description       |
| ----- | ------------------- | ----------------- |
| index | <code>number</code> | The account index |

**Returns** the balance getter instance for chained calls.

#### initialAddressIndex(index): BalanceGetter

Sets the initial address index. Defaults to 0 if the function isn't called.

| Param | Type                | Description               |
| ----- | ------------------- | ------------------------- |
| index | <code>number</code> | The initial address index |

**Returns** the balance getter instance for chained calls.

#### get(): Promise<number>

Performs the operation.

**Returns** a promise resolving to the account balance.

### MessageFinder

Gets a message by indexation key or identifier.

#### initialAddressIndex(index): Promise<string[]>

| Param | Type                | Description        |
| ----- | ------------------- | ------------------ |
| index | <code>string</code> | The indexation key |

Gets a list of message identifiers associated with the given indexation key.

**Returns** a promise resolving to the list of associated ids.

#### data(id): Promise<Message>

Gets the message object associated with the given identifier.

| Param | Type                | Description            |
| ----- | ------------------- | ---------------------- |
| id    | <code>string</code> | The message identifier |

**Returns** a [Message](#message) object.

#### raw(id): Promise<string>

Gets the message raw data.

| Param | Type                | Description            |
| ----- | ------------------- | ---------------------- |
| id    | <code>string</code> | The message identifier |

**Returns** the message raw data as string.

#### children(id): Promise<string[]>

Gets the children of the given message.

| Param | Type                | Description            |
| ----- | ------------------- | ---------------------- |
| id    | <code>string</code> | The message identifier |

**Returns** the list of message ids of the message children.

#### metadata(id): Promise<MessageMetadata>

Gets the metadata of the given message.

| Param | Type                | Description            |
| ----- | ------------------- | ---------------------- |
| id    | <code>string</code> | The message identifier |

**Returns** a [MessageMetadata](#messagemetadata) object.

### BrokerOptions

| Field               | Type                | Description                                                                                           |
| ------------------- | ------------------- | ----------------------------------------------------------------------------------------------------- |
| automaticDisconnect | <code>number</code> | Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not. |
| timeout             | <code>number</code> | MQTT connection timeout in secods                                                                     |

### Address

| Field | Type                   | Description                |
| ----- | ---------------------- | -------------------------- |
| type  | <code>'Ed25519'</code> | Address type               |
| data  | <code>string</code>    | Address as a Bech32 string |

### Message

| Field     | Type                             | Description                                    |
| --------- | -------------------------------- | ---------------------------------------------- |
| networkId | <code>number</code>              | Network identifier                             |
| parents   | <code>string[]</code>              | Message ids of the message references          |
| payload   | <code>[Payload](#payload)</code> | Message payload                                |
| nonce     | <code>number</code>              | Message nonce                                  |

#### Payload

| Field | Type                                                                                                                                                              | Description  |
| ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| data  | <code>{ type: 'Transaction', data: TransactionPayload } \| { type: 'Indexation', data: IndexationPayload } \| { type: 'Milestone', data: MilestonePayload}</code> | Payload data |

##### TransactionPayload

| Field         | Type                                   | Description         |
| ------------- | -------------------------------------- | ------------------- |
| essence       | <code>TransactionPayloadEssence</code> | Transaction essence |
| unlock_blocks | <code>UnlockBlock[]</code>             | Unlock blocks       |

- TransactionPayloadEssence

| Field   | Type                              | Description          |
| ------- | --------------------------------- | -------------------- |
| inputs  | <code>Input[]</code>              | Inputs               |
| outputs | <code>Output[]</code>             | Outputs              |
| payload | <code>Payload \| undefined</code> | Payload for chaining |

- Input

| Field | Type                | Description              |
| ----- | ------------------- | ------------------------ |
| type  | <code>'UTXO'</code> | Input type identifier    |
| data  | <code>string</code> | The associated output id |

- Output

| Field | Type                                             | Description            |
| ----- | ------------------------------------------------ | ---------------------- |
| type  | <code>'SignatureLockedSingle'</code>             | Output type identifier |
| data  | <code>{ address: string, amount: number }</code> | The output definition  |

- UnlockBlock

| Field | Type                                                                           | Description                                           |
| ----- | ------------------------------------------------------------------------------ | ----------------------------------------------------- |
| type  | <code>'Signature' \| 'Reference'</code>                                        | Unlock block type identifier                          |
| data  | <code>WotsSignatureUnlockBlock \| Ed25519SignatureUnlockBlock \| number</code> | Unlock block data (signature type or reference index) |

- WotsSignatureUnlockBlock = number[] (WOTS signature)

- Ed25519SignatureUnlockBlock

| Field      | Type                  | Description        |
| ---------- | --------------------- | ------------------ |
| public_key | <code>number[]</code> | Ed25519 public key |
| signature  | <code>number[]</code> | Ed25519 signature  |

##### IndexationPayload

| Field | Type                  | Description                   |
| ----- | --------------------- | ----------------------------- |
| index | <code>string</code>   | Indexation key                |
| data  | <code>number[]</code> | Indexation data as byte array |

##### MilestonePayload

| Field      | Type                          | Description          |
| ---------- | ----------------------------- | -------------------- |
| essence    | <code>MilestoneEssence</code> | Milestone essence    |
| signatures | <code>number[][]</code>       | Milestone signatures |

- MilestoneEssence

| Field        | Type                    | Description                                               |
| ------------ | ----------------------- | --------------------------------------------------------- |
| index        | <code>number</code>     | Milestone index                                           |
| timestamp    | <code>number</code>     | Timestamp                                                 |
| parents      | <code>string[]</code>   | Message ids of the messages the milestone references      |
| merkle_proof | <code>number[]</code>   | Merkle proof                                              |
| public_keys  | <code>number[][]</code> | public keys                                               |

### MessageDto

| Field   | Type                                   | Description                                                                 |
| ------- | -------------------------------------- | --------------------------------------------------------------------------- |
| parents | <code>string[] \| undefined</code>     | Message ids of the messages it references. `getTips` is used by default     |
| payload | <code>[PayloadDto](#payloaddto)</code> | Message payload                                                             |

#### PayloadDto

| Field | Type                                                       | Description  |
| ----- | ---------------------------------------------------------- | ------------ |
| data  | <code>TransactionPayloadDto \| IndexationPayloadDto</code> | Payload data |

##### TransactionPayloadDto

| Field        | Type                                      | Description         |
| ------------ | ----------------------------------------- | ------------------- |
| essence      | <code>TransactionPayloadEssenceDto</code> | Transaction essence |
| unlockBlocks | <code>UnlockBlockDto[]</code>             | Unlock blocks       |

- TransactionPayloadEssenceDto

| Field   | Type                                 | Description          |
| ------- | ------------------------------------ | -------------------- |
| inputs  | <code>string[]</code>                | Inputs               |
| outputs | <code>Output[]</code>                | Outputs              |
| payload | <code>PayloadDto \| undefined</code> | Payload for chaining |

- OutputDto

| Field   | Type                | Description    |
| ------- | ------------------- | -------------- |
| address | <code>string</code> | Output address |
| amount  | <code>amount</code> | Output amount  |

- UnlockBlockDto

| Field | Type                                                                                 | Description                                           |
| ----- | ------------------------------------------------------------------------------------ | ----------------------------------------------------- |
| data  | <code>WotsSignatureUnlockBlockDto \| Ed25519SignatureUnlockBlockDto \| number</code> | Unlock block data (signature type or reference index) |

- WotsSignatureUnlockBlockDto = number[] (WOTS signature)

- Ed25519SignatureUnlockBlockDto

| Field     | Type                  | Description        |
| --------- | --------------------- | ------------------ |
| publicKey | <code>number[]</code> | Ed25519 public key |
| signature | <code>number[]</code> | Ed25519 signature  |

##### IndexationPayloadDto

| Field | Type                    | Description     |
| ----- | ----------------------- | --------------- |
| index | <code>string</code>     | Indexation key  |
| data  | <code>Uint8Array</code> | Indexation data |

### MessageMetadata

| Field                      | Type                              | Description                                               |
| -------------------------- | --------------------------------- | --------------------------------------------------------- |
| messageId                  | <code>string</code>               | Message identifier                                        |
| parents                    | <code>string[]</code>               | Message id of the messages it references                  |
| isSolid                    | <code>boolean</code>              | Message solid state                                       |
| shouldPromote              | <code>boolean \| undefined</code> | Indicates whether the message should be promoted or not   |
| shouldReattach             | <code>boolean \| undefined</code> | Indicates whether the message should be reattached or not |
| referencedByMilestoneIndex | <code>number \| undefined</code>  | Index of the milestone that references this message       |
| ledgerInclusionState       | <code>string \| undefined</code>  | Ledger inclusion state                                    |

### NodeInfo

| Field                | Type                  | Description                   |
| -------------------- | --------------------- | ----------------------------- |
| name                 | <code>string</code>   | Node name                     |
| version              | <code>string</code>   | Node version                  |
| isHealthy            | <code>boolean</code>  | Node health status            |
| networkId            | <code>string</code>   | Node network identifier       |
| bech32HRP            | <code>string</code>   | Bech32 HRP for this network   |
| latestMilestoneIndex | <code>number</code>   | Index of the latest milestone |
| solidMilestoneIndex  | <code>number</code>   | Index of the solid milestone  |
| pruningIndex         | <code>number</code>   | Pruning index                 |
| features             | <code>string[]</code> | List of node features         |

### OutputMetadata

| Field         | Type                 | Description                                      |
| ------------- | -------------------- | ------------------------------------------------ |
| messageId     | <code>string</code>  | Id of the message associated with the output     |
| transactionId | <code>string</code>  | Id of the transaction associated with the output |
| outputIndex   | <code>number</code>  | Output index                                     |
| isSpent       | <code>boolean</code> | Output spent state                               |
| address       | <code>string</code>  | Output address                                   |
| amount        | <code>number</code>  | Output amount                                    |

### MilestoneMetadata

| Field          | Type                | Description                                     |
| -------------- | ------------------- | ----------------------------------------------- |
| milestoneIndex | <code>number</code> | Milestone index                                 |
| messageId      | <code>string</code> | Id of the message associated with the milestone |
| timestamp      | <code>number</code> | Milestone timestamp                             |

### MilestoneUTXOChanges

| Field           | Type                  | Description                        |
| --------------- | --------------------- | ---------------------------------- |
| index           | <code>number</code>   | Milestone index                    |
| createdOutputs  | <code>string[]</code> | OutputIds from new created outputs |
| consumedOutputs | <code>string[]</code> | OutputIds from consumed outputs    |
