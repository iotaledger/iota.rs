

# High Level Abstraction API Spec

Specification of High Level Abstraction API

## Table of Content


* [High Level Abstraction API Spec](#High-Level-Abstraction-API-Spec)
    * [Table of Content](#Table-of-Content)
* [Introduction](#Introduction)
* [Builder](#Builder)
* [General API](#General-API)
    * [Send](#Send)
    * [FindMessages](#FindMessages)
    * [GenerateNewAddress](#GenerateNewAddress)
    * [GetAddresses](#GetAddresses)
    * [GetBalance](#GetBalance)
    * [GetBalanceOfAddresses](#GetBalanceOfAddresses)
    * [Reattach](#Reattach)
    * [IsConfirmed](#IsConfirmed)
* [Bee / IRI API](#Bee-/-IRI-API)
    * [AttachToTangle](#AttachToTangle)
    * [GetInclusionState](#GetInclusionState)
    * [GetMessagesToApprove](#GetMessagesToApprove)
    * [GetBytes](#GetBytes)
    * [BroadcastMessages](#BroadcastMessages)
    * [StoreMessages](#StoreMessages)
    * [WereAddressesSpentFrom](#WereAddressesSpentFrom)
* [Objects](#Objects)
    * [Network](#Network)
    * [Hash](#Hash)
    * [Seed](#Seed)
    * [Encoding](#Encoding)
    * [Message](#Message)


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


## Send

A generic send function for easily sending data or value transactions. 


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
   <td><strong>address</strong>
   </td>
   <td>&#10004;
   </td>
   <td>

<a href="#Hash">Hash</a>
   </td>
   <td>The address to send to. This should be a valid Address with correct checksum. Otherwise, it will return an error.
   </td>
  </tr>
  <tr>
   <td><strong>value</strong>
   </td>
   <td>&#10004;
   </td>
   <td>u64
   </td>
   <td>The amount of IOTA to send, in iota. If this is a data only transaction we can ignore this field or provide 0. If the amount of this field is higher than 0 we need to provide a seed as well *
   </td>
  </tr>
  <tr>
   <td><strong>seed</strong>
   </td>
   <td>&#10004;
   </td>
   <td>

<a href="#Seed">Seed</a>
   </td>
   <td>Only required for value transfers; this is a draft, seed storage will probably be handled by a secure vault which should be used directly in the higher level client libs
   </td>
  </tr>
  <tr>
   <td><strong>message </strong>
   </td>
   <td>&#10008;
   </td>
   <td>String
   </td>
   <td>A message to send together with this transaction. Note: String in rust is utf-8 encoded which is compatible to ascii. If users want to use other encodings, they will have to convert themselves. 
   </td>
  </tr>
  <tr>
   <td><strong>local_pow </strong>
   </td>
   <td>&#10008;
   </td>
   <td>bool
   </td>
   <td>Determines if proof-of-work should be offloaded to the connected node. <strong>Default to false.</strong>
   </td>
  </tr>
</table>

### Return

A simple transaction hash. Since bundles have no place anymore and transactions can have a variable size with Atomic transactions this makes most sense. It could be a transaction object as well if that makes more sense.


### Implementation Details

There could be two different scenarios if which this method is used: \




1. Data transaction:  \
Following are the steps for implementing this method if provided value is zero:
*   Validate address and its checksum;
*   Validate message semantics;
*   Get transactions to approve using [getTransactionsToApprove()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#gettransactionstoapprove);
*   Perform proof-of-work (If _local_pow_ is set to false, the proof-of-work should be offloaded to the selected node using [attachToTangle()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#attachtotangle). Otherwise, proof-of-work should be performed locally)
*   Store transactions on the tangle using [storeTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#storetransactions);
*   Broadcast transactions to the tangle using [broadcastTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#storetransactions).
2. Value transaction:

	Following are the steps for implementing this method if provided value is greater than    zero:



*   Validate address and its checksum;
*   Validate message semantics;
*   Prepare inputs (See [Input Selection process](https://docs.google.com/document/d/17JHw7HpNn3_qKKXaxoQJFxQv4em9xomh0EvvWOzIQzI/edit#heading=h.eby2xfmp8y49) for more details. Input selection process should make sure the _value_ doesn’t exceed the total balance);
*   Sign transaction (To be decided how this will be signed using _external_signer_);
*   Get transactions to approve using [getTransactionsToApprove()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#gettransactionstoapprove);
*   Perform proof-of-work (If _local_pow_ is set to false, the proof-of-work should be offloaded to the selected node using [attachToTangle()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#attachtotangle). Otherwise, proof-of-work should be performed locally)
*   Store transactions on the tangle using [storeTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#storetransactions);
*   Broadcast transactions to the tangle using [broadcastTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#storetransactions).


## GetMessage

Retrieve a single message object using the message hash; Given the variable transaction length/atomic transactions in Chrysalis this will be a more commonly used function over retrieving multiple transactions from a bundle which we won’t have any more with Chrysalis.


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
   <td><strong>message_hash</strong>
   </td>
   <td>&#10004;
   </td>
   <td>
<a href="#Hash">Hash</a>
   </td>
   <td>The hash of the transaction we are fetching; since we are just looking to use this function to get 1 transaction in total only this parameter makes sense unlike an address which can contain multiple transactions.
   </td>
  </tr>
  <tr>
   <td><strong>encoding</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>
<a href="#Encoding">Encoding</a>
   </td>
   <td>The converter/encoder that was used to convert the message into bytes/trytes (whatever the transaction would need). The underlying functionality of this function will automatically process the raw transaction data and use this converter (default to utf-8/bytes) to give the end user something usable back. This converter can be any function including some defaults as documented in the `send` function. <strong>Default to Encoding::UTF8</strong>
   </td>
  </tr>
</table>



### Return

[Transaction](#Transaction)


### Implementation Details

Following are the steps for implementing this method: \




*   Validate message hash semantics;
*   Get transaction bytes using [getBytes()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#gettrytes);
*   Parse transaction trytes to transaction object (See [asTransactionObject()](https://github.com/iotaledger/iota.js/blob/next/packages/transaction-converter/src/index.ts#L236) for parsing trytes to transaction object)


## FindMessages

Find multiple messages using one or multiple fields. If multiple search fields are provided consider the search function to work as a AND implementation.


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
   <td><strong>transaction_hashes</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>
<a href="#Hash">Hash</a>
   </td>
   <td>An optional argument where you can provide a list of transaction hashes that will be fetched. 
   </td>
  </tr>
  <tr>
   <td><strong>address</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>
<a href="#Hash">Hash</a>
   </td>
   <td>An address to find the transactions for; One address can contain multiple transactions.
   </td>
  </tr>
  <tr>
   <td><strong>tag</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>
<a href="#Hash">Hash</a>
   </td>
   <td>A tag to search for, returns transactions starting with the provided tag prefix. This can be useful for for example prefix tags like in the Industry Marketplace or Location data (IOTA Area Codes).
   </td>
  </tr>
  <tr>
   <td><strong>offset</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>usize
   </td>
   <td><strong>By default this function will return up to 100 of the latest transactions</strong> matching the search criteria. In order to allow iterating over more transactions we can provide an offset which by default is 0. Page 1 would be offset: 0, limit: 100, page 2 would be offset: 100, limit 100, etc. We might want to be able to provide something else like a transaction hash for offset instead since transactions might move to a second page while iterating because a new transaction came in while iterating.
   </td>
  </tr>
  <tr>
   <td><strong>limit</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>usize
   </td>
   <td>The amount of transactions to retrieve in 1 go. <strong>By default this is 100.</strong>
   </td>
  </tr>
  <tr>
   <td><strong>encoding</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>
<a href="#Encoding">Encoding</a>
   </td>
   <td>The converter/encoder that was used to convert the message into bytes/trytes (whatever the transaction would need). The underlying functionality of this function will automatically process the raw transaction data and use this converter (default to utf-8/bytes) to give the end user something usable back. This converter can be any function including some defaults as documented in the `send` function. <strong>Default to Encoding::UTF8</strong>
   </td>
  </tr>
</table>



### Return

A list of [Message](#Message)s


### Implementation Details

Following are the steps for implementing this method: \




*   Validate _transaction_hashes_;
*   Validate _address_;
*   Validate _tag_;
*   If the _transaction_hashes _parameter is provided, it should ignore all other parameters and fetch transaction trytes for the provided hashes using [getTrytes()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#gettrytes);
*   If the transaction_hashes parameter is not provided, it should fetch transaction hashes using [findTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#findtransactions). Duplicate transaction hashes should be removed. Transaction trytes of deduplicated transaction hashes should be fetched using [getTrytes()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#gettrytes);
*   Details of _limit_ and _offset_ parameters are yet to be decided;
*   All transaction trytes fetched from the network should be parsed to transaction objects (see [asTransactionObject()](https://github.com/iotaledger/iota.js/blob/next/packages/transaction-converter/src/index.ts#L236) for a reference implementation).


## GenerateNewAddress

Return a valid unused address with checksum.


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
   <td><strong>seed</strong>
   </td>
   <td>&#10004;
   </td>
   <td>
<a href="#Seed">Seed</a>
   </td>
   <td>Only required for value transfers; this is a draft, seed storage will probably be handled by a secure vault which should be used directly in the higher level client libs
   </td>
  </tr>
  <tr>
   <td><strong>index</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>usize
   </td>
   <td>Key index to start search at. <strong>Default is 0.</strong>
   </td>
  </tr>
</table>



### Return

[Hash](#Hash) of Address with checksum


### Implementation Details

Following are the steps for implementing this method: \




*   Start generating address at index 0 with a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20;
*   Check for balances and transactions on the generated addresses using [getBalances()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#getbalances) and [findTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#findtransactions);
*   If there are no transactions and zero balances on all addresses, return the (checksummed) address with the least index that has no transactions and zero balance;
*   If there are transactions or any balance on the generated addresses, generate more gap limit number of addresses starting from the index of the last address with transactions or balance. Repeat this process until a set of addresses is found with zero balances and no transactions. Once such a set of addresses is found, return the (checksummed) address with the least index that has no transactions and zero balance.


## GetAddresses

Return a list of addresses with checksum from the seed regardless of their validity.


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
   <td><strong>seed</strong>
   </td>
   <td>&#10004;
   </td>
   <td>
<a href="#Seed">Seed</a>
   </td>
   <td>Only required for value transfers; this is a draft, seed storage will probably be handled by a secure vault which should be used directly in the higher level client libs
   </td>
  </tr>
  <tr>
   <td><strong>start</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>usize
   </td>
   <td>Key index to start search at. <strong>Default is 0.</strong>
   </td>
  </tr>
  <tr>
   <td><strong>end</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>uszie
   </td>
   <td>Key index to end the search. <strong>Default is 20.</strong>
   </td>
  </tr>
</table>



### Return

A list of Address [Hash](#Hash)es with checksum


### Implementation Details

Following are the steps for implementing this method: \




*   Start generating address at index 0 with a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20;
*   Return the (checksummed) addresses.


## GetBalance

Returns the balance for a provided seed by checking the addresses for a seed up until a given point. 


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
   <td><strong>seed</strong>
   </td>
   <td>&#10004;
   </td>
   <td>
<a href="#Seed">Seed</a>
   </td>
   <td>Only required for value transfers; this is a draft, seed storage will probably be handled by a secure vault which should be used directly in the higher level client libs
   </td>
  </tr>
  <tr>
   <td><strong>index</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>usize
   </td>
   <td>Key index to start search at. <strong>Default is 0.</strong>
   </td>
  </tr>
</table>



### Return

Account balance in type of usize.


### Implementation Details

Following are the steps for implementing this method: \




*   Start generating address at index 0 with a default [gap limit](https://blog.blockonomics.co/bitcoin-what-is-this-gap-limit-4f098e52d7e1) of 20;
*   Check for balances on the generated addresses using [getBalances()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#getbalances) and keep track of the positive balances;
*   Repeat the above step till a set of addresses are found that all have zero balances;
*   Accumulate the positive balances and return the result.


## GetBalanceOfAddresses

Returns the balance in iota for the given addresses; No seed or security level needed to do this since we are only checking and already know the addresses.


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
   <td><strong>addresses</strong>
   </td>
   <td>&#10004;
   </td>
   <td>[Address]
   </td>
   <td>List of addresses with checksum.
   </td>
  </tr>
</table>



### Return

A list of tuples with value of  (Address, usize). The usize is the balance of the address accordingly. 


### Implementation details:

Following are the steps for implementing this method: \




*   Validate _address_ semantics;
*   Get latest balance for the provided address using [getBalances()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#getbalances);
*   Return the latest balance.


## Reattach

Reattaches transaction for provided transaction hash. 

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
   <td><strong>transaction_hashes</strong>
   </td>
   <td>&#10004;
   </td>
   <td>
<a href="#Hash">Hash</a>
   </td>
   <td>The hash of the transaction that need to be reattached.
   </td>
  </tr>
</table>



### Returns:

Newly reattached [Transaction](#Transaction).


### Implementation Details

Following are the steps for implementing this method: \




*   Only an unconfirmed transaction should be allowed to reattach. The method should validate the confirmation state (using [getInclusionStates()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#getinclusionstates)) of the provided transaction. If a transaction hash of a confirmed transaction is provided, the method should error out;
*   The method should also validate if the transaction reattachment is necessary. This can be done by checking if the transaction falls below max depth. The criteria of checking whether the transaction has fallen below max depth is through time. If 11 minutes have passed since the timestamp of the most recent (reattachment), the transaction can be allowed to be reattached. See [this](https://github.com/iotaledger/trinity-wallet/blob/3fab4f671c97e805a2b0ade99b4abb8b508c2842/src/shared/libs/iota/transfers.js#L141) implementation for reference;
*   Get transactions to approve using [getTransactionsToApprove()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#gettransactionstoapprove);
*   Perform proof-of-work (If _offload_pow_ is set to true, the proof-of-work should be offloaded to the selected node using [attachToTangle()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#attachtotangle). Otherwise, proof-of-work should be performed locally)
*   Store transactions on the tangle using [storeTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#storetransactions);
*   Broadcast transactions to the tangle using [broadcastTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#storetransactions). 


## IsConfirmed

Fetch inclusion states of the given transactions to determine if the transactions are confirmed.

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
   <td><strong>transaction_hashes</strong>
   </td>
   <td>&#10004;
   </td>
   <td>[<a href="#Hash">Hash</a>]
   </td>
   <td>List of transaction hashes for which you want to get the inclusion state
   </td>
  </tr>
</table>



### Returns:

List of tuples with values of the transaction [Hash](#heading=Hash)es and a bool which is the confirm state of it.
Depend on bee api in the end, this might be a enum instead of plan boolean. For instance, a node could return a state
like `unkown` saying it not sure about the state of transaction because of pruning.


### Implementation Details

Following are the steps for implementing this method: \




*   Query the confirmation state (using [getInclusionStates()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#getinclusionstates)) of the provided transaction. 
*   Return the list of transactions state tuples.


# Bee / Hornet API

API of Bee and Hornet will still be public. Users who know these relative low level API can still call them directly if they are confident and think it’s good for them. Note that both Bee and hornet
haven't finalized their APIs either. Following items and signatures might change later.


## AttachToTangle

Does proof of work for the given transaction trytes. The `branch_transaction` and `trunk_transaction` parameters are returned from the [GetMessagesToApprove](#GetMessagesToApprove) method.

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
   <td><strong>trunk_transaction</strong>
   </td>
   <td>&#10004;
   </td>
   <td>
<a href="#Hash">Hash</a>
   </td>
   <td>Trunk transaction hash provided by <a href="#GetMessagesToApprove">GetMessagesToApprove</a>.
   </td>
  </tr>
  <tr>
   <td><strong>branch_transaction</strong>
   </td>
   <td>&#10004;
   </td>
   <td>
<a href="#Hash">Hash</a>
   </td>
   <td>Branch transaction hash provided by <a href="#GetMessagesToApprove">GetMassagesToApprove</a>.
   </td>
  </tr>
  <tr>
   <td><strong>trytes</strong>
   </td>
   <td>&#10004;
   </td>
   <td>[
<a href="#Transaction">Transaction</a>]
   </td>
   <td>List of transactions. When sending transactions in a bundle, make sure that the trytes of the last transaction in the bundle are in index 0 of the array.
   </td>
  </tr>
</table>



### Returns:

List of [Message](#Message) objects which are ready to broadcast and store to tangle.


### Implementation Details

Following are the steps for implementing this method: \




*   Validate _trunk transaction hash_ semantics;
*   Validate _branch transaction hash_ semantics;
*   Validate min weight magnitude;
*   Validate trytes semantics; The last element should be the last transaction of the bundle.
*   Return the list of transactions that are attached to tangle.


## GetInclusionState

Gets the inclusion states of a set of transactions. This endpoint determines if a transaction is confirmed by the network (referenced by a valid milestone). 

Parameters


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
   <td><strong>transaction_hashes</strong>
   </td>
   <td>&#10004;
   </td>
   <td>[<a href="#Hash">Hash</a>]
   </td>
   <td>List of transaction hashes for which you want to get the inclusion state
   </td>
  </tr>
</table>



### Returns:

List of tuples with values of the transaction [Hash](#Hash)es and a bool which is the confirm state of it.


### Implementation Details

Following are the steps for implementing this method: \




*   Validate transaction hashes semantics;
*   Return the list of transactions state tuples.


## GetTransactionToApprove

Gets two consistent tip transaction hashes to use as branch/trunk transactions.

Parameters


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
   <td><strong>depth</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>usize
   </td>
   <td>Number of milestones to go back to start the tip selection algorithm. <strong>Default is 3.</strong>
   </td>
  </tr>
  <tr>
   <td><strong>reference</strong>
<p>
   </td>
   <td>&#10008;
   </td>
   <td>
<a href="#Hash">Hash</a>
   </td>
   <td>Transaction hash from which to start the weighted random walk. Use this parameter to make sure the returned tip transaction hashes approve a given reference transaction
   </td>
  </tr>
</table>



### Returns:

A tuple with trunk and branch transaction [Hash](#Hash)es.


### Implementation Details

Following are the steps for implementing this method: \




*   Validate reference hash semantics if provided;
*   Return the transactions tuple.

## BroadcastTransactions

Broadcast transactions to the connected node. This will be useful if the initial broadcast fails. 


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
   <td><strong>transactions</strong>
   </td>
   <td>&#10004;
   </td>
   <td>[<a href="#Transaction">Transaction</a>]
   </td>
   <td>List of the transactions that need to be broadcasted.
   </td>
  </tr>
</table>



### Implementation Details

Following are the steps for implementing this method: \




*   Validate _transactions_ semantics;
*   Broadcast transactions to the tangle using [broadcastTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#storetransactions).


## StoreTransactions

Store transactions to the connected node. 


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
   <td><strong>transactions</strong>
   </td>
   <td>&#10004;
   </td>
   <td>[<a href="#Transaction">Transaction</a>]
   </td>
   <td>List of the transactions that need to be stored.
   </td>
  </tr>
</table>



### Implementation Details

Following are the steps for implementing this method: \




*   Validate _transactions_ semantics;
*   Store transactions to the tangle using [storeTransactions()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#storetransactions).


## WereAddressesSpentFrom

Checks if an address was ever withdrawn from. Will be required for WOTS to Ed25519 transition.


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
   <td><strong>addresses</strong>
   </td>
   <td>&#10004;
   </td>
   <td>[Address]
   </td>
   <td>List of addresses with checksum.
   </td>
  </tr>
</table>



### Return

A list of tuples with values of  (Address, bool). The bool is the result of the address accordingly. 


### Implementation Details

Following are the steps for implementing this method: \




*   Validate _addresses_ semantics;
*   Get spend statuses using [wereAddressesSpentFrom()](https://docs.iota.org/docs/node-software/0.1/iri/references/api-reference#wereaddressesspentfrom);
*   Return the spend statuses.


# Objects

Here are the objects used in the API above. They aim to provide a secure way to handle certain data structures specified in the Iota stack.


## Network

Network will be an enumeration with elements of **[mainnet|comnet|devnet]. **Some languages might lack of type like an enum. In this case, Network can be a set of constant variables.


## Hash


<table>
  <tr>
    <td><strong>Property</strong></td>
    <td><strong>Required</strong></td>
    <td><strong>Type</strong></td>
    <td><strong>Description</strong></td>
  </tr>
  <tr>
    <td>seed</td>
    <td>&#10004;</td>
    <td>[u8; 32]</td>
    <td>A valid IOTA hash which can be treated as many objects like Address, Transaction hash, and more. The inner structure of course will instantiate the actual objects. This serves as a convenient but secure way for users passing parameters.</td>
  </tr>
</table>

## Seed



<table>
  <tr>
    <td><strong>Property</strong></td>
    <td><strong>Required</strong></td>
    <td><strong>Type</strong></td>
    <td><strong>Description</strong></td>
  </tr>
  <tr>
    <td>seed</td>
    <td>&#10004;</td>
    <td>[u8; 32]</td>
    <td>An IOTA seed that inner structure is omitted. Users can create this type by passing a String. It will verify and return an error if it’s not valid.</td>
  </tr>
</table>

## Encoding

The converter/encoder used to convert the message into bytes/trytes (whatever the transaction would need). We should offer several off-the-shelve encoders for this to set some standards, if we still use Ternary for encoding it would like this:



*   Encoding::Bytes (convert bytes to trytes using the most efficient method for this, to be defined)
*   Encoding::UTF8 (the default, converts UTF-8/Unicode to trytes which basically comes down to being an alias for converters.Bytes but maybe with another conversion function in there to convert a Unicode string to UTF-8/bytes first).
*   Encoding::Ascii (legacy fallback to Ascii character to Tryte conversion only, implemented as done in the Typescript/Go/Python lib (current Rust implementation is incomplete and does not include characters beyond alphanumeric).


## Message

The message object returned by various functions; based on the RFC for the Message object.

<table>
  <tr>
    <td><strong>Property</strong></td>
    <td><strong>Required</strong></td>
    <td><strong>Type</strong></td>
    <td><strong>Description</strong></td>
  </tr>
  <tr>
    <td>version</td>
    <td>&#10004;</td>
    <td>number</td>
    <td>Message version. Defaults to `1`.</td>
  </tr>
  <tr>
    <td>trunk</td>
    <td>&#10004;</td>
    <td>string</td>
    <td>Message id of the first message this message refers to.</td>
  </tr>
  <tr>
    <td>branch</td>
    <td>&#10004;</td>
    <td>string</td>
    <td>Message id of the second message this message refers to.</td>
  </tr>
  <tr>
    <td>payload_length</td>
    <td>&#10004;</td>
    <td>number</td>
    <td>Length of the payload.</td>
  </tr>
    <tr>
    <td>payload</td>
    <td>&#10004;</td>
    <td>
        <a href="#signedtransactionpayload">SignedTransactionPayload</a> |
        <a href="#unsigneddatapayload">UnsignedDataPayload</a> |
        <a href="#signeddatapayload">SignedDataPayload</a>
    </td>
    <td>Transaction amount (exposed as a custom type with additional methods).</td>
  </tr>
  <tr>
    <td>timestamp</td>
    <td>&#10004;</td>
    <td><a href="#timestamp">Timestamp</a></td>
    <td>Transaction timestamp (exposed as a custom type with additional methods).</td>
  </tr>
  <tr>
    <td>nonce</td>
    <td>&#10004;</td>
    <td>string</td>
    <td>Transaction nonce.</td>
  </tr>
  <tr>
    <td>confirmed</td>
    <td>&#10004;</td>
    <td>boolean</td>
    <td>Determines if the transaction is confirmed.</td>
  </tr>
</table>
