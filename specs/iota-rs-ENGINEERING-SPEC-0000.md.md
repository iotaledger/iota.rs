

# High Level Abstraction API Spec

Specification of High Level Abstraction API

## Table of Content


* [High Level Abstraction API Spec](#High-Level-Abstraction-API-Spec)
  * [Table of Content](#Table-of-Content)
* [Introduction](#Introduction)
* [Builder](#Builder)
* [General API](#General-API)
  * [`send`](#send)
  * [`get_unspent_address`](#get_unspent_address)
  * [`get_addresses`](#get_addresses)
  * [`get_balance`](#get_balance)
  * [`get_address_balances`](#get_address_balances)
  * [`reattach`](#reattach)
* [Full Node API](#Full-Node-API)
  * [`get_info`](#get_info-get-info)
  * [`get_tips`](#get_tips-get-tips)
  * [`get_messages`](#get_messages-get-messages)
  * [`post_messages`](#post_messages-post-messages)
  * [`get_transactions`](#get_transactions-get-transactions)
  * [`get_outputs`](#get_outputs-get-outputs)
* [Objects](#Objects)
  * [`Network`]
  * [`Hash`]
  * [`Seed`]
  * [`Message`]
  * [`Payload`]
  * [`Output`]
  * [`BIP32Path`]
  * [`Address`]


# Introduction

This document specifies a user friendly API to be used in the client libraries. The main implementation will be in Rust which will receive automatically compiled client libraries in other languages via C or Webassembly bindings. There are also many crates to support developers creating foreign function interfaces with native bindings. 

# Builder

The data structure to initialize the instance of the Higher level client library. This is always called first when starting a new interaction with the library. Note: This is the common approach to do initialization in Rust. Different languages might use different methods such as just calling an initialization function directly.


### Parameters


<table>
  <tr>
   <td>Field
   </td>
   <td>Required
   </td>
   <td>Type
   </td>
   <td>Description
   </td>
  </tr>
  <tr>
   <td><strong>network</strong>
   </td>
   <td>&#10004;
   </td>
   <td>
<a href="#Network">Network</a>
   </td>
   <td>Pass an enumeration with elements of <strong> [ mainnet | comnet | devnet ] </strong>to determine the network. If none of the below are given node_pool_urls will default to nood pool lists for mainnet, devnet or comnet based on the network parameter (defaulting to ‘mainnet’, so with no parameters at all it will randomly pick some nodes for mainnet) provided by the IOTA Foundation. Similar to Trinity: \

<p>
```
<p>
export const NODELIST_ENDPOINTS = [
<p>
	'https://nodes.iota.works/api/ssl/live',
<p>
	'https://iota-node-api.now.sh/api/ssl/live',
<p>
	'https://iota.dance/api/ssl/live',
<p>
];
<p>
```
   </td>
  </tr>
  <tr>
   <td><strong>node </strong>
   </td>
   <td>&#10008;
   </td>
   <td>String
   </td>
   <td>The URL of a node to connect to; format: `<a href="https://node:port">https://node:port</a>`
   </td>
  </tr>
  <tr>
   <td><strong>nodes </strong>
   </td>
   <td>&#10008;
   </td>
   <td>[String]
   </td>
   <td>A list of nodes to connect to; nodes are added with the `<a href="https://node:port">https://node:port</a>` format. The amount of nodes specified in quorum_size are randomly selected from this node list to check for quorum based on the quorum threshold. If quorum_size is not given the full list of nodes is checked.
   </td>
  </tr>
  <tr>
   <td><strong>node_pool_urls </strong>
   </td>
   <td>&#10008;
   </td>
   <td>String
   </td>
   <td>A list of URLs containing address pools of multiple nodes in JSON format; Example of such a endpoint: <a href="https://nodes.iota.works/api/ssl/live">https://nodes.iota.works/api/ssl/live</a> - will pick a random pool from the list and will automatically retry if the URL is not available with another one. Consider the nodes found in this list as being entered in the nodes parameter.
   </td>
  </tr>
  <tr>
   <td><strong>quorum_size </strong>
   </td>
   <td>&#10008;
   </td>
   <td>usize
   </td>
   <td>If multiple nodes are given the quorum size defines how many of these nodes will be queried at the same time to check for quorum. If this parameter is not given it defaults to either the length of the `nodes` parameter list, or if node_pool_urls is given a sensible default like 3.
   </td>
  </tr>
  <tr>
   <td><strong>quorum_threshold </strong>
   </td>
   <td>&#10008;
   </td>
   <td>usize
   </td>
   <td>The quorum threshold defines the minimum amount of nodes from the quorum pool that need to agree if we want to consider the result true. The default is 50 meaning at least 50% of the nodes need to agree. (so at least 2 out of 3 nodes when the quorum size is 3).
   </td>
  </tr>
</table>

### Return

Finalize the builder will run the instance in the background. Users don’t need to worry about the return object handling.

# General API

Here is the high level abstraction API collection with sensible default values for users easy to use.


## `send()`

A generic send function for easily sending value transaction messages. 

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **seed** | ✔ | [Seed] | The seed of the account we are going to spend. |
| **path** | ✔ | [BIP32Path] | The wallet chain BIP32 path we want to search for. |
| **address** | ✔ | [Address] | The address to send to. |
| **value** | ✔ | std::num::NonZeroU64 | The amount of IOTA to send. It is type of NoneZero types, so it connot be zero. |
| **index** | ✘ | u32 | Start index of the wallet account address. Default is 0, but note taht **it's recommended to provide index** since this method consider spent address as error for security. And because this is a stateless crate, account user should keep track of what's the unuspent address index of corresponding wallet chain themselves. |

### Return

The [Message] object we build.

### Implementation Details

There could be two different scenarios if which this method is used:

* Validate inputs, such as address, seed, and path to check if they are correct. For example, the provided path must be
  wallet chain which should have depth of 2;
* Check if account balance is bigger or equal to the value using method similar to [`get_balance()`](#get_balance);
* Build and Validate the Message with signed transaction payloads accordingly;
* Get tips using [`get_tips()`](#get_tips);
* Perform proof-of-work locally; 
* Send the message using [`post_messages()`](#post_messages);

## `get_unspent_address()`

Return a valid unuspent address.

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **seed** | ✔ | [Seed] | The seed we want to search for. |
| **path** | ✔ | [BIP32Path] | The wallet chain BIP32 path we want to search for. |
| **index** | ✘ | u32 | Start index of the address. **Default is 0.** |

### Return

Return a tuple with type of `([Address], usize)` as the address and corresponding index.

### Implementation Details

Following are the steps for implementing this method:

* Start generating addresses with given wallet chain path and starting index. We will have a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20 at a time;
* Check for balances on the generated addresses using [`get_outputs()`](#get_outputs-get-outputs) and keep track of the positive balances;
* Repeat the above step till there's an unuspent addresses found;
* Return the address with corresponding index on the wallet chain;


## `get_addresses()`

Return a list of addresses from the seed regardless of their validity.

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **seed** | ✔ | [Seed] | The seed we want to search for. |
| **path** | ✔ | [BIP32Path] | The wallet chain BIP32 path we want to search for. |
| **range** | ✘ | std::ops::Range | Range indice of the addresses we want to search for **Default is (0..20)** |

### Return

A list of Address [Hash](#Hash)es

### Implementation Details

Following are the steps for implementing this method:

*   Start generating address at index 0 with a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20;
*   Return the addresses.

## `get_balance()`

Return the balance for a provided seed and its wallet chain BIP32 path. BIP32 derivation path of the address should be in form of `m/0'/0'/k'`. So the wallet chain is expected to be `m/0'/0'`. Addresses with balance must be consecutive, so this method will return once it encounters a zero balance address.

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **seed** | ✔ | [Seed] | The seed we want to search for. |
| **path** | ✔ | [BIP32Path] | The wallet chain BIP32 path we want to search for. |
| **index** | ✘ | u32 | Start index of the address. **Default is 0.** |

### Return

Total Account balance.

### Implementation Details

Following are the steps for implementing this method:

* Start generating addresses with given wallet chain path and starting index. We will have a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20 at a time;
* Check for balances on the generated addresses using [`get_outputs()`](#get_outputs-get-outputs) and keep track of the positive balances;
* Repeat the above step till an addresses of zero balance is found;
* Accumulate the positive balances and return the result.


## `get_address_balances()`

Return the balance in iota for the given addresses; No seed or security level needed to do this since we are only checking and already know the addresses.

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **addresses** | ✔ | [[Hash]] | List of addresses with checksum. |

### Return

A list of tuples with value of  (Address, usize). The usize is the balance of the address accordingly. 

### Implementation details:

Following are the steps for implementing this method:

*   Validate _address_ semantics;
*   Get latest balance for the provided address using [`get_outputs()`](#get_outputs-get-outputs) with addresses as
    parameter;
*   Return the list of Output which contains corresponding pairs of address and balance.

## `reattach()`

Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
confirmed for a while. 

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **hashes** | ✔ | [[Hash]] | The identifier of message. |

### Returns:

Newly reattached [Message](#Message).

### Implementation Details

Following are the steps for implementing this method: 

* Only unconfirmed messages should be allowed to reattach. The method should validate the confirmation state of the provided messages. If a message hash of a confirmed message is provided, the method should error out;
* The method should also validate if the message reattachment is necessary. This can be done by checking if the message falls below max depth. The criteria of checking whether the message has fallen below max depth is through time. If 11 minutes have passed since the timestamp of the most recent (reattachment), the message can be allowed to be reattached. See [this](https://github.com/iotaledger/trinity-wallet/blob/3fab4f671c97e805a2b0ade99b4abb8b508c2842/src/shared/libs/iota/transfers.js#L141) implementation for reference;
* Get tips pair using [`get_tips()`](#get_tips-get-tips);
* Perform proof-of-work;
* Store messages on the tangle using [`post_messages()`](#post_messages-post-messages);

# Full Node API

API of Bee and Hornet will still be public. Users who know these relative low level Restful API can still call them directly if they are confident and think it’s good for them. Note that both Bee and hornet haven't finalized their APIs either. Following items and signatures might change later.

## `get_health()` (`GET /health`)

Returns the health of the node, which can be used for load-balancing or uptime monitoring.

### Parameters

None

### Returns

Boolean to indicate if node is healthy.

## `get_info()` (`GET /api/v1/info`)

Returns information about the node.

### Parameters

None

### Returns

A Response Object similar to this:

```rust
struct getInfoResponse {
       name: String,
       version: String,
       isHealthy: bool,
       operatingNetwork: String,
       peers: usize,
       coordinatorAddress: String,
       isSynced: bool,
       latestMilestone: Hash,
       latestMilestoneIndex: usize,
       latestSolidMilestone: Hash,
       latestSolidMilestoneIndex: usize,
       pruningIndex: usize,
       time: usize,
       features": Vec<String>,
}
```

## `get_tips()` (`GET /tips`)

Returns two non-lazy tips. There could be however the case that the node can provide only one tip, or in the worst-case no tip. The array therefore needs to be validated.

### Parameters

None

### Returns

A tuple with two hashes:

```rust
(Hash, Hash)
```

## `get_message()` (`GET /api/v1/message/{messageId}}`)

Find all messages filtered by provided parameters.

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **message_id** | ✘ | [Hash] | The identifier of message. |

### Returns

Depend on the final calling method, users could get different outputs they need:

- `metadata()`: Return metadata of the message.
- `data()`: Return a [Message] object.
- `raw()`: Return the given message raw data.
- `children()`: Returns the list of message IDs that reference a message by its identifier.

## `post_messages()` (`POST /messages`)

Submit a message as a JSON object to the node. If certain fields are missing the node tries to take care of it (e.g. missing nonce, missing branch/trunk, …) and builds the message. On success, the node stores the message and broadcasts it to its peers. Furthermore it returns the hash of the message.

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **messages** | ✘ | [[Message]] | The list of messages. |

### Returns

A vector of Message [Hash] object.

## `get_transactions()` (`GET /transactions`)

Find all transactions filtered by provided parameters.

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **hashes** | ✘ | [[Hash]] | The identifier of message. |
| **addresses** | ✘ | [[Hash]] | The hashes of addresses. |
| **confirmed** | ✘ | bool | Search transaction that are confirmed if this sets to ture. |

*At least one parameter has to be provided.

### Returns

A vector of [Message] object.

## `get_outputs()` (`GET /outputs`)

Get the producer of the output, the corresponding address, amount and spend status of an output. This information can only be retrieved for outputs which are part of a confirmed transaction.

### Parameters

| Field | Requried | Type | Definition |
| - | - | - | - |
| **hashes** | ✘ | [[Hash]] | The identifier of message. |
| **addresses** | ✘ | [[Hash]] | The hashes of addresses. |

*At least one parameter has to be provided.

### Returns

A vector of [Output] object.


# Objects

Here are the objects used in the API above. They aim to provide a secure way to handle certain data structures specified in the Iota stack.


## `Network`
[`Network`]: #Network

Network is an enumeration with elements of **[mainnet|comnet|devnet]**. Some languages might lack of type like an enum. In this case, Network can be a set of constant variables.

```rust
enum Network {
  Mainnet,
  Comnet,
  Devnet,
}
```

## `Hash`
[`Hash`]: #Hash

| Field | Requried | Type | Definition |
| - | - | - | - |
| **hash** | ✔ | `[u8; 32]` | A valid IOTA hash which can be treated as many objects like Address, Message hash, and more. The inner structure of course will instantiate the actual objects. This serves as a convenient but secure way for users passing parameters. |

## `Seed`
[`Seed`]: #Seed

| Field | Requried | Type | Definition |
| - | - | - | - |
| **seed** | ✔ | `[u8; 32]` | An IOTA seed that inner structure is omitted. Users can create this type by passing a String. It will verify and return an error if it’s not valid. |

## `Message`
[`Message`]: #Message

The message object returned by various functions; based on the RFC for the Message object.

| Field | Requried | Type | Definition |
| - | - | - | - |
| **version** | ✔ | usize | Message version. Defaults to `1`. |
| **trunk** | ✔ | [Hash] | Message hash of the first message this message refers to. |
| **branch** | ✔ | [Hash] | Message hash of the second message this message refers to. |
| **payload_length** | ✔ | usize | Length of the payload. |
| **payload** | ✔ | [[Payload]] | List of the payload. |
| **timestamp** | ✔ | usize | Transaction timestamp (exposed as a custom type with additional methods). |
| **nonce** | ✔ | [Hash] | Transaction nonce. |
| **confirmed** | ✔ | bool | Determines if the transaction is confirmed. |

## `Payload`
[`Payload`]: #Payload

The payload object returned by various functions; based on the RFC for the payload object.

## `Output`
[`Output`]: #Output

The contexts of an output address

| Field | Requried | Type | Definition |
| - | - | - | - |
| **producer** | ✔ | [Hash] | The hash of the message which contains this output. |
| **address** | ✔ | [Address] | Corresponding address |
| **balance** | ✔ | usize | The balance in this output. |
| **spent** | ✔ | bool | The output has been spent if true. |

## `BIP32Path`
[`BIP32Path`]: #BIP32Path

A valid BIP32 path. The field is ommited.

## `Address`
[`Address`]: #Address

An address is a enum which could be either Ed25519 format or the legay WOTS.

