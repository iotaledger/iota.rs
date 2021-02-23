

# High Level API Specification

## Table of Contents

* [Introduction](#Introduction)
* [Builder](#Builder)
* [General high level API](#General-high-level-API)
  * [`message`](#message)
  * [`get_message`](#get_message)
  * [`find_messages`](#find_messages)
  * [`get_unspent_address`](#get_unspent_address)
  * [`get_addresses`](#get_addresses)
  * [`get_balance`](#get_balance)
  * [`get_address_balances`](#get_address_balances)
  * [`subscriber`](#subscriber)
  * [`retry`](#retry)
  * [`reattach`](#reattach)
  * [`promote`](#promote)
* [Full node API](#Full-node-API)
  * [`get_health`](#get_health)
  * [`get_health`](#get_peers)
  * [`get_info`](#get_info)
  * [`get_tips`](#get_tips)
  * [`post_message`](#post_message)
  * [`get_output`](#get_output)
  * [`get_address`](#get_address)
  * [`find_outputs`](#find_outputs)
  * [`get_milestone`](#get_milestone)
  * [`get_milestone_utxo_changes`](#get_milestone_utxo_changes)
* [Objects](#Objects)
  * [Network]
  * [Seed]
  * [Message]
  * [Payload]
  * [Output]
  * [Address]
  * [AddressBalancePair]
  * [Milestone]
  * [API]
  * [BrokerOptions]
  * [Topic]

# Introduction

This document specifies a user friendly API to be used in the client libraries. The main implementation will be in Rust which will receive automatically compiled client libraries in other languages via C or Webassembly bindings. There are also many crates to support developers creating foreign function interfaces with native bindings.

# Builder

The data structure to initialize the instance of the Higher level client library. This is always called first when starting a new interaction with the library. Note: This is the common approach to do initialization in Rust. Different languages might use different methods such as just calling an initialization function directly.

### Parameters

| Parameter | Required | Default Value | Type | Definition |
| - | - | - | - | - |
| **network** | ✘ | Testnet | &str | Optional, the network type can be "testnet" or "mainnet". If no node url is provided, some default nodes are used for the specified network. Nodes that aren't in this network will be ignored. |
| **node** | ✘ | None | &str | The URL of a node to connect to; format: `https://node:port` |
| **nodes** | ✘ | None | &[&str] | A list of nodes to connect to; nodes are added with the `https://node:port` format. The amount of nodes specified in quorum_size are randomly selected from this node list to check for quorum based on the quorum threshold. If quorum_size is not given the full list of nodes is checked. |
| **node_sync_interval** | ✘ | Duration::from_secs(60) | std::time::Duration | The interval in milliseconds to check for node health and sync |
| **node_sync_disabled** | ✘ | false | bool | If disabled also unhealty nodes will be used |
| **node_pool_urls** | None | ✘ | &[String] | A list of node_pool_urls from which nodes are added. The amount of nodes specified in quorum_size are randomly selected from this node list to check for quorum based on the quorum threshold. If quorum_size is not given the full list of nodes is checked. |
| **request_timeout** | ✘ | Duration::from_secs(30) | std::time::Duration | The amount of seconds a request can be outstanding to a node before it's considered timed out |
| **api_timeout** | ✘ | Api::GetInfo: Duration::from_secs(2)),<br /> Api::GetHealth: Duration::from_secs(2),<br />Api::GetPeers: Duration::from_secs(2),<br />Api::GetMilestone: Duration::from_secs(2),<br />Api::GetTips: Duration::from_secs(2),<br />Api::PostMessage: Duration::from_secs(2),<br />Api::PostMessageWithRemotePow: Duration::from_secs(30),<br />Api::GetOutput: Duration::from_secs(2) | HashMap<[Api],<br /> std::time::Duration> | The amount of milliseconds a request to a specific Api endpoint can be outstanding to a node before it's considered timed out. |
| **local_pow** | ✘ | True | bool | If not defined it defaults to local PoW to offload node load times |
| **tips_interval** | ✘ | 15 | u64 | Time interval during PoW when new tips get requested. |
| **mqtt_broker_options** | ✘ | True,<br />Duration::from_secs(30),<br />True | [BrokerOptions] | If not defined the default values will be used, use_ws: false will try to connect over tcp|

* Note that there must be at least one node to build the instance successfully.

### Return

Finalize the builder with `finish()` will run the instance in the background. Users don’t need to worry about the return object handling.

## On initialization

On initialisation, call getNodeInfo API. Check the health of each node in the node list, and put healty nodes, matching the PoW settings and network in a synced nodelist.

| Node metadata | Description |
| - | - |
| network | If this parameter does not match the global builder parameter, don't add it to the synced nodelist. |
| pow | If the global local_pow parameter is set to false, then put only nodes with the PoW feature in the synced nodelist. |

## Sync Process

When a `Client` instance (The instance which is used for calling the client APIs) is built, the status of each node listed is checked. If the returned status of the node information is healthy, which means the node is synced, then this node will be pushed into a `synced_nodes` list. The rust-like pseudo code of `synced_nodes` construction process follows. The process of syncing a node is repeated every 60 seconds or at the interval specified in the `node_sync_interval` argument of the initializer if set.

```rust
synced_nodes = Vec::new()
for node in node_pool_urls{
   status = Client.get_info(node).await?;
   if status == healthy{
      synced_nodes.push(node)
   }
}
```

# `General high level API`

Here is the high level abstraction API collection with sensible default values for users easy to use.

## `message()`

A generic send function for easily sending a message.

### Parameters

| Parameter | Required | Default | Type | Definition |
| - | - | - | - | - |
| **seed** | ✘ | None | [Seed] | The seed of the account we are going to spend, only needed for transactions |
| **account_index** | ✘ | 0 | usize | The account index, responsible for the value `✘` in the Bip32Path `m/44'/4218'/✘'/0'/0'`. |
| **initial_address_index** | ✘ | 0 | usize | The index from where to start looking for balance. Responsible for the value `✘` in the Bip32Path `m/44'/4218'/0'/0'/✘'`. |
| **input** | ✘ | None | UTXOInput | Users can manually select their UTXOInputs instead of having automatically selected inputs. |
| **input_range** | ✘ | 0..100 | Range | Custom range to search for the input addresses if custom inputs are provided. |
| **output** | ✘ | None | address: &[Bech32Address],<br />amount: u64 | Address to send to and amount to send. Address needs to be Bech32 encoded. |
| **output_hex** | ✘ | None | address: &str,<br />amount: u64 | Address to send to and amount to send. Address needs to be hex encoded. |
| **index** | ✘ | None | &[u8] / &str | An optional indexation key for an indexation payload. 1-64 bytes long. |
| **data** | ✘ | None | Vec<u8> | Optional data for the indexation payload. |
| **parents** | ✘ | None | [MessageId] | 1-8 optional parents [MessageId] to be used. |

Depending on the provided values this function will create a message with:

* no payload
* an indexation payload
* a transaction payload
* a transaction payload containing an indexation payload

### Return

The [Message] object we build.

### Implementation Details

* Validate inputs, such as address and seed to check if they are correct.
* Check if account balance is bigger or equal to the value using method similar to [`get_balance()`](#get_balance);
* Build and validate the message with signed transaction payload accordingly;
* Get tips using [`get_tips()`](#get_tips);
* Perform proof-of-work locally (if not set to remote);
* Send the message using [`post_messages()`](#post_messages);

## `get_message()`

(`GET /api/v1/messages`)

Endpoint collection all about GET messages.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **message_id** | ✔ | [MessageId] | The identifier of message. |
| **index** | ✔ | &[u8] / &str | An indexation key. |

### Returns

Depend on the final calling method, users could get different results they need:

- `metadata(&MessageId)`: Return [MessageMetadata](#MessageMetadata) of the message.
- `data(&MessageId)`: Return a [Message] object.
- `raw(&MessageId)`: Return the raw data of given message.
- `children(&MessageId)`: Return the list of [MessageId]s that reference a message by its identifier.
- `index(&[u8] | &str)` : Return the list of [MessageId]s that have this str as indexation key

## `find_messages()`

Find all messages by provided message IDs.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **indexation_keys** | ✘ | [&[u8] / &str] | The index key of the indexation payload. |
| **message_ids** | ✘ | [[MessageId]] | The identifier of message. |

### Returns

A vector of [Message] Object.

## `get_unspent_address()`

Return a valid unspent public Bech32 encoded address.

### Parameters

| Parameter | Required | Default | Type | Definition |
| - | - | - | - | - |
| **seed** | ✔ | - | [Seed] | The seed we want to use. |
| **account_index** | ✘ | 0 | usize | The account index, responsible for the value `✘` in the Bip32Path `m/44'/4218'/✘'/0'/0'`. |
| **initial_address_index** | ✘ | 0 | usize | Start index of the addresses to search. Responsible for the value `✘` in the Bip32Path `m/44'/4218'/0'/0'/✘'`. |

### Return

Return a tuple with type of `(Bech32Address, usize)` as the address and corresponding index in the account.

### Implementation Details

Following are the steps for implementing this method:

* Start generating addresses with given account index and starting index. We will have a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20 at a time;
* Check for balances on the generated addresses using [`find_outputs()`](#find_outputs) and keep track of the positive balances;
* Repeat the above step till there's an unspent address found;
* Return the address with corresponding index on the wallet chain;

## `get_addresses()`

Return a list of addresses from the seed regardless of their validity.

### Parameters

| Parameter | Required | Default | Type | Definition |
| - | - | - | - | - |
| **seed** | ✔ | None | [Seed] | The seed we want to search for. |
| **account_index** | ✘ | 0 | usize | The account index, responsible for the value `✘` in the Bip32Path `m/44'/4218'/✘'/0'/0'`. |
| **range** | ✘ | None | std::ops::Range | Range indices of the addresses we want to search for. Default is (0..20) |
| **get_all** | ✘ | ✘ | ✘ | Get public and [change addresses](https://bitcoin.stackexchange.com/questions/75033/bip44-and-change-addresses). Will return Vec<([Bech32Address], bool)>, where the bool is indicating whether it's a change address|

### Return

Vec<[Bech32Address]>, with the public addresses

## `get_balance()`

Return the balance for a provided seed and its wallet account index.

### Parameters

| Parameter | Required | Default | Type | Definition |
| - | - | - | - | - |
| **seed** | ✔ | - | [Seed] | The seed we want to search for. |
| **account_index** | ✘ | 0 | usize | The account index, responsible for the value `✘` in the Bip32Path `m/44'/4218'/✘'/0'/0'`. |
| **initial_address_index** | ✘ | 0 | usize | Start index from which to generate addresses. Default is 0. Responsible for the value `✘` in the Bip32Path `m/44'/4218'/0'/0'/✘'`. |

### Return

Total account balance.

### Implementation Details

Following are the steps for implementing this method:

* Start generating addresses with given wallet account index and starting index. We will have a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20 at a time;
* Check for balances on the generated addresses using [`find_outputs()`](#find_outputs) and keep track of the positive balances;
* Repeat the above step till an address of zero balance is found;
* Accumulate the positive balances and return the result.

## `get_address_balances()`

Return the balance in iota for the given addresses; No seed or security level needed to do this since we are only checking and already know the addresses.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **addresses** | ✔ | [[Bech32Address]] | List of Bech32 encoded addresses. |

### Return

A list of tuples with value of [AddressBalancePair]. The usize is the balance of the address accordingly.

### Implementation details:

Following are the steps for implementing this method:

* Validate _address_ semantics;
* Get latest balance for the provided address using [`find_outputs()`](#find_outputs) with addresses as parameter;
* Return the list of Output which contains corresponding pairs of address and balance.

## `subscriber()`

Subscribe to a node event [Topic] (MQTT)

Required: one of

* `topic()`: Add a new [Topic] to the list.
* `topics()`: Add a vector of [Topic] to the list.

* `subscribe()`: Subscribe to the given topics with the callback, which will be called every time when the topic is detected.
* `unsubscribe()`: Unsubscribes from all subscriptions.
* `disconnect()`: Disconnects the broker. This will clear the stored topic handlers and close the MQTT connection.

### Returns

Nothing apart from a Ok(()) result if successful

## `retry()`

Retries (promotes or reattaches) a message for provided [MessageId] if the node suggests it. The need to use this function should be low, because the confirmation throughput of the node is expected to be quite high.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **message_id** | ✔ | [MessageId] | The identifier of message. |

### Returns:

A tuple with the newly promoted or reattached `(MessageId,  Message)`.

### Implementation Details

Following are the steps for implementing this method:

* Only unconfirmed messages should be allowed to retry. The method should validate the confirmation state of the provided messages. If a message id of a confirmed message is provided, the method should error out;
* The method should also validate if a retry is necessary. This can be done by leveraging the `/messages/{messageId}/metadata` endpoint (already available through [get_message](#get_message)). See [this](https://github.com/iotaledger/trinity-wallet/blob/develop/src/shared/libs/iota/transfers.js#L105-L131) implementation for reference;
* Use [reattach](#reattach) or [promote](#promote) accordingly.

## `reattach()`

Depends on [find_messages](#find_messages), [get_message](#get_message) and [post_message](#post_message).

Reattaches a message. The method should validate if a reattachment is necessary through [get_message](#get_message). If not, the method should error out and should not allow unnecessary reattachments.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **message_id** | ✔ | [MessageId] | The identifier of message. |

### Returns

A tuple with the newly reattached `(MessageId,  Message)`.

## `promote()`

Depends on [find_messages](#find_messages), [get_message](#get_message) and [post_message](#post_message).

Promotes a message. The method should validate if a promotion is necessary through [get_message](#get_message). If not, the method should error out and should not allow unnecessary promotions.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **message_id** | ✔ | [MessageId] | The identifier of message. |

### Returns

A tuple with the newly promoted `(MessageId,  Message)`.

# Full node API

Full node API of Bee and HORNET will still be public. Users who know these relative low level Restful API can still call them directly if they are confident and think it’s good for them. Note that both Bee and HORNET haven't finalized their APIs either. Following items and signatures might change later.

## `get_health()`

(`GET /health`)

Returns the health of the node, which can be used for load-balancing or uptime monitoring.

### Parameters

None

### Returns

Boolean to indicate if node is healthy.

## `get_peers()`

(`GET /peers`)

Get information about the peers of the node.

### Parameters

None

### Returns

```Rust
pub struct PeerDto {
    pub id: String,
    #[serde(rename = "multiAddresses")]
    pub multi_addresses: Vec<String>,
    pub alias: Option<String>,
    pub relation: RelationDto,
    pub connected: bool,
    pub gossip: Option<GossipDto>,
}
```

## `get_info()`

(`GET /api/v1/info`)

Returns information about the node.

### Parameters

None

### Returns

A Response Object similar to this:

```rust
pub struct NodeInfo {
    pub name: String,
    pub version: String,
    pub is_healthy: bool,
    pub network_id: String,
    pub latest_milestone_index: usize,
    pub min_pow_score: f64,
    pub solid_milestone_index: usize,
    pub pruning_index: usize,
    pub features: Vec<String>,
}
```

## `get_tips()`

(`GET /tips`)

Returns two non-lazy tips. In case the node can only provide one tip, tip1 and tip2 are identical.

### Parameters

None

### Returns

A tuple with two [MessageId]:

```rust
(MessageId, MessageId)
```

## `post_message()`

(`POST /message`)

Submit a message. The node takes care of missing fields and tries to build the message. On success, the message will be stored in the Tangle. This endpoint will return the identifier of the message.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **message** | ✔ | [Message] | The message object. |

### Returns

The [MessageId] of the message object.

## `get_output()`

(`GET /outputs`)

Get the producer of the output, the corresponding address, amount and spend status of an output. This information can only be retrieved for outputs which are part of a confirmed transaction.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **output_id** | ✔ | UTXOInput | Identifier of the output. |

### Returns

An [OutputMetadata](#OutputMetadata) that contains various information about the output.

## `get_address()`

(`GET /addresses`)

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **address** | ✔ | [Bech32Address] | The address to search for. |

### Returns

Depend on the final calling method, users could get different outputs they need:

* `balance()`: Return confirmed balance of the address.
* `outputs()`: Return UTXOInput array (transaction IDs with corresponding output index).

## `find_outputs()`

Find all outputs based on the requests criteria.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **output_id** | ✘ | [UTXOInput] | The identifier of output. |
| **addresses** | ✘ | [[Bech32Address]] | The Bech32 encoded address. |

### Returns

A vector of [OutputMetadata](#OutputMetadata).

## `get_milestone()`

(`GET /milestones`)

Get the milestone by the given index.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **index** | ✔ | u32 | Index of the milestone. |

### Returns

An [Milestone] object.

## `get_milestone_utxo_changes()`

(`GET /milestones/{}/utxo-changes`)

Get all UTXO changes of a given milestone.

### Parameters

| Parameter | Required | Type | Definition |
| - | - | - | - |
| **index** | ✔ | u32 | Index of the milestone. |

### Returns

```Rust
MilestoneUTXOChanges {
    index: 1,
    created_outputs: [],
    consumed_outputs: [],
}
````

# Objects

Here are the objects used in the API above. They aim to provide a secure way to handle certain data structures specified in the Iota stack.

## `MessageId`

[MessageId]: #MessageId

MessageId is a 32 bytes array which can represent as hex string.

```rust
struct MessageId([u8; MESSAGE_ID_LENGTH]);
```

## `Seed`

[Seed]: #Seed

```Rust
pub enum Seed {
    /// Ed25519 variant
    Ed25519(Ed25519Seed)
}
```

An IOTA seed that inner structure is omitted. Users can create this type by passing a String. It will verify and return an error if it’s not valid. |

## `Message`

[Message]: #Message

The message object returned by various functions; based on the [RFC](https://github.com/GalRogozinski/protocol-rfcs/blob/message/text/0017-message/0017-message.md) for the Message object. Here's the brief overview of each components in Message type would look like:

```rust
struct Message {
    parents: Vec<MessageId>,
    payload: Option<Payload>,
    nonce: u64,
}

enum Payload {
    Transaction(Box<Transaction>),
    Milestone(Box<Milestone>),
    Indexation(Box<Indexation>),
}

struct Transaction {
    pub essence: TransactionPayloadEssence,
    pub unlock_blocks: Vec<UnlockBlock>,
}

struct Milestone {
    essence: MilestoneEssence,
    signatures: Vec<Box<[u8]>>,
}

struct Indexation {
    index: String,
    data: Box<[u8]>,
}

struct TransactionPayloadEssence {
    pub(crate) inputs: Box<[Input]>,
    pub(crate) outputs: Box<[Output]>,
    pub(crate) payload: Option<Payload>,
}

enum Input {
    UTXO(UTXOInput(OutputId)),
}

struct OutputId {
    transaction_id: TransactionId,
    index: u16,
}

enum Output {
    SignatureLockedSingle(SignatureLockedSingleOutput),
}

struct SignatureLockedSingleOutput {
    address: Address,
    amount: u64,
}

enum UnlockBlock {
    Signature(SignatureUnlock),
    Reference(ReferenceUnlock),
}

enum SignatureUnlock {
    Ed25519(Ed25519Signature),
}

struct Ed25519Signature {
    public_key: [u8; 32],
    signature: Box<[u8]>,
}

struct ReferenceUnlock(u16);
```

## `MessageMetadata`

[`MessageMetadata`]: #MessageMetadata

```rust
pub struct MessageMetadata {
    /// Message ID
    pub message_id: String,
    /// Message IDs of parents
    pub parents: Vec<String>,
    /// Solid status
    pub is_solid: bool,
    /// Should promote
    pub should_promote: Option<bool>,
    /// Should reattach
    pub should_reattach: Option<bool>,
    /// Referenced by milestone index
    pub referenced_by_milestone_index: Option<u32>,
    /// Ledger inclusion state
    pub ledger_inclusion_state: Option<String>,
}
```

## `OutputMetadata`

[`OutputMetadata`]: #OutputMetadata

The metadata of an output:

```rust
pub struct OutputMetadata {
    /// Message ID of the output
    pub message_id: Vec<u8>,
    /// Transaction ID of the output
    pub transaction_id: Vec<u8>,
    /// Output index.
    pub output_index: u16,
    /// Spend status of the output
    pub is_spent: bool,
    /// Corresponding address
    pub address: Address,
    /// Balance amount
    pub amount: u64,
}
```

## `Bech32Address`

[Bech32Address]: #Bech32Address

Wrapper type to be used in most cases where an address is involved.

```Rust
pub struct Bech32Address(pub String);
```

## `Address`

[Address]: #Address

An Ed25519 address can be encoded in Bech32 or Hex, with Bech32 being preferred and also used in most functions.

```Rust
pub enum Address {
    Ed25519(Ed25519Address),
}
```

## `AddressBalancePair`

[AddressBalancePair]: #AddressBalancePair

```Rust
pub struct AddressBalancePair {
    /// Address
    pub address: Bech32Address,
    /// Balance in the address
    pub balance: u64,
}
```

## `Milestone`

[Milestone]: #Milestone

A milestone metadata.

```rust
pub struct MilestoneMetadata {
    /// Milestone index
    pub milestone_index: u32,
    /// Milestone ID
    pub message_id: String,
    /// Timestamp
    pub timestamp: u64,
}
```

## `Api`

[Api]: #Api

```Rust
pub enum Api {
    /// `get_health` API
    GetHealth,
    /// `get_info`API
    GetInfo,
    /// `get_tips` API
    GetTips,
    /// `post_message` API
    PostMessage,
    /// `post_message` API with remote pow
    PostMessageWithRemotePow,
    /// `get_output` API
    GetOutput,
    /// `get_milestone` API
    GetMilestone,
}
```

## `BrokerOptions`

[BrokerOptions]: #BrokerOptions

```Rust
pub struct BrokerOptions {
    #[serde(default = "default_broker_automatic_disconnect", rename = "automaticDisconnect")]
    pub(crate) automatic_disconnect: bool,
    #[serde(default = "default_broker_timeout")]
    pub(crate) timeout: std::time::Duration,
    #[serde(default = "default_use_ws")]
    pub(crate) use_ws: bool,
}
```

## `Topic`

[Topic]: #Topic

A string with the exact MQTT topic to monitor, can have one of the following variations:

```
milestones/latest
milestones/solid

messages
messages/referenced
messages/indexation/{index}
messages/{messageId}/metadata

outputs/{outputId}

addresses/{address}/outputs
addresses/ed25519/{address}/outputs
```
