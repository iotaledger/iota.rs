# IOTA Client Python Library

## Requirements
- Rust 1.45.0+
- Python 3.6+

## Try Run w/ Local Hornet

1. Run your local Hornet
- `$ git clone git@github.com:gohornet/hornet.git`
- checkout `chrysalis-pt2` branch
- Modify your `create_snapshot_alphanet.sh`, modify Line 14 to `go run ../main.go tool snapgen alphanet1 96f9de0989e77d0e150e850a5a600e83045fa57419eaf3b20225b763d4e23813 snapshots/alphanet1/full_export.bin`
- `$ ./run_coo_bootstrap.sh `

2. Build the iota-client-python library by yourself
- Go to `bindings/python/native`
- `$ cargo build --release`
- The built library is located in `target/release/`
- On MacOS, rename `libiota_client.dylib` to `iota_client.so`, on Windows, use `iota_client.dll` directly, and on Linux `libiota_client.so` to `iota_client.so`.
- Copy your renamed library to `bindings/python/examples/`
- Go to `bindings/python/examples`
- `$ python example.py`

## Python Example
```python
import iota_client
import os
LOCAL_NODE_URL = "http://0.0.0.0:14265"

# NOTE! Load the seed from your env path instead
# NEVER assign the seed directly in your codes!
# DO NOT USE THIS!!:
# SEED = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2"

# USE THIS INSTEAD
SEED = os.getenv('MY_IOTA_SEED')

EMPTY_ADDRESS = "iot1qxgamuxntdxq06q4zpmvmdnrerj2f94058ge3flfyx567unw25amvr978uw"
client = iota_client.Client(
    node=LOCAL_NODE_URL, node_sync_disabled=True)


def main():
    print('get_health()')
    print(f'health: client.get_health()')

    print('get_info()')
    print(f'node_info: client.get_info()')

    print('get_tips()')
    print(f'tips: client.get_tips()')

    print('find_addresses')
    address_changed_list = client.find_addresses(
        seed=SEED, account_index=0, begin=0, end=10, get_all=True)
    print(f'address_changed list: {address_changed_list}')

    # Get the (address, changed ) for the first found address
    address, changed = address_changed_list[0]
    print(f'get_address_balance() for address {address}')
    print(f'balance: {client.get_address_balance(address)}')

    print(f'get_address_balance() for address {EMPTY_ADDRESS}')
    print(f'balance: {client.get_address_balance(EMPTY_ADDRESS)}')

    print(f'get_address_outputs() for address {EMPTY_ADDRESS}')
    print(f'outputs(): {client.get_address_outputs(EMPTY_ADDRESS)}')

    print(f'message() 100 tokens to address {EMPTY_ADDRESS}')
    message_id = client.message(
        seed=SEED, outputs=[{'address': EMPTY_ADDRESS, 'amount': 100}])
    print(f'Token sent with message_id: {message_id}')
    print(f'Please check http://127.0.0.1:14265/api/v1/messages/{message_id}')

    print(f'get_message_metadata() for message_id {message_id}')
    message_metadata = client.get_message_metadata(message_id)
    print(f'message_metadata: {message_metadata}')

    print(f'get_message_data() for message_id {message_id}')
    message_data = client.get_message_data(message_id)
    print(f'message_data: {message_data}')

    print(f'get_message_raw() for message_id {message_id}')
    message_raw = client.get_message_raw(message_id)
    print(f"message_raw: {bytearray(message_raw, 'utf-8')}")

    print(f'get_message_children() for message_id {message_id}')
    children = client.get_message_children(message_id)
    print(f"children: {children}")

    print(f'message() Indexation')
    message_id_indexation = client.message(
        index="Hello", data=bytes("Tangle", 'utf-8'))
    print(f'Indexation sent with message_id: {message_id_indexation}')
    print(
        f'Please check http://127.0.0.1:14265/api/v1/messages/{message_id_indexation}')

    print(f"get_message_index() for index 'Hello'")
    message_id_indexation_queried = client.get_message_index("Hello")
    print(f'Indexation: {message_id_indexation_queried}')

    print(f"find_messages() for indexation_keys = ['Hello']")
    messages = client.find_messages(indexation_keys=["Hello"])
    print(f'Messages: {messages}')

    print(f"get_unspent_address()")
    unspent_addresses = client.get_unspent_address(seed=SEED)
    print(f'(unspent_address, index): {unspent_addresses}')

    print(f"get_balance()")
    balance = client.get_balance(seed=SEED)
    print(f'balance: {balance}')

    addresses = []
    for address, _changed in address_changed_list:
        addresses.append(address)
    print(f"get_address_balances() for {addresses}")
    balances = client.get_address_balances(addresses)
    print(f'balances: {balance}')


if __name__ == "__main__":
    main()
```

