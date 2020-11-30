# IOTA Client Library - Node.js binding

Node.js binding to the IOTA client library.

## Installation

Currently the package isn't published so you'd need to link it to your project using `npm` or `yarn`.

- Using NPM:
```
$ git clone https://github.com/iotaledger/iota.rs
$ cd iota.rs/bindings/node
$ npm link
$ cd /path/to/nodejs/project/
$ npm link iota-client
```
- Using yarn: 
```
$ git clone https://github.com/iotaledger/iota.rs
$ cd iota.rs/bindings/node
$ yarn link
$ cd /path/to/nodejs/project/
$ yarn link iota-client
```

## Getting Started

After you linked the library, you can create a `Client` instance and interface with it.

```javascript
const { ClientBuilder } = require('iota-wallet')
const client = new ClientBuilder()
  .node('http://localhost:14265')
  .build()
client.getTips().then(console.log).catch(console.error)
```

## API Reference

### ClientBuilder

#### node(url): ClientBuilder

Adds an IOTA node to the client pool.

| Param | Type                | Default                | Description |
| ----- | ------------------- | ---------------------- | ----------- |
| url   | <code>string</code> | <code>undefined</code> | A node URL  |

**Returns** the client builder instance for chained calls.

#### nodes(urls): ClientBuilder

Adds a list of IOTA nodes to the client pool.

| Param | Type                  | Default                | Description           |
| ----- | --------------------- | ---------------------- | --------------------- |
| url   | <code>string[]</code> | <code>undefined</code> | An array of node URLs |

**Returns** the client builder instance for chained calls.

#### quorumSize(size): ClientBuilder

Defines how many of nodes will be queried at the same time to check for quorum.

| Param | Type                | Default                | Description                              |
| ----- | ------------------- | ---------------------- | ---------------------------------------- |
| size  | <code>number</code> | <code>undefined</code> | The number of nodes that will be queried |

**Returns** the client builder instance for chained calls.

#### quorumThreshold(threshold): ClientBuilder

Defines the minimum amount of nodes from the quorum pool that need to agree if we want to consider the result true.

| Param     | Type                | Default                | Description             |
| --------- | ------------------- | ---------------------- | ----------------------- |
| threshold | <code>number</code> | <code>undefined</code> | Minimum amount of nodes |

**Returns** the client builder instance for chained calls.

#### brokerOptions(options): ClientBuilder

Sets the options for the MQTT connection with the node.

