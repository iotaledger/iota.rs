---
sidebar_label: secret_manager
title: iota_client.secret_manager
---

## LedgerNanoSecretManager Objects

```python
class LedgerNanoSecretManager(dict)
```

Secret manager that uses a Ledger Nano hardware wallet or Speculos simulator.

### \_\_init\_\_

```python
def __init__(is_simulator)
```

Initialize a ledger nano secret manager.

## MnemonicSecretManager Objects

```python
class MnemonicSecretManager(dict)
```

Secret manager that uses a mnemonic in plain memory. It&#x27;s not recommended for production use. Use LedgerNano or Stronghold instead.

### \_\_init\_\_

```python
def __init__(mnemonic)
```

Initialize a mnemonic secret manager.

## SeedSecretManager Objects

```python
class SeedSecretManager(dict)
```

### \_\_init\_\_

```python
def __init__(seed)
```

Initialize a seed secret manager.

## StrongholdSecretManager Objects

```python
class StrongholdSecretManager(dict)
```

Secret manager that uses Stronghold.

### \_\_init\_\_

```python
def __init__(snapshot_path, password)
```

Initialize a stronghold secret manager.

