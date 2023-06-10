---
description: Official IOTA Client Library Python API reference.
image: /img/logo/libraries.png
keywords:
- api
- python
- param
- type
- run
---
# IOTA Client Python Library API Reference 

Note that in the following APIs, the corresponding exception will be returned if an error occurs.
Also for all the optional values, the default values are the same as the ones in the Rust version.

### Client

#### constructor(network (optional), storage (optional), password (optional), polling_interval (optional)): [AccountManager](#accountmanager)

Creates a new instance of the Client.

| Param                                | Type                              | Default     | Description                                                                                                                                                                                                                                              |
| ------------------------------------ | --------------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [network]                            | `str`                             | `undefined` | The network                                                                                                                                                                                                                                              |
| [primary_node_jwt_name_password]     | `list[str]`                       | `undefined` | An array of array with node URLs and optional JWT and basic auth name and password (length 1 is only the url, length 2 is url with JWT, length 3 is url with basic auth name and password and length 4 is url with JWT and basic auth name and password) |
| [primary_pow_node_jwt_name_password] | `list[str]`                       | `undefined` | An array of array with node URLs and optional JWT and basic auth name and password (length 1 is only the url, length 2 is url with JWT, length 3 is url with basic auth name and password and length 4 is url with JWT and basic auth name and password) |
| [nodes_name_password]                | `list[]list[str]`                 | `undefined` | An array of array with node URLs and optional JWT and basic auth name and password (length 1 is only the url, length 2 is url with JWT, length 3 is url with basic auth name and password and length 4 is url with JWT and basic auth name and password) |
| [permanodes_name_password]           | `list[]list[str]`                 | `undefined` | An array of array with node URLs and optional JWT and basic auth name and password (length 1 is only the url, length 2 is url with JWT, length 3 is url with basic auth name and password and length 4 is url with JWT and basic auth name and password) |
| [node_sync_interval]                 | `int`                             | `undefined` | The interval for the node syncing process                                                                                                                                                                                                                |
| [node_sync_disabled]                 | `bool`                            | `undefined` | Disables the node syncing process. Every node will be considered healthy and ready to use                                                                                                                                                                |
| [node_pool_urls]                     | `str`                             | `undefined` | An array of node pool URLs                                                                                                                                                                                                                               |
| [quorum]                             | `bool`                            | `false`     | Bool to define if quorum should be used                                                                                                                                                                                                                  |
| [quorum_size]                        | `int`                             | `3`         | An int that defines how many nodes should be used for quorum                                                                                                                                                                                             |
| [quorum_threshold]                   | `int`                             | `66`        | Define the % of nodes that need to return the same response to accept it                                                                                                                                                                                 |
| [request_timeout]                    | `int`                             | `undefined` | Sets the default HTTP request timeout                                                                                                                                                                                                                    |
| [api_timeout]                        | `dict`                            | `undefined` | The API to set the request timeout. Key: 'GetHealth', 'GetInfo', 'GetPeers', 'GetTips', 'PostMessage', 'GetOutput', 'GetMilestone' Value: timeout in milliseconds                                                                                        |
| [local_pow]                          | `bool`                            | `undefined` | Flag determining if PoW should be done locally or remotely                                                                                                                                                                                               |
| [tips_interval]                      | `int`                             | `undefined` | Time between requests for new tips during PoW                                                                                                                                                                                                            |
| [mqtt_broker_options]                | `[BrokerOptions](#brokeroptions)` | `undefined` | Sets the options for the MQTT connection with the node                                                                                                                                                                                                   |