| Param   | Type                                         | Default                | Description             |
| ------- | -------------------------------------------- | ---------------------- | ----------------------- |
| options | <code>[BrokerOptions](#brokeroptions)</code> | <code>undefined</code> | The MQTT broker options |

**Returns** the client builder instance for chained calls.

#### build(): Client

Builds the client instance.

**Returns** a [Client](#client) instance.

### Client

#### subscriber(): TopicSubscriber

Gets a handle to the MQTT topic subscriber.

**Returns** a [TopicSubscriber](#topicsubscriber) instance.

#### send(seed): ValueTransactionSender

Initiates the builder to send funds.

| Param | Type                | Default                | Description                      |
| ----- | ------------------- | ---------------------- | -------------------------------- |
| seed  | <code>string</code> | <code>undefined</code> | The seed of the account to spend |

**Returns** a [ValueTransactionSender](#valuetransactionsender) instance.

#### getUnspentAddress(seed): UnspentAddressGetter

Get a valid unspent address.

| Param | Type                | Default                | Description        |
| ----- | ------------------- | ---------------------- | ------------------ |
| seed  | <code>string</code> | <code>undefined</code> | The seed to search |

**Returns** a [UnspentAddressGetter](#unspentaddressgetter) instance.

#### findAddresses(seed): AddressFinder

Find addresses from the seed regardless of their validity.

| Param | Type                | Default                | Description        |
| ----- | ------------------- | ---------------------- | ------------------ |
| seed  | <code>string</code> | <code>undefined</code> | The seed to search |

**Returns** a [AddressFinder](#addressfinder) instance.

#### findMessages(indexationKeys, messageIds): Promise<Message[]>

Finds all messages associated with the given indexation keys and message ids.

| Param          | Type                  | Default                | Description                             |
| -------------- | --------------------- | ---------------------- | --------------------------------------- |
| indexationKeys | <code>string[]</code> | <code>undefined</code> | The list of indexations keys too search |
| messageIds     | <code>string[]</code> | <code>undefined</code> | The list of message ids to search       |

**Returns** a promise resolving to the list of the found messages.

#### getBalance(seed: string): BalanceGetter

Get balance on a given seed and its wallet chain BIP32 path.

| Param | Type                | Default                | Description        |
| ----- | ------------------- | ---------------------- | ------------------ |
| seed  | <code>string</code> | <code>undefined</code> | The seed to search |

**Returns** a [BalanceGetter](#balancegetter) instance.

#### getAddressBalances(addresses): Promise<AddressBalance[]>

Get the balance in iotas for the given addresses.

| Param     | Type                  | Default                | Description                     |
| --------- | --------------------- | ---------------------- | ------------------------------- |
| addresses | <code>string[]</code> | <code>undefined</code> | The list of addresses to search |

**Returns** A promise resolving to the list of `{ address, balance }` pairs.

#### retry(messageId: string): Promise<Message>

Retries (promotes or reattaches) the message associated with the given id.

| Param     | Type                | Default                | Description                    |
| --------- | ------------------- | ---------------------- | ------------------------------ |
| messageId | <code>string</code> | <code>undefined</code> | The id of the message to retry |

**Returns** A promise resolving to the new [Message](#message) instance.

#### getInfo(): Promise<NodeInfo>

Gets information about the node.

**Returns** a promise resolving to the [NodeInfo](#nodeinfo) object.

#### getTips(): Promise<[string, string]>

Gets two non-lazy tips.

**Returns** a promise resolving to an array of length 2 containing the message ids of the tips.

#### postMessage(message): Promise<string>

Submits a message.

| Param   | Type                             | Default                | Description           |
| ------- | -------------------------------- | ---------------------- | --------------------- |
| message | <code>[Message](#message)</code> | <code>undefined</code> | The message to submit |

**Returns** the message identifier.

#### getMessage(): MessageFinder

Gets a message from its identifier.

**Returns** an instance of the [MessageFinder](#messagefinder) for choices of response.

#### getOutput(outputId): Promise<OutputMetadata>

Gets the UTXO outputs associated with the given output id.

| Param    | Type                | Default                | Description                    |
| -------- | ------------------- | ---------------------- | ------------------------------ |
| outputId | <code>string</code> | <code>undefined</code> | The id of the output to search |

**Returns** a promise resolving to the associated [OutputMetadata](#outputmetadata).

#### findOutputs(outputIds, addresses): Promise<OutputMetadata[]>

Gets the UTXO outputs associated with the given output ids and addresses.

| Param     | Type                  | Default                | Description                      |
| --------- | --------------------- | ---------------------- | -------------------------------- |
| addresses | <code>string[]</code> | <code>undefined</code> | The list of addresses to search  |
| outputIds | <code>string[]</code> | <code>undefined</code> | The list of output ids to search |

**Returns** a promise resolving to a list of [OutputMetadata](#outputmetadata).

#### getAddressOutputs(address): Promise<string[]>

Gets the UTXO outputs associated with the given address.

| Param   | Type                | Default                | Description               |
| ------- | ------------------- | ---------------------- | ------------------------- |
| address | <code>string</code> | <code>undefined</code> | The address Bech32 string |

**Returns** a promise resolving to a list of output ids.

#### getAddressBalance(address): Promise<number>

Gets the balance of the given address.

| Param   | Type                | Default                | Description               |
| ------- | ------------------- | ---------------------- | ------------------------- |
| address | <code>string</code> | <code>undefined</code> | The address Bech32 string |

#### getMilestone(index): Promise<MilestoneMetadata>

Gets the milestone by the given index.

| Param | Type                | Default                | Description                |
| ----- | ------------------- | ---------------------- | -------------------------- |
| index | <code>number</code> | <code>undefined</code> | The index of the milestone |

**Returns** a promise resolving to the [MilestoneMetadata](#milestonemetadata).

#### reattach(messageId): Promise<Message>

Reattaches the message associated with the given id.

| Param     | Type                | Default                | Description                       |
| --------- | ------------------- | ---------------------- | --------------------------------- |
| messageId | <code>string</code> | <code>undefined</code> | The id of the message to reattach |

**Returns** A promise resolving to the new [Message](#message) instance.

#### promote(messageId): Promise<Message>

Promotes the message associated with the given id.

| Param     | Type                | Default                | Description                      |
| --------- | ------------------- | ---------------------- | -------------------------------- |
| messageId | <code>string</code> | <code>undefined</code> | The id of the message to promote |

**Returns** A promise resolving to the new [Message](#message) instance.

### TopicSubscriber

#### topic(topic): TopicSubscriber

Adds a topic to this manager instance.

| Param | Type                | Default                | Description  |
| ----- | ------------------- | ---------------------- | ------------ |
| topic | <code>string</code> | <code>undefined</code> | A MQTT topic |

**Returns** the topic subscriber instance for chained calls.

#### topics(topic): TopicSubscriber

Adds a list of topics to this manager instance.

| Param  | Type                  | Default                | Description             |
| ------ | --------------------- | ---------------------- | ----------------------- |
| topics | <code>string[]</code> | <code>undefined</code> | An array of MQTT topics |

**Returns** the topic subscriber instance for chained calls.

#### subscribe(cb): TopicSubscriber

Subscribe to the provided topics.

| Param | Type                  | Default                | Description                                                      |
| ----- | --------------------- | ---------------------- | ---------------------------------------------------------------- |
| cb    | <code>function</code> | <code>undefined</code> | The topic handler callback in the form of `(err, message) => {}` |

**Returns** the topic subscriber instance for chained calls.

#### unsubscribe(cb: Callback): TopicSubscriber

Unsubscribes from the provided topics.

| Param | Type                  | Default                | Description                                                                                |
| ----- | --------------------- | ---------------------- | ------------------------------------------------------------------------------------------ |
| cb    | <code>function</code> | <code>undefined</code> | A callback executed when the unsubscribe is finished in the form of `(err, message) => {}` |

**Returns** the topic subscriber instance for chained calls.

### ValueTransactionSender

Submits a value transaction message.

#### path(path): ValueTransactionSender

Sets the account BIP32 path. This field is required.

| Param | Type                | Default                | Description            |
| ----- | ------------------- | ---------------------- | ---------------------- |
| path  | <code>string</code> | <code>undefined</code> | The account BIP32 path |

**Returns** the message submit instance for chained calls.

#### output(address, amount): ValueTransactionSender

Adds an output to the transaction.

| Param   | Type                | Default                | Description        |
| ------- | ------------------- | ---------------------- | ------------------ |
| address | <code>string</code> | <code>undefined</code> | The output address |
| amount  | <code>number</code> | <code>undefined</code> | The output amount  |

**Returns** the message submit instance for chained calls.

#### index(index): ValueTransactionSender

Sets the initial address index to search for balance. Defaults to 0 if the function isn't called.

| Param | Type                | Default                | Description               |
| ----- | ------------------- | ---------------------- | ------------------------- |
| index | <code>number</code> | <code>undefined</code> | The initial address index |

**Returns** the message submit instance for chained calls.

#### submit(): Promise<string>

Submits the message.

**Returns** a promise resolving to the message identifier.

### UnspentAddressGetter

Gets a valid unspent address associated with the seed.

#### path(path): UnspentAddressGetter

Sets the account BIP32 path. This field is required.

| Param | Type                | Default                | Description            |
| ----- | ------------------- | ---------------------- | ---------------------- |
| path  | <code>string</code> | <code>undefined</code> | The account BIP32 path |

**Returns** the address getter instance for chained calls.

#### index(index): UnspentAddressGetter

Sets the initial address index. Defaults to 0 if the function isn't called.

| Param | Type                | Default                | Description               |
| ----- | ------------------- | ---------------------- | ------------------------- |
| index | <code>number</code> | <code>undefined</code> | The initial address index |

**Returns** the address getter instance for chained calls.

#### get(): Promise<[Address, number]>

Performs the operation.

**Returns** a promise resolving to the [Address](#address) instance and its index.

### AddressFinder

Finds addresses on a given seed.

#### path(path): AddressFinder

Sets the account BIP32 path. This field is required.

| Param | Type                | Default                | Description            |
| ----- | ------------------- | ---------------------- | ---------------------- |
| path  | <code>string</code> | <code>undefined</code> | The account BIP32 path |

**Returns** the address finder instance for chained calls.

#### range(start, end): AddressFinder

Defines the range of addresses to get. Defaults to `0..20` if the function isn't called.

| Param | Type                | Default                | Description             |
| ----- | ------------------- | ---------------------- | ----------------------- |
| start | <code>number</code> | <code>undefined</code> | The first address index |
| end   | <code>number</code> | <code>undefined</code> | The last address index  |

**Returns** the address finder instance for chained calls.

#### get(): Address[]

Performs the operation.

**Returns** an array of [Address](#address) instances.

### BalanceGetter

Gets balance on a given seed.

#### path(path): BalanceGetter

Sets the account BIP32 path. This field is required.

| Param | Type                | Default                | Description            |
| ----- | ------------------- | ---------------------- | ---------------------- |
| path  | <code>string</code> | <code>undefined</code> | The account BIP32 path |

**Returns** the balance getter instance for chained calls.

#### index(index): BalanceGetter

Sets the initial address index. Defaults to 0 if the function isn't called.

| Param | Type                | Default                | Description               |
| ----- | ------------------- | ---------------------- | ------------------------- |
| index | <code>number</code> | <code>undefined</code> | The initial address index |

**Returns** the balance getter instance for chained calls.

#### get(): Promise<number>

Performs the operation.

**Returns** a promise resolving to the account balance.

### MessageFinder

Gets a message by indexation key or identifier.

#### index(index): Promise<string[]>

| Param | Type                | Default                | Description        |
| ----- | ------------------- | ---------------------- | ------------------ |
| index | <code>string</code> | <code>undefined</code> | The indexation key |

Gets a list of message identifiers associated with the given indexation key.

**Returns** a promise resolving to the list of associated ids.

#### data(id): Promise<Message>

Gets the message object associated with the given identifier.

| Param | Type                | Default                | Description            |
| ----- | ------------------- | ---------------------- | ---------------------- |
| id    | <code>string</code> | <code>undefined</code> | The message identifier |

**Returns** a [Message](#message) object.

#### raw(id): Promise<string>

Gets the message raw data.

| Param | Type                | Default                | Description            |
| ----- | ------------------- | ---------------------- | ---------------------- |
| id    | <code>string</code> | <code>undefined</code> | The message identifier |

**Returns** the message raw data as string.

#### children(id): Promise<string[]>

Gets the children of the given message.

| Param | Type                | Default                | Description            |
| ----- | ------------------- | ---------------------- | ---------------------- |
| id    | <code>string</code> | <code>undefined</code> | The message identifier |

**Returns** the list of message ids of the message children.

#### metadata(id): Promise<MessageMetadata>

Gets the metadata of the given message.

| Param | Type                | Default                | Description            |
| ----- | ------------------- | ---------------------- | ---------------------- |
| id    | <code>string</code> | <code>undefined</code> | The message identifier |

**Returns** a [MessageMetadata](#messagemetadata) object.

### BrokerOptions

| Field               | Type                | Description                                                                                           |
| ------------------- | ------------------- | ----------------------------------------------------------------------------------------------------- |
| automaticDisconnect | <code>number</code> | Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not. |
| timeout             | <code>number</code> | MQTT connection timeout in secods                                                                     |

### Address

| Field | Type                                    | Description                |
| ----- | --------------------------------------- | -------------------------- |
| type  | <code>'Wots'        \| 'Ed25519'</code> | Address type               |
| data  | <code>string</code>                     | Address as a Bech32 string |

### Message

| Field     | Type                             | Description                                    |
| --------- | -------------------------------- | ---------------------------------------------- |
| networkId | <code>number</code>              | Network identifier                             |
| parent1   | <code>string</code>              | Message id of the first message it references  |
| parent2   | <code>string</code>              | Message id of the second message it references |
| payload   | <code>[Payload](#payload)</code> | Message payload                                |
| nonce     | <code>number</code>              | Message nonce                                  |

### MessageMetadata

| Field                      | Type                              | Description                                               |
| -------------------------- | --------------------------------- | --------------------------------------------------------- |
| messageId                  | <code>string</code>               | Message identifier                                        |
| parent1                    | <code>string</code>               | Message id of the first message it references             |
| parent2                    | <code>string</code>               | Message id of the second message it references            |
| isSolid                    | <code>boolean</code>              | Message solid state                                       |
| shouldPromote              | <code>boolean \| undefined</code> | Indicates whether the message should be promoted or not   |
| shouldReattach             | <code>boolean \| undefined</code> | Indicates whether the message should be reattached or not |
| referencedByMilestoneIndex | <code>number \| undefined</code>  | Index of the milestone that references this message       |
| ledgerInclusionState       | <code>string \| undefined</code>  | Ledger inclusion state                                    |

### NodeInfo

| Field                    | Type                  | Description                        |
| ------------------------ | --------------------- | ---------------------------------- |
| name                     | <code>string</code>   | Node name                          |
| version                  | <code>string</code>   | Node version                       |
| isHealthy                | <code>boolean</code>  | Node health status                 |
| coordinatorPublicKey     | <code>string</code>   | Public key of the node coordinator |
| latestMilestoneMessageId | <code>string</code>   | Message id of the latest milestone |
| latestMilestoneIndex     | <code>number</code>   | Index of the latest milestone      |
| solidMilestoneMessageId  | <code>string</code>   | Message id of the solid milestone  |
| solidMilestoneIndex      | <code>number</code>   | Index of the solid milestone       |
| pruningIndex             | <code>number</code>   | Pruning index                      |
| features                 | <code>string[]</code> | List of node features              |

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
