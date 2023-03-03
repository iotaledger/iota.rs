---
sidebar_label: client
title: iota_client.client
---

## IotaClient Objects

```python
class IotaClient(NodeCoreAPI, NodeIndexerAPI, HighLevelAPI, Utils)
```

### \_\_init\_\_

```python
def __init__(client_config=None)
```

Initialize the IOTA Client.

### build\_alias\_output

```python
def build_alias_output(alias_id,
                       unlock_conditions,
                       amount=None,
                       native_tokens=None,
                       state_index=None,
                       state_metadata=None,
                       foundry_counter=None,
                       features=None,
                       immutable_features=None)
```

Build an AliasOutput.

### build\_basic\_output

```python
def build_basic_output(unlock_conditions,
                       amount=None,
                       native_tokens=None,
                       features=None)
```

Build a BasicOutput.

### build\_foundry\_output

```python
def build_foundry_output(serial_number,
                         token_scheme,
                         unlock_conditions,
                         amount=None,
                         native_tokens=None,
                         features=None,
                         immutable_features=None)
```

Build a FoundryOutput.

### build\_nft\_output

```python
def build_nft_output(nft_id,
                     unlock_conditions,
                     amount=None,
                     native_tokens=None,
                     features=None,
                     immutable_features=None)
```

Build an NftOutput.

### generate\_addresses

```python
def generate_addresses(secret_manager, options)
```

Generate addresses.

### build\_and\_post\_block

```python
def build_and_post_block(secret_manager=None, options=None)
```

Build and post a block.

### get\_node

```python
def get_node()
```

Get a node candidate from the healthy node pool.

### get\_network\_info

```python
def get_network_info()
```

Gets the network related information such as network_id and min_pow_score.

### get\_network\_id

```python
def get_network_id()
```

Gets the network id of the node we&#x27;re connecting to.

### get\_bech32\_hrp

```python
def get_bech32_hrp()
```

Returns the bech32_hrp.

### get\_min\_pow\_score

```python
def get_min_pow_score()
```

Returns the min pow score.

### get\_tips\_interval

```python
def get_tips_interval()
```

Returns the tips interval.

### get\_local\_pow

```python
def get_local_pow()
```

Returns if local pow should be used or not.

### get\_fall\_back\_to\_local\_pow

```python
def get_fall_back_to_local_pow()
```

Get fallback to local proof of work timeout.

### unhealthy\_nodes

```python
def unhealthy_nodes()
```

Returns the unhealthy nodes.

### get\_ledger\_nano\_status

```python
def get_ledger_nano_status(is_simulator)
```

Returns the Ledger Status.

### prepare\_transaction

```python
def prepare_transaction(secret_manager=None, options=None)
```

Prepare a transaction for signing.

### sign\_transaction

```python
def sign_transaction(secret_manager, prepared_transaction_data)
```

Sign a transaction.

### store\_mnemonic

```python
def store_mnemonic(secret_manager, mnemonic)
```

Store a mnemonic in the Stronghold vault.

### submit\_payload

```python
def submit_payload(payload_dto)
```

Submit a payload in a block.

