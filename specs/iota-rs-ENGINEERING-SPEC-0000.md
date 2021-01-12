

# High Level API Specification

## Table of Contents

* [Introduction](#Introduction)
* [Builder](#Builder)
* [General high level API](#General-high-level-API)
  * [`send`](#send)
  * [`get_message`](#get_message)
  * [`find_messages`](#find_messages)
  * [`get_unspent_address`](#get_unspent_address)
  * [`find_addresses`](#find_addresses)
  * [`get_balance`](#get_balance)
  * [`get_address_balances`](#get_address_balances)
  * [`subscriber`](#subscriber)
  * [`retry`](#retry)
  * [`reattach`](#reattach)
  * [`promote`](#promote)
* [Full node API](#Full-node-API)
  * [`get_health`](#get_health)
  * [`get_info`](#get_info)
  * [`get_tips`](#get_tips)
  * [`post_message`](#post_message)
  * [`get_output`](#get_output)
  * [`get_address`](#get_address)
  * [`find_outputs`](#find_outputs)
  * [`get_milestone`](#get_milestone)
* [Objects](#Objects)
  * [Network]
  * [Seed]
  * [Message]
  * [Payload]
  * [Output]
  * [Address]


# Introduction

This document specifies a user friendly API to be used in the client libraries. The main implementation will be in Rust which will receive automatically compiled client libraries in other languages via C or Webassembly bindings. There are also many crates to support developers creating foreign function interfaces with native bindings.

# Builder

The data structure to initialize the instance of the Higher level client library. This is always called first when starting a new interaction with the library. Note: This is the common approach to do initialization in Rust. Different languages might use different methods such as just calling an initialization function directly.

### Parameters

| Field | Required | Default Value | Type | Definition |
| - | - | - | - | - |
| **with_network** | ✘ | 'mainnet' | [Network] | Pass an enumeration with elements of **mainnet/comnet/devnet** to determine the network. If none of the below are given node_pool_urls will default to node pool lists for mainnet, devnet or comnet based on the network parameter (defaulting to ‘mainnet’, so with no parameters at all it will randomly pick some nodes for mainnet) provided by the IOTA Foundation. Similar to Trinity: `export const NODELIST_ENDPOINTS = [	'https://nodes.iota.works/api/ssl/live', 'https://iota-node-api.now.sh/api/ssl/live', 'https://iota.dance/api/ssl/live',];`|
| **with_node** | ✘ | None | &str | The URL of a node to connect to; format: `https://node:port` |
| **with_nodes** | ✘ | None | &[&str] | A list of nodes to connect to; nodes are added with the `https://node:port` format. The amount of nodes specified in quorum_size are randomly selected from this node list to check for quorum based on the quorum threshold. If quorum_size is not given the full list of nodes is checked. |
| **with_node_sync_interval** | ✘ | Duration::from_secs(60) | std::time::Duration | The interval in milliseconds to check for node health and sync |
| **with_request_timeout** | ✘ | Duration::from_secs(30) | std::time::Duration | The amount of seconds a request can be outstanding to a node before it's considered timed out |
| **with_api_timeout** | ✘ | self.request_timeout | Api, std::time::Duration | The amount of milliseconds a request to a specific Api endpoint can be outstanding to a node before it's considered timed out |
| **with_local_pow** | ✘ | True | bool | If not defined it defaults to local PoW to offload node load times |
| **with_mqtt_broker_options** | ✘ | automatic_disconnect: True, timeout: Duration::from_secs(30), use_ws: True | BrokerOptions | If not defined it defaults to local PoW to offload node load times |


* Note that there must be at least one node to build the instance successfully.

### Return

Finalize the builder will run the instance in the background. Users don’t need to worry about the return object handling.


## On initialization
On initialisation, call getNodeInfo API. Check the health of each node in the node list, place any nodes that are unresponsive or with isHealthy = false on a temporary blacklist. Store important metadata including MQTT port, network, remote proof of work for each node.

| Node metadata | Description |
| - | - |
| network | If this parameter does not match the global builder parameter, add node to blacklist and return error. |
| mqtt_port | Used in establishing MQTT subscriptions. If failure to connect to MQTT, place node in blacklist. |
| pow | If the global local_pow parameter is set to false, then put any nodes without pow support in the blacklist. |


## Sync Process

When a `Client` instance (The instance which is used for calling the client APIs) is built, the status of each node listed in the `node_pool_urls` should be checked first. If the returned status of the node information is healthy, which means the node is synced, then this node will be pushed back into a `synced_node_list`. The rust-like pseudo code of `synced_node_list` construction process follows. The process of syncing a node is repeated every 60 seconds or at the interval specified in the `node_sync_interval` argument of the initializer if set.

```rust
synced_node_list = Vec::new()
for node in node_pool_urls{
   status = Client.get_info(node).await?;
   if status == healthy{
      synced_node_list.push(node)
   }
}
```

# `General high level API`

Here is the high level abstraction API collection with sensible default values for users easy to use.


## `send()`

A generic send function for easily sending a message.

### Methods

| Method | Required | Default | Type | Definition |
| - | - | - | - | - |
| **with_seed** | ✘ | None | [Seed] | The seed of the account we are going to spend, only needed for SignedTransactions (value) |
| **with_account_index** | ✘ | 0 | usize | The account index |
| **with_initial_address_index** | ✘ | 0 | usize | The index from where to start looking for balance |
| **with_input** | ✘ | None | UTXOInput | Users can manually pick their own UTXOInput instead of having node decide on which output should be used. |
| **with_output** | ✘ | None | address: &[Bech32Address], amount: u64 | Address to send to and amount to send. Address needs to be Bech32 encoded. |
| **with_output_hex** | ✘ | None | address: &str, amount: u64 | Address to send to and amount to send. Address needs to be hex encoded. |
| **with_index** | ✘ | None | &str | An optional indexation key for an indexation payload. |
| **with_data** | ✘ | None | Vec<u8> | An optional indexation data of the indexation payload. |
| **with_parent** | ✘ | None | MessageId | An optional parent message to be used as one parent. |
| **with_network_id** | ✘ | None | u64 | Optional network id, if not set it will be used from the nodeinfo. |

* Depending on the provided values this function will create a message without a payload, with an indexation payload or with a transaction payload, conatining an indexation payload.

### Return

The [Message] object we build.

### Implementation Details

There could be two different scenarios in which this method can be used:

* Validate inputs, such as address and seed to check if they are correct.
* Check if account balance is bigger or equal to the value using method similar to [`get_balance()`](#get_balance);
* Build and validate the Message with signed transaction payload accordingly;
* Get tips using [`get_tips()`](#get_tips);
* Perform proof-of-work locally (if not set to remote);
* Send the message using [`post_messages()`](#post_messages);

## `get_message()`

(`GET /api/v1/messages`)

Endpoint collection all about GET messages.

### Parameters

| Field | Required | Type | Definition |
| - | - | - | - |
| **message_id** | ✔ | [MessageId] | The identifier of message. |
| **&str** | ✔ | [MessageId] | An indexation key. |

### Returns

Depend on the final calling method, users could get different results they need:

- `metadata(&MessageId)`: Return MessageMetadata of the message.
- `data(&MessageId)`: Return a [Message] object.
- `raw(&MessageId)`: Return the raw data of given message.
- `children(&MessageId)`: Return the list of [messageId]s that reference a message by its identifier.
- `index(&str)` : Return the list of [messageId]s that have this string as indexation key

## `find_messages()`

Find all messages by provided message IDs. This method will try to query multiple nodes if the request amount exceed individual node limit.

### Parameters

| Field | Required | Type | Definition |
| - | - | - | - |
| **message_ids** | ✘ | [[MessageId]] | The identifier of message. |

### Returns

A vector of [Message] Object.

## `get_unspent_address()`

Return a valid unspent public Bech32 encoded address.

### Parameters

| Field | Required | Default | Type | Definition |
| - | - | - | - | - |
| **with_seed** | ✔ | - | [Seed] | The seed we want to use. |
| **with_account_index** | ✘ | 0 | usize | The account index. |
| **with_initial_address_index** | ✘ | 0 | usize | Start index of the addresses to search. |

### Return

Return a tuple with type of `(Bech32Address, usize)` as the address and corresponding index in the account.

### Implementation Details

Following are the steps for implementing this method:

* Start generating addresses with given account index and starting index. We will have a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20 at a time;
* Check for balances on the generated addresses using [`get_outputs()`](#get_outputs-get-outputs) and keep track of the positive balances;
* Repeat the above step till there's an unspent address found;
* Return the address with corresponding index on the wallet chain;

## `find_addresses()`

Return a list of addresses from the seed regardless of their validity.

### Parameters/Methods

| Field | Required | Default | Type | Definition |
| - | - | - | - | - |
| **seed** | ✔ | None | [Seed] | The seed we want to search for. |
| **with_account_index()** | ✘ | 0 | usize | The account index. |
| **with_range()** | ✘ | None | std::ops::Range | Range indices of the addresses we want to search for. Default is (0..20) |
| **get_all()** | ✘ | ✘ | ✘ | Get public and [change addresses](https://bitcoin.stackexchange.com/questions/75033/bip44-and-change-addresses). Will return Vec<(Bech32Address, bool)> |


### Return

Vec<[Bech32Address]>, with the public addresses


## `get_balance()`

Return the balance for a provided seed and its wallet account index. 

### Parameters

| Field | Required | Default | Type | Definition |
| - | - | - | - | - |
| **seed** | ✔ | - | [Seed] | The seed we want to search for. |
| **with_account_index** | ✘ | 0 | usize | The account index. |
| **with_initial_address_index** | ✘ | 0 | usize | Start index of the address. **Default is 0.** |

### Return

Total account balance.

### Implementation Details

Following are the steps for implementing this method:

* Start generating addresses with given wallet account index and starting index. We will have a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20 at a time;
* Check for balances on the generated addresses using [`get_outputs()`](#get_outputs-get-outputs) and keep track of the positive balances;
* Repeat the above step till an address of zero balance is found;
* Accumulate the positive balances and return the result.


## `get_address_balances()`

Return the balance in iota for the given addresses; No seed or security level needed to do this since we are only checking and already know the addresses.

### Parameters

| Field | Required | Type | Definition |
| - | - | - | - |
| **addresses** | ✔ | [Bech32Address] | List of Bech32 encoded addresses. |

### Return

A list of tuples with value of [AddressBalancePair]. The usize is the balance of the address accordingly.

### Implementation details:

Following are the steps for implementing this method:

*   Validate _address_ semantics;
*   Get latest balance for the provided address using [`get_outputs()`](#get_outputs-get-outputs) with addresses as
    parameter;
*   Return the list of Output which contains corresponding pairs of address and balance.

## `subscriber()`

Subscribe to a node event topic (MQTT)

Required: one of

* `with_topic()`: Add a new [Topic] to the list.
* `with_topics()`: Add a vector of [Topic] to the list.

* `subscribe()`: Subscribe to the given topics with the callback, which will be called every time when the topic is detected.
* `unsubscribe()`: Unsubscribes from all subscriptions.
* `disconnect()`: Disconnects the broker. This will clear the stored topic handlers and close the MQTT connection.

### Returns

Nothing apart from a Ok(()) result if successful

## `retry()`

Retries (promotes or reattaches) a message for provided [MessageId] if the node suggests it. The need to use this function should be low, because the confirmation throughput of the node is expected to be quite high when there is no attack.

### Parameters

| Field | Required | Type | Definition |
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

| Field | Required | Type | Definition |
| - | - | - | - |
| **message_id** | ✔ | [MessageId] | The identifier of message. |

### Returns

A tuple with the newly reattached `(MessageId,  Message)`.

## `promote()`

Depends on [find_messages](#find_messages), [get_message](#get_message) and [post_message](#post_message).

Promotes a message. The method should validate if a promotion is necessary through [get_message](#get_message). If not, the method should error out and should not allow unnecessary promotions.

### Parameters

| Field | Required | Type | Definition |
| - | - | - | - |
| **message_id** | ✔ | [MessageId] | The identifier of message. |

### Returns

A tuple with the newly promoted `(MessageId,  Message)`.

# Full node API

Full node API of Bee and Hornet will still be public. Users who know these relative low level Restful API can still call them directly if they are confident and think it’s good for them. Note that both Bee and hornet haven't finalized their APIs either. Following items and signatures might change later.

## `get_health()`

(`GET /health`)

Returns the health of the node, which can be used for load-balancing or uptime monitoring.

### Parameters

None

### Returns

Boolean to indicate if node is healthy.

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
    pub min_pow_score: usize,
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

| Field | Required | Type | Definition |
| - | - | - | - |
| **message** | ✔ | [Message] | The message object. |

### Returns

The [MessageId] of the message object.

## `get_output()`

(`GET /outputs`)

Get the producer of the output, the corresponding address, amount and spend status of an output. This information can only be retrieved for outputs which are part of a confirmed transaction. It will have additional methods such as reattach to perform extra functionality.

### Parameters

| Field | Required | Type | Definition |
| - | - | - | - |
| **output_id** | ✔ | UTXOInput | Identifier of the output. |

### Returns

An [OutputMetadata](#outputmetadata) that contains various information about the output.

## `get_address()`

(`GET /addresses`)

### Parameters

| Field | Required | Type | Definition |
| - | - | - | - |
| **address** | ✔ | [Bech32Address] | The address to search for. |

### Returns

Depend on the final calling method, users could get different outputs they need:

- `balance()`: Return confirmed balance of the address.
- `outputs()`: Return UTXOInput array (transaction IDs with corresponding output index).

## `find_outputs()`

Find all outputs based on the requests criteria. This method will try to query multiple nodes if the request amount exceed individual node limit.

### Parameters

| Field | Required | Type | Definition |
| - | - | - | - |
| **output_id** | ✘ | [UTXOInput] | The identifier of output. |
| **addresses** | ✘ | [[Bech32Address]] | The identifier of address. |

### Returns

A vector of [OutputMetadata](#outputmetadata).

## `get_milestone()`

(`GET /milestones`)

Get the milestone by the given index.

### Parameters

| Field | Required | Type | Definition |
| - | - | - | - |
| **index** | ✔ | u64 | Index of the milestone. |

### Returns

An [Milestone] object.

# Objects

Here are the objects used in the API above. They aim to provide a secure way to handle certain data structures specified in the Iota stack.


## `Network`

[Network]: #Network

Network is an enumeration with elements of **[mainnet|comnet|devnet]**. Some languages might lack of type like an enum. In this case, Network can be a set of constant variables.

```rust
enum Network {
  Mainnet,
  Comnet,
  Devnet,
}
```

## `MessageId`

[MessageId]: #MessageId

MessageId is a 32 bytes array which can represent as hex string.

```rust
struct MessageId([u8; MESSAGE_ID_LENGTH]);
```

## `Seed`

[Seed]: #Seed

| Field | Required | Type | Definition |
| - | - | - | - |
| **seed** | ✔ | `[u8; 32]` | An IOTA seed that inner structure is omitted. Users can create this type by passing a String. It will verify and return an error if it’s not valid. |

## `Message`

[Message]: #Message

The message object returned by various functions; based on the RFC for the Message object. Here's the brief overview of each components in Message type would look like:

```rust
struct Message {
    parent1: MessageId,
    parent2: MessageId,
    payload: Option<Payload>,
    nonce: u64,
}

enum Payload {
    Transaction(Box<Transaction>),
    Milestone(Box<Milestone>),
    Indexation(Box<Indexation>),
}

struct Transaction {
    pub essence: TransactionEssence,
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

struct TransactionEssence {
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

[`MessageMetadata`]: #MessageMetadata

```rust
pub struct MessageMetadata {
    /// Message ID
    pub message_id: String,
    /// Message ID of parent1
    pub parent1: String,
    /// Message ID of parent2
    pub parent2: String,
    /// Solid status
    pub is_solid: bool,
    /// Should promote
    pub should_promote: Option<bool>,
    /// Should reattach
    pub should_reattach: Option<bool>,
    /// Referenced by milestone index
    pub referenced_by_milestone_index: Option<u64>,
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
    Wots(WotsAddress),
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
struct MilestoneMetadata {
    /// Milestone index
    pub index: u64,
    /// Milestone ID
    pub message_ids: String,
    /// Timestamp
    pub timestamp: u64,
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

## `Api`

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
    /// `get_output` API
    GetOutput,
    /// `get_milestone` API
    GetMilestone,
}
```

## `BrokerOptions`

```Rust
pub struct BrokerOptions {
    #[serde(default = "default_broker_automatic_disconnect", rename = "automaticDisconnect")]
    pub(crate) automatic_disconnect: bool,
    #[serde(default = "default_broker_timeout")]
    pub(crate) timeout: Duration,
    #[serde(default = "default_use_ws")]
    pub(crate) use_ws: bool,
}
```