## API Reference

Note that in the following APIs, the corresponding exception will be returned if an error occurs.
Also for all the optional values, the default values are the same as the ones in the Rust version.

### Client

#### constructor(network (optional), storage (optional), password (optional), polling_interval (optional)): [AccountManager](#accountmanager)

Creates a new instance of the Client.

| Param                 | Type                                         | Default                | Description                                                                                                                                                       |
| --------------------- | -------------------------------------------- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [network]             | <code>str</code>                             | <code>undefined</code> | The network                                                                                                                                                       |
| [node]                | <code>str</code>                             | <code>undefined</code> | A node URL                                                                                                                                                        |
| [nodes]               | <code>list[str]</code>                       | <code>undefined</code> | An array of node URLs                                                                                                                                             |
| [node_sync_interval]  | <code>int</code>                             | <code>undefined</code> | The interval for the node syncing process                                                                                                                         |
| [node_sync_disabled]  | <code>bool</code>                            | <code>undefined</code> | Disables the node syncing process. Every node will be considered healthy and ready to use                                                                         |
| [node_pool_urls]      | <code>str</code>                             | <code>undefined</code> | An array of node pool URLs                                                                                                                                        |
| [request_timeout]     | <code>int</code>                             | <code>undefined</code> | Sets the default HTTP request timeout                                                                                                                             |
| [api_timeout]         | <code>dict</code>                            | <code>undefined</code> | The API to set the request timeout. Key: 'GetHealth', 'GetInfo', 'GetPeers', 'GetTips', 'PostMessage', 'GetOutput', 'GetMilestone' Value: timeout in milliseconds |
| [local_pow]           | <code>bool</code>                            | <code>undefined</code> | Flag determining if PoW should be done locally or remotely                                                                                                        |
| [tips_interval]       | <code>int</code>                             | <code>undefined</code> | Time between requests for new tips during PoW                                                                                                                     |
| [mqtt_broker_options] | <code>[BrokerOptions](#brokeroptions)</code> | <code>undefined</code> | Sets the options for the MQTT connection with the node                                                                                                            |

**Returns** The constructed [Client](#client).

### Full Node APIs

#### get_health(): bool

Gets the node health status.

**Returns** whether the node is healthy.

#### get_info(): InfoResponse

Gets information about the node.

**Returns** the [InfoResponse](#inforesponse).

#### get_peers(): list[PeerDto]

Gets peers of the node.

**Returns** the list of [PeerDto](#peerdto).

#### get_tips(): list[str]

Gets non-lazy tips.

**Returns** two non-lazy tips' message ids in list.

#### post_message(msg): str

Submits a message.

| Param | Type                             | Default                | Description           |
| ----- | -------------------------------- | ---------------------- | --------------------- |
| [msg] | <code>[Message](#message)</code> | <code>undefined</code> | The message to submit |

**Returns** the message id of the submitted message.

#### get_output(output_id): OutputResponse

Gets the UTXO outputs associated with the given output id.

| Param       | Type             | Default                | Description                    |
| ----------- | ---------------- | ---------------------- | ------------------------------ |
| [output_id] | <code>str</code> | <code>undefined</code> | The id of the output to search |

**Returns** the OutputResponse[#outputresponse].

#### get_address_balance(address): BalanceForAddressResponse

Gets the balance in the address.

| Param     | Type                   | Default                | Description               |
| --------- | ---------------------- | ---------------------- | ------------------------- |
| [address] | <code>list[str]</code> | <code>undefined</code> | The address Bech32 string |

**Returns** the [BalanceForAddressResponse](#balanceforaddressresponse).

#### get_address_outputs(address): list[UTXOInput]

Gets the UTXO outputs associated with the given address.

| Param     | Type             | Default                | Description               |
| --------- | ---------------- | ---------------------- | ------------------------- |
| [address] | <code>str</code> | <code>undefined</code> | The address Bech32 string |

**Returns** the list of [UTXOInput](#utxoinput).

#### find_outputs(output_ids (optional), addresses (optional)): list[OutputResponse]

Gets the UTXO outputs associated with the given output ids and addresses.

| Param        | Type                   | Default                | Description                      |
| ------------ | ---------------------- | ---------------------- | -------------------------------- |
| [output_ids] | <code>list[str]</code> | <code>undefined</code> | The list of addresses to search  |
| [addresses]  | <code>list[str]</code> | <code>undefined</code> | The list of output ids to search |

**Returns** the list of [OutputResponse](#outputresponse).

#### get_milestone(index): MilestoneDto

Gets the milestone by the given index.

| Param   | Type             | Default                | Description                |
| ------- | ---------------- | ---------------------- | -------------------------- |
| [index] | <code>int</code> | <code>undefined</code> | The index of the milestone |

**Returns** the [MilestoneDto](#milestonedto).

#### get_milestone_utxo_changes(index): MilestoneUTXOChanges

Gets the utxo changes by the given milestone index.

| Param   | Type             | Default                | Description                |
| ------- | ---------------- | ---------------------- | -------------------------- |
| [index] | <code>int</code> | <code>undefined</code> | The index of the milestone |

**Returns** the [MilestoneUTXOChanges](#milestoneutxochanges).

### High-Level APIs

#### message(seed (optional), account_index (optional), initial_address_index (optional), inputs (optional), input_range_begin (optional), input_range_end (optional), outputs (optional), dust_allowance_outputs (optional), index (optional), index_raw (optional), data (optional), data_str (optional), parents (optional)): Message

Build a message.

| Param                    | Type                                 | Default                | Description                                  |
| ------------------------ | ------------------------------------ | ---------------------- | -------------------------------------------- |
| [seed]                   | <code>str</code>                     | <code>undefined</code> | The hex-encoded seed of the account to spend |
| [account_index]          | <code>int</code>                     | <code>undefined</code> | The account index                            |
| [initial_address_index]  | <code>int</code>                     | <code>undefined</code> | The initial address index                    |
| [inputs]                 | <code>list[[Input](#input)]</code>   | <code>undefined</code> | Inputs                                       |
| [input_range_begin]      | <code>int</code>                     | <code>undefined</code> | The begin index of the input                 |
| [input_range_end]        | <code>int</code>                     | <code>undefined</code> | The end index of the input                   |
| [outputs]                | <code>list[[Output](#output)]</code> | <code>undefined</code> | Outputs                                      |
| [dust_allowance_outputs] | <code>list[[Output](#output)]</code> | <code>undefined</code> | Dust allowance output to the transaction     |
| [index]                  | <code>str</code>                     | <code>undefined</code> | The indexation string                        |
| [index_raw]              | <code>list[int]</code>               | <code>undefined</code> | The indexation byte array                    |
| [data]                   | <code>list[int]</code>               | <code>undefined</code> | The data in bytes                            |
| [data_str]               | <code>str</code>                     | <code>undefined</code> | The data string                              |
| [parents]                | <code>list[str]</code>               | <code>undefined</code> | The message ids of the parents               |

**Returns** the built [Message](#message).

#### get_message_metadata(message_id): MessageMetadataResponse

| Param        | Type             | Default                | Description    |
| ------------ | ---------------- | ---------------------- | -------------- |
| [message_id] | <code>str</code> | <code>undefined</code> | The message id |

**Returns** the [MessageMetadataResponse](#messagemetadataresponse).

#### get_message_data(message_id): Message

Gets the message data from the message id.

| Param        | Type             | Default                | Description    |
| ------------ | ---------------- | ---------------------- | -------------- |
| [message_id] | <code>str</code> | <code>undefined</code> | The message id |

**Returns** the [Message](#message).

#### get_message_raw(message_id): str

Gets the raw message string from the message id.

| Param        | Type             | Default                | Description    |
| ------------ | ---------------- | ---------------------- | -------------- |
| [message_id] | <code>str</code> | <code>undefined</code> | The message id |

**Returns** the raw message string.

#### get_message_children(message_id): list[str]

Gets the children of the given message.

| Param        | Type             | Default                | Description    |
| ------------ | ---------------- | ---------------------- | -------------- |
| [message_id] | <code>str</code> | <code>undefined</code> | The message id |

**Returns** the list of children strings.

#### get_message_index(index): list[str]

Gets the list of message indices from the message_id.

| Param   | Type             | Default                | Description               |
| ------- | ---------------- | ---------------------- | ------------------------- |
| [index] | <code>str</code> | <code>undefined</code> | The identifier of message |

**Returns** the list of message ids.

#### find_messages(indexation_keys (optional), message_ids (optional)): list[Message]

Finds all messages associated with the given indexation keys and message ids.

| Param             | Type                   | Default                | Description                             |
| ----------------- | ---------------------- | ---------------------- | --------------------------------------- |
| [indexation_keys] | <code>list[str]</code> | <code>undefined</code> | The list of indexations keys too search |
| [message_ids]     | <code>list[str]</code> | <code>undefined</code> | The list of message ids to search       |

**Returns** the list of the found messages.

#### get_unspent_address(seed, account_index (optional), initial_address_index(optional)): (str, int)

Gets a valid unspent address.

| Param                   | Type             | Default                | Description                    |
| ----------------------- | ---------------- | ---------------------- | ------------------------------ |
| [seed]                  | <code>str</code> | <code>undefined</code> | The hex-encoded seed to search |
| [account_index]         | <code>int</code> | <code>undefined</code> | The account index              |
| [initial_address_index] | <code>int</code> | <code>undefined</code> | The initial address index      |

**Returns** a tuple with type of `(str, int)` as the address and corresponding index in the account.

#### find_addresses(seed, account_index (optional), input_range_begin (optional), input_range_end (optional) get_all (optional)): list[(str, bool (optional))]

Finds addresses from the seed regardless of their validity.

| Param               | Type              | Default                | Description                    |
| ------------------- | ----------------- | ---------------------- | ------------------------------ |
| [seed]              | <code>str</code>  | <code>undefined</code> | The hex-encoded seed to search |
| [account_index]     | <code>int</code>  | <code>undefined</code> | The account index              |
| [input_range_begin] | <code>int</code>  | <code>undefined</code> | The begin of the address range |
| [input_range_end]   | <code>int</code>  | <code>undefined</code> | The end of the address range   |
| [get_all]           | <code>bool</code> | <code>undefined</code> | Get all addresses              |

**Returns** a list of tuples with type of `(str, int)` as the address and corresponding index in the account.

#### get_balance(seed, account_index (optional), initial_address_index(optional)): int

Get balance on a given seed and its wallet account index.

| Param                   | Type             | Default                | Description                    |
| ----------------------- | ---------------- | ---------------------- | ------------------------------ |
| [seed]                  | <code>str</code> | <code>undefined</code> | The hex-encoded seed to search |
| [account_index]         | <code>int</code> | <code>undefined</code> | The account index              |
| [initial_address_index] | <code>int</code> | <code>undefined</code> | The initial address index      |

**Returns** the amount of balance.

#### get_address_balances(addresses): list[AddressBalancePair]

Get the balance in iotas for the given addresses.

| Param       | Type                   | Default                | Description                     |
| ----------- | ---------------------- | ---------------------- | ------------------------------- |
| [addresses] | <code>list[str]</code> | <code>undefined</code> | The list of addresses to search |

**Returns** the list of [AddressBalancePair](#addressbalancepair).

#### retry(message_id): (str, Message)

Retries (promotes or reattaches) the message associated with the given id.

| Param        | Type             | Default                | Description    |
| ------------ | ---------------- | ---------------------- | -------------- |
| [message_id] | <code>str</code> | <code>undefined</code> | The message id |

**Returns** the message id and the retried [Message](#message).

#### reattach(message_id): (str, Message)

Reattaches the message associated with the given id.

| Param        | Type             | Default                | Description    |
| ------------ | ---------------- | ---------------------- | -------------- |
| [message_id] | <code>str</code> | <code>undefined</code> | The message id |

**Returns** the message id and the reattached [Message](#message).

#### promote(message_id): (str, Message)

Promotes the message associated with the given id.

| Param        | Type             | Default                | Description    |
| ------------ | ---------------- | ---------------------- | -------------- |
| [message_id] | <code>str</code> | <code>undefined</code> | The message id |

**Returns** the message id and the promoted [Message](#message).

### MQTT APIs

#### subscribe_topic(topic, callback): void

Subscribe a topic and assign the associated callback.

| Param      | Type                  | Default                | Description           |
| ---------- | --------------------- | ---------------------- | --------------------- |
| [topic]    | <code>str</code>      | <code>undefined</code> | The MQTT topic        |
| [callback] | <code>function</code> | <code>undefined</code> | The callback function |

#### subscribe_topics(topics, callback): void

Subscribe topics and assign the associated callbacks, respectively.

| Param      | Type                   | Default                | Description            |
| ---------- | ---------------------- | ---------------------- | ---------------------- |
| [topics]   | <code>list[str]</code> | <code>undefined</code> | The MQTT topics        |
| [callback] | <code>function</code>  | <code>undefined</code> | The callback functions |

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

#### BalanceForAddressResponse

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

#### UTXOInput

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
    'use_ws': bool
}
```

#### LedgerInclusionStateDto

A dict with the following key/value pairs.

```python
ledger_inclusion_state_dto = {
    'state': str
}
```

#### InfoResponse

A dict with the following key/value pairs.

```python
info_response = {
    'name': str,
    'version': str,
    'is_healthy': bool,
    'network_id': str,
    'bech32_hrp': str,
    'latest_milestone_index': int,
    'solid_milestone_index': int,
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