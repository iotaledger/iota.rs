---
sidebar_label: utils
title: iota_client.utils
---

## Utils Objects

```python
class Utils(BaseAPI)
```

### bech32\_to\_hex

```python
def bech32_to_hex(bech32)
```

Transforms bech32 to hex.

### hex\_to\_bech32

```python
def hex_to_bech32(hex, bech32_hrp)
```

Transforms a hex encoded address to a bech32 encoded address.

### alias\_id\_to\_bech32

```python
def alias_id_to_bech32(alias_id, bech32_hrp)
```

Transforms an alias id to a bech32 encoded address.

### nft\_id\_to\_bech32

```python
def nft_id_to_bech32(nft_id, bech32_hrp)
```

Transforms an nft id to a bech32 encoded address.

### hex\_public\_key\_to\_bech32\_address

```python
def hex_public_key_to_bech32_address(hex, bech32_hrp=None)
```

Transforms a hex encoded public key to a bech32 encoded address.

### parse\_bech32\_address

```python
def parse_bech32_address(address)
```

Returns a valid Address parsed from a String.

### is\_address\_valid

```python
def is_address_valid(address)
```

Checks if a String is a valid bech32 encoded address.

### generate\_mnemonic

```python
def generate_mnemonic()
```

Generates a new mnemonic.

### mnemonic\_to\_hex\_seed

```python
def mnemonic_to_hex_seed(mnemonic)
```

Returns a hex encoded seed for a mnemonic.

### compute\_alias\_id

```python
def compute_alias_id(output_id)
```

Computes the alias id for the given alias output id.

### compute\_nft\_id

```python
def compute_nft_id(output_id)
```

Computes the NFT id for the given NFT output id.

### compute\_foundry\_id

```python
def compute_foundry_id(alias_address, serial_number, token_scheme_kind)
```

Computes the foundry id.

### block\_id

```python
def block_id(block)
```

Returns a block ID (Blake2b256 hash of block bytes) from a block.

### hash\_transaction\_essence

```python
def hash_transaction_essence(essence)
```

Compute the hash of a transaction essence.