**Returns** The constructed [Client](#client).

### Full Node APIs

#### get_health(): bool

Gets the node health status.

**Returns** whether the node is healthy.

#### get_info(): NodeInfoWrapper

Gets information about the node.

**Returns** the [NodeInfoWrapper](#nodeinfowrapper).

#### get_peers(): list[PeerDto]

Gets peers of the node.

**Returns** the list of [PeerDto](#peerdto).

#### get_tips(): list[str]

Gets non-lazy tips.

**Returns** two non-lazy tips' message ids in list.

#### post_message(msg): str

Submits a message.

| Param | Type                  | Default     | Description           |
| ----- | --------------------- | ----------- | --------------------- |
| [msg] | `[Message](#message)` | `undefined` | The message to submit |

**Returns** the message id of the submitted message.

#### get_output(output_id): OutputResponse

Gets the UTXO outputs associated with the given output id.

| Param       | Type  | Default     | Description                    |
| ----------- | ----- | ----------- | ------------------------------ |
| [output_id] | `str` | `undefined` | The id of the output to search |

**Returns** the OutputResponse[#outputresponse].

#### get_address_balance(address): BalanceAddressResponse

Gets the balance in the address.

| Param     | Type        | Default     | Description               |
| --------- | ----------- | ----------- | ------------------------- |
| [address] | `list[str]` | `undefined` | The address Bech32 string |

**Returns** the [BalanceAddressResponse](#BalanceAddressResponse).

#### get_address_outputs(address, options (optional)): list[UtxoInput]

Gets the UTXO outputs associated with the given address.

| Param     | Type                                                | Default     | Description               |
| --------- | --------------------------------------------------- | ----------- | ------------------------- |
| [address] | `str`                                               | `undefined` | The address Bech32 string |
| [options] | `[[AddressOutputsOptions](#addressoutputsoptions)]` | `undefined` | The query filters         |

**Returns** the list of [UtxoInput](#UtxoInput).

#### find_outputs(output_ids (optional), addresses (optional)): list[OutputResponse]

Gets the UTXO outputs associated with the given output ids and addresses.

| Param        | Type        | Default     | Description                      |
| ------------ | ----------- | ----------- | -------------------------------- |
| [output_ids] | `list[str]` | `undefined` | The list of addresses to search  |
| [addresses]  | `list[str]` | `undefined` | The list of output ids to search |

**Returns** the list of [OutputResponse](#outputresponse).

#### get_milestone(index): MilestoneDto

Gets the milestone by the given index.

| Param   | Type  | Default     | Description                |
| ------- | ----- | ----------- | -------------------------- |
| [index] | `int` | `undefined` | The index of the milestone |

**Returns** the [MilestoneDto](#milestonedto).

#### get_milestone_utxo_changes(index): MilestoneUTXOChanges

Gets the utxo changes by the given milestone index.

| Param   | Type  | Default     | Description                |
| ------- | ----- | ----------- | -------------------------- |
| [index] | `int` | `undefined` | The index of the milestone |

**Returns** the [MilestoneUTXOChanges](#milestoneutxochanges).

#### get_receipts(): Vec<ReceiptDto/>

Get all receipts.

**Returns** the [ReceiptDto](#ReceiptDto).

#### get_receipts_migrated_at(index): Vec<ReceiptDto/>

Get all receipts for a given milestone index.

| Param   | Type  | Default     | Description                |
| ------- | ----- | ----------- | -------------------------- |
| [index] | `int` | `undefined` | The index of the milestone |

**Returns** the [ReceiptDto](#ReceiptDto).

#### get_treasury(): TreasuryResponse

Get the treasury amount.

**Returns** the [TreasuryResponse](#TreasuryResponse).

#### get_included_message(): Message

Get the included message of a transaction.

| Param   | Type  | Description               |
| ------- | ----- | ------------------------- |
| [index] | `str` | The id of the transaction |

**Returns** the new [Message](#message).


### High-Level APIs

#### message(seed (optional), account_index (optional), initial_address_index (optional), inputs (optional), input_range_begin (optional), input_range_end (optional), outputs (optional), dust_allowance_outputs (optional), index (optional), index_raw (optional), data (optional), data_str (optional), parents (optional)): Message

Build a message.

| Param                    | Type                                       | Default                | Description                                  |
| ------------------------ | ------------------------------------------ | ---------------------- | -------------------------------------------- |
| [seed]                   | `str`                                      | `undefined`            | The hex-encoded seed of the account to spend |
| [account_index]          | `int`                                      | `undefined`            | The account index                            |
| [initial_address_index]  | `int`                                      | `undefined`            | The initial address index                    |
| [inputs]                 | <code>list[[UtxoInput](#utxoinput)]</code> | <code>undefined</code> | UtxoInputs                                   |
| [input_range_begin]      | `int`                                      | `undefined`            | The begin index of the input                 |
| [input_range_end]        | `int`                                      | `undefined`            | The end index of the input                   |
| [outputs]                | `list[[Output](#output)]`                  | `undefined`            | Outputs                                      |
| [dust_allowance_outputs] | `list[[Output](#output)]`                  | `undefined`            | Dust allowance output to the transaction     |
| [index]                  | `str`                                      | `undefined`            | The indexation string                        |
| [index_raw]              | `list[int]`                                | `undefined`            | The indexation byte array                    |
| [data]                   | `list[int]`                                | `undefined`            | The data in bytes                            |
| [data_str]               | `str`                                      | `undefined`            | The data string                              |
| [parents]                | `list[str]`                                | `undefined`            | The message ids of the parents               |

**Returns** the built [Message](#message).

#### get_message_metadata(message_id): MessageMetadataResponse

| Param        | Type  | Default     | Description    |
| ------------ | ----- | ----------- | -------------- |
| [message_id] | `str` | `undefined` | The message id |

**Returns** the [MessageMetadataResponse](#messagemetadataresponse).

#### get_message_data(message_id): Message

Gets the message data from the message id.

| Param        | Type  | Default     | Description    |
| ------------ | ----- | ----------- | -------------- |
| [message_id] | `str` | `undefined` | The message id |

**Returns** the [Message](#message).

#### get_message_raw(message_id): str

Gets the raw message string from the message id.

| Param        | Type  | Default     | Description    |
| ------------ | ----- | ----------- | -------------- |
| [message_id] | `str` | `undefined` | The message id |

**Returns** the raw message string.

#### get_message_children(message_id): list[str]

Gets the children of the given message.

| Param        | Type  | Default     | Description    |
| ------------ | ----- | ----------- | -------------- |
| [message_id] | `str` | `undefined` | The message id |

**Returns** the list of children strings.

#### get_message_id(payload_str): str

Get the message id from the payload string.

| Param       | Type  | Default     | Description                                    |
| ----------- | ----- | ----------- | ---------------------------------------------- |
| payload_str | `str` | `undefined` | The payload string from the mqtt message event |

**Returns** The identifier of message.

#### get_message_index(index): list[str]

Gets the list of message indices from the message_id.

| Param   | Type  | Default     | Description               |
| ------- | ----- | ----------- | ------------------------- |
| [index] | `str` | `undefined` | The identifier of message |

**Returns** the list of message ids.

#### find_messages(indexation_keys (optional), message_ids (optional)): list[Message]

Finds all messages associated with the given indexation keys and message ids.

| Param             | Type        | Default     | Description                             |
| ----------------- | ----------- | ----------- | --------------------------------------- |
| [indexation_keys] | `list[str]` | `undefined` | The list of indexations keys too search |
| [message_ids]     | `list[str]` | `undefined` | The list of message ids to search       |

**Returns** the list of the found messages.

#### get_unspent_address(seed, account_index (optional), initial_address_index(optional)): (str, int)

Gets a valid unspent address.

| Param                   | Type  | Default     | Description                    |
| ----------------------- | ----- | ----------- | ------------------------------ |
| [seed]                  | `str` | `undefined` | The hex-encoded seed to search |
| [account_index]         | `int` | `undefined` | The account index              |
| [initial_address_index] | `int` | `undefined` | The initial address index      |

**Returns** a tuple with type of `(str, int)` as the address and corresponding index in the account.

#### get_addresses(seed, account_index (optional), input_range_begin (optional), input_range_end (optional) get_all (optional)): list[(str, bool (optional))]

Finds addresses from the seed regardless of their validity.

| Param               | Type   | Default     | Description                    |
| ------------------- | ------ | ----------- | ------------------------------ |
| [seed]              | `str`  | `undefined` | The hex-encoded seed to search |
| [account_index]     | `int`  | `undefined` | The account index              |
| [input_range_begin] | `int`  | `undefined` | The begin of the address range |
| [input_range_end]   | `int`  | `undefined` | The end of the address range   |
| [get_all]           | `bool` | `undefined` | Get all addresses              |

**Returns** a list of tuples with type of `(str, int)` as the address and corresponding index in the account.

#### get_balance(seed, account_index (optional), initial_address_index(optional), gap_limit(optional)): int

Get balance on a given seed and its wallet account index.

| Param                   | Type  | Default     | Description                    |
| ----------------------- | ----- | ----------- | ------------------------------ |
| [seed]                  | `str` | `undefined` | The hex-encoded seed to search |
| [account_index]         | `int` | `undefined` | The account index              |
| [initial_address_index] | `int` | `undefined` | The initial address index      |
| [gap_limit]             | `int` | `undefined` | The gap limit                  |

**Returns** the amount of balance.

#### get_address_balances(addresses): list[AddressBalancePair]

Get the balance in iotas for the given addresses.

| Param       | Type        | Default     | Description                     |
| ----------- | ----------- | ----------- | ------------------------------- |
| [addresses] | `list[str]` | `undefined` | The list of addresses to search |

**Returns** the list of [AddressBalancePair](#addressbalancepair).

#### generate_mnemonic()

Returns a random generated Bip39 mnemonic with the English word list.

**Returns** A String

#### mnemonic_to_hex_seed(mnemonic)

Returns the seed hex encoded.

| Param    | Type  | Default     | Description                                           |
| -------- | ----- | ----------- | ----------------------------------------------------- |
| mnemonic | `str` | `undefined` | Bip39 mnemonic with words from the English word list. |

**Returns** A String

#### find_inputs(addresses, amount: u64)

Return the inputs from addresses for a provided amount (useful for offline signing)

| Param     | Type        | Default     | Description             |
| --------- | ----------- | ----------- | ----------------------- |
| addresses | `list[str]` | `undefined` | The input address list. |
| amount    | `str`       | `undefined` | The input amount.       |


**Returns** The list of [UtxoInput](#utxoinput).

#### bech32_to_hex(bech32)

Returns a parsed hex String from bech32.

| Param  | Type  | Default     | Description               |
| ------ | ----- | ----------- | ------------------------- |
| bech32 | `str` | `undefined` | The address Bech32 string |

**Returns** A String

#### hex_to_bech32(hex, bech32_hrp (optional))

Returns a parsed bech32 String from hex.

| Param      | Type  | Default     | Description               |
| ---------- | ----- | ----------- | ------------------------- |
| bech32     | `str` | `undefined` | The address Bech32 string |
| bech32_hrp | `str` | `undefined` | The Bech32 hrp string     |

**Returns** A String

#### hex_public_key_to_bech32_address(hex, bech32_hrp (optional))

Returns the bech32 address from the hex public key.

| Param      | Type  | Default     | Description            |
| ---------- | ----- | ----------- | ---------------------- |
| hex        | `str` | `undefined` | Hex encoded public key |
| bech32_hrp | `str` | `undefined` | The Bech32 hrp string  |

**Returns** A String

#### is_address_valid(address): bool

Checks if a given address is valid.

| Param   | Type  | Default     | Description               |
| ------- | ----- | ----------- | ------------------------- |
| address | `str` | `undefined` | The address Bech32 string |

**Returns** A boolean.

#### retry(message_id): (str, Message)

Retries (promotes or reattaches) the message associated with the given id.

| Param        | Type  | Default     | Description    |
| ------------ | ----- | ----------- | -------------- |
| [message_id] | `str` | `undefined` | The message id |

**Returns** the message id and the retried [Message](#message).

#### retry_until_included(message_id, interval (optional), max_attempts (optional)): list[(str, Message)]

Retries (promotes or reattaches) the message associated with the given id.

| Param        | Type  | Default     | Description                                            |
| ------------ | ----- | ----------- | ------------------------------------------------------ |
| [message_id] | `str` | `undefined` | The message id                                         |
| interval     | `int` | `5`         | The interval in seconds in which we retry the message. |
| max_attempts | `int` | `40`        | The maximum of attempts we retry the message.          |

**Returns** the message ids and [Message](#message) of reattached messages.

#### consolidate_funds(seed, account_index, start_index, end_index): str

Function to consolidate all funds from a range of addresses to the address with the lowest index in that range

| Param           | Type  | Description                                                           |
| --------------- | ----- | --------------------------------------------------------------------- |
| [seed]          | `str` | The seed                                                              |
| [account_index] | `int` | The account index.                                                    |
| [start_index]   | `int` | The lowest address index, funds will be consolidated to this address. |
| [end_index]     | `int` | The address index until which funds will be consolidated              |

**Returns** the address to which the funds got consolidated, if any were available.

#### search_address(seed, bech32_hrp, account_index, start_index, end_index, address): (int, bool)

Function to find the index and address type of an address

| Param           | Type                | Description                      |
| --------------- | ------------------- | -------------------------------- |
| [seed]          | <code>str</code>    | The seed                         |
| [bech32_hrp]    | <code>string</code> | The Bech32 HRP                   |
| [account_index] | <code>int</code>    | The account index                |
| [start_index]   | <code>int</code>    | The start address index          |
| [end_index]     | <code>int</code>    | The end address index (excluded) |
| [address]       | <code>str</code>    | The address Bech32 string        |

**Returns** index and address type of an address.

#### reattach(message_id): (str, Message)

Reattaches the message associated with the given id.

| Param        | Type  | Default     | Description    |
| ------------ | ----- | ----------- | -------------- |
| [message_id] | `str` | `undefined` | The message id |

**Returns** the message id and the reattached [Message](#message).

#### promote(message_id): (str, Message)

Promotes the message associated with the given id.

| Param        | Type  | Default     | Description    |
| ------------ | ----- | ----------- | -------------- |
| [message_id] | `str` | `undefined` | The message id |

**Returns** the message id and the promoted [Message](#message).

### MQTT APIs

#### subscribe_topic(topic, callback): void

Subscribe a topic and assign the associated callback.

| Param      | Type       | Default     | Description           |
| ---------- | ---------- | ----------- | --------------------- |
| [topic]    | `str`      | `undefined` | The MQTT topic        |
| [callback] | `function` | `undefined` | The callback function |

#### subscribe_topics(topics, callback): void

Subscribe topics and assign the associated callbacks, respectively.

| Param      | Type        | Default     | Description            |
| ---------- | ----------- | ----------- | ---------------------- |
| [topics]   | `list[str]` | `undefined` | The MQTT topics        |
| [callback] | `function`  | `undefined` | The callback functions |

#### unsubscribe(): void

Unsubscribe all topics.

#### disconnect(): void

Disconnect the mqtt broker.

#### WalletAddress

A dict with the following key/value pairs.

```python
message_metadata_response = {
    'message_id': str,
    'parent_message_ids': list[str],
    'is_solid': bool,
    'referenced_by_milestone_index': int, # (optional)
    'milestone_index': int,  # (optional)
    'ledger_inclusion_state': LedgerInclusionStateDto,  # (optional)
    'conflict_reason': int,  # (optional)
    'should_promote:' bool  # (optional)
    'should_reattach': bool  # (optional)
}
```

Please refer to [LedgerInclusionStateDto](#ledgerinclusionstatedto) for the details of this type.

#### BalanceAddressResponse

A dict with the following key/value pairs.

```python
balance_for_address_response = {
    'address_type': int,
    'address': str,
    'balance': int
}
```

#### AddressBalancePair

A dict with the following key/value pairs.

```python
address_balance_pair = {
    'address': str,
    'balance': int
    'dust_allowed': bool
}
```

#### MilestoneDto

A dict with the following key/value pairs.

```python
milestoned_to = {
    'index': int,
    'timestamp': int,
    'message_id':  str
}
```

#### MilestoneUTXOChanges

A dict with the following key/value pairs.

```python
milestone_utxo_changes = {
    'index': int,
    'created_outputs': list[str],
    'consumed_outputs': list[str]
}
```

#### ReceiptDto

A dict with the following key/value pairs.

```python
receiptDto = {
    'receipt': Receipt,
    'milestone_index': int,
}
```

#### TreasuryResponse

A dict with the following key/value pairs.

```python
treasuryResponse = {
    'milestone_id': str,
    'amount': int,
}
```

#### UtxoInput

A dict with the following key/value pairs.

```python
utxo_input = {
    'transaction_id': list[int],
    'index': int
}
```

#### OutputResponse

A dict with the following key/value pairs.

```python
output_response = {
    'message_id': str,
    'transaction_id': str,
    'output_index': int,
    'is_spent': bool,
    'output': OutputDto
}
```

Please refer to [OutputDto](#outputdto) for the details of this type.

#### OutputDto

A dict with the following key/value pairs.

```python
output_dto = {
    'treasury': TreasuryOutputDto, # (opitonal)
    'signature_locked_single': SignatureLockedSingleOutputDto, # (opitonal)
    'signature_locked_dust_allowance': SignatureLockedDustAllowanceOutputDto # (opitonal)
}
```

Please refer to [TreasuryOutputDto](#treasuryoutputdto), [SignatureLockedSingleOutputDto](#signaturelockedsingleoutputdto), and [SignatureLockedDustAllowanceOutputDto](#signaturelockedDustallowanceoutputdto) for the details of these types.

#### SignatureLockedSingleOutputDto

A dict with the following key/value pairs.

```python
signature_locked_single_output_dto = {
    'kind': int,
    'address': AddressDto,
    'amount': int
}
```

Please refer to [AddressDto](#addressdto) for the details of this type.

#### SignatureLockedDustAllowanceOutputDto

A dict with the following key/value pairs.

```python
signature_locked_dust_allowance_output_dto = {
    'kind': int,
    'address': AddressDto,
    'amount': int
}
```

Please refer to [AddressDto](#addressdto) for the details of this type.

#### pub struct TreasuryOutputDto {


A dict with the following key/value pairs.

```python
treasury_output_dto = {
    'kind': int,
    'amount':int
}
```

#### AddressDto

A dict with the following key/value pairs.

```python
address_dto = {
    'ed25519': Ed25519AddressDto
}
```

Please refer to [Ed25519AddressDto](#ed25519addressdto) for the details of this type.

#### Ed25519AddressDto

A dict with the following key/value pairs.

```python
ed25519_address_dto = {
    'kind': int,
    'address': str
}
```

#### Message

A dict with the following key/value pairs.

```python
message = {
    'message_id': str,
    'network_id': int,
    'parents': list[str],
    'payload': Payload, # (optional)
    'nonce': int
}
```

Please refer to [Payload](#payload) for the details of this type.

#### Payload

A dict with the following key/value pairs.

```python
payload = {
    'transaction': list[Transaction], # (optional)
    'milestone': list[Milestone], # (optional)
    'indexation': list[Indexation], # (optional)
}
```

Please refer to [Transaction](#transaction), [Milestone](#milestone), and [Indexation](#indexation) for the details of these types.

#### Transaction

A dict with the following key/value pairs.

```python
transaction = {
    'essence': RegularEssence,
    'unlock_blocks': list[UnlockBlock]
}
```

Please refer to [RegularEssence](#regularessence), and [UnlockBlock](#unlockblock) for the details of these types.

#### Milestone

A dict with the following key/value pairs.

```python
milestone = {
    'essence': MilestonePayloadEssence,
    'signatures': list[list[int]]
}
```

Please refer to [MilestonePayloadEssence](#milestonepayloadessence) for the details of this type.

#### MilestonePayloadEssence

A dict with the following key/value pairs.

```python
milestone_payload_essence = {
    'index': int,
    'timestamp': int,
    'parents': list[str],
    'merkle_proof': list[int],
    'next_pow_score': int,
    'next_pow_score_milestone_index': int,
    'public_keys': list[list[int]]
}
```

#### Indexation

A dict with the following key/value pairs.

```python
indexation = {
    'index': str,
    'data': list[int]
}
```

#### RegularEssence

A dict with the following key/value pairs.

```python
regular_essence = {
    'inputs': list[Input],
    'outputs': list[Output],
    'payload': list[Payload]
}
```

Please refer to [Input](#input), [Output](#output), and [Payload](#payload) for the details of these types.

#### Output

A dict with the following key/value pairs.

```python
output = {
    'address': str,
    'amount': int
}
```

#### Input

A dict with the following key/value pairs.

```python
input = {
    'transaction_id': str,
    'index': int
}
```

#### UnlockBlock

A dict with the following key/value pairs.

```python
unlock_block = {
    'signature': Ed25519Signature, # (optional)
    'reference': int # (optional)
}
```

Please refer to [Ed25519Signature](#ed25519Signature) for the details of this type.

#### Ed25519Signature

A dict with the following key/value pairs.

```python
ed25519_signature = {
    'public_key': list[int],
    'signature': list[int]
}
```

#### BrokerOptions

A dict with the following key/value pairs.

```python
broker_options = {
    'automatic_disconnect': bool,
    'timeout': int,
    'max_reconnection_attempts': int,
}
```

#### LedgerInclusionStateDto

A dict with the following key/value pairs.

```python
ledger_inclusion_state_dto = {
    'state': str
}
```

#### NodeInfoWrapper

A dict with the following key/value pairs.

```python
nodeinfo_wrapper{
    url: str,
    nodeinfo: info_response,
}
info_response = {
    'name': str,
    'version': str,
    'is_healthy': bool,
    'network_id': str,
    'bech32_hrp': str,
    'min_pow_score': float,
    'messages_per_second': float,
    'referenced_messages_per_second': float,
    'referenced_rate': float,
    'latest_milestone_timestamp': u64,
    'latest_milestone_index': int,
    'confirmed_milestone_index': int,
    'pruning_index': int,
    'features': list[str],
    'min_pow_score': float,
}
```

#### NetworkInfo

A dict with the following key/value pairs.

```python
network_info = {
    'network': str,
    'network_id': int,
    'bech32_hrp': str,
    'min_pow_score': float,
    'local_pow': bool,
    'tips_interval': int,
}
```

#### PeerDto

A dict with the following key/value pairs.

```python
peer_dto = {
    'id': str,
    'multi_addresses': list[str],
    'alias': str, # (optional)
    'relation': RelationDto,
    'connected': bool,
    'gossip': GossipDto, # (optional)
}
```

Please refer to [RelationDto](#relationdto) and [GossipDto](#gossipdto) for the details of these types.

#### RelationDto

A dict with the following key/value pairs.

```python
relation_dto = {
    'relation': str
}
```

#### GossipDto

A dict with the following key/value pairs.

```python
gossip_dto = {
    'heartbeat': HeartbeatDto,
    'metrics': MetricsDto
}
```

Please refer to [HeartbeatDto](#heartbeatdto) and [MetricsDto](#metricsdto) for the details of these types.

#### HeartbeatDto

A dict with the following key/value pairs.

```python
heart_beat_dto = {
    'solid_milestone_index': int,
    'pruned_milestone_index': int,
    'latest_milestone_index': int,
    'connected_neighbors': int,
    'synced_neighbors': int
}
```

#### MetricsDto

A dict with the following key/value pairs.

```python
metrics_dto = {
    'received_messages': int,
    'known_messages': int,
    'received_message_requests': int,
    'received_milestone_requests': int,
    'received_heartbeats': int,
    'sent_messages': int,
    'sent_message_requests': int,
    'sent_milestone_requests': int,
    'sent_heartbeats': int,
    'dropped_packets': int,
}
```

#### AddressOutputsOptions

A dict with the following key/value pairs.

```python
options = {
    'include_spent': bool,
    'output_type': string
}
