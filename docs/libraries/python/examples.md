# Examples

> Please note: In is not recommended to store passwords/seeds on host's environment variables or in the source code in a production setup! Please make sure you follow our [backup and security](https://chrysalis.docs.iota.org/guides/backup_security.html) recommendations for production use!

## Client instance
All features of `iota.rs` library are accessible via an instance of `Client` class that provides high-level abstraction to all interactions with IOTA network (Tangle). This object has to be instantiated before starting any interactions with the library, or more precisely with [IOTA nodes](https://chrysalis.docs.iota.org/node-software/node-software.html) that power IOTA network.

You may be familiar with a fact that in case of IOTA 1.0 network one had to know an address of IOTA node to start participating on the network. It is no longer needed in IOTA 1.5 (Chrysalis) world. The library is designed to automatically choose a starting IOTA node based on the network one would like to participate in: `testnet` or `mainnet`.

So very simplistic example how to connect to [IOTA testnet](https://chrysalis.docs.iota.org/testnet.html) is the following one:

```python
import iota_client

# client will connect to testnet by default
client = iota_client.Client()
print(client.get_info())
```

Output example of `get_info()` function of the `Client` instance:
```json
{
    'name': 'HORNET',
    'version': '0.6.0-alpha',
    'is_healthy': True,
    'network_id': 'testnet6',
    'bech32_hrp': 'atoi',
    'latest_milestone_index': 192448,
    'confirmed_milestone_index': 192448,
    'pruning_index': 174931,
    'features': ['PoW'],
    'min_pow_score': 4000.0
}
```
The most important properties:
* `is_healthy`: indicates whether the node is in sync with the network and so it is safe to use it
* `bech32_hrp`: it indicates whether the given node is part of testnet (`atoi`) or mainnet (`iota`). See more info regarding [IOTA address format](../../welcome.md#iota-15-address-anatomy)

Needless to say, `Client` class constructor provides several parameters via which one can control the process.

The most common ones:
* `network`: can be `Testnet` or `Mainnet`. It instructs the library whether to automatically select testnet nodes or mainnet nodes
* `node`: specify address of actual running IOTA node that should be used to communicate with (format https://node:port), for ex: https://api.lb-0.testnet.chrysalis2.com:443
* `node_pool_urls`: library also supports a management of pool of nodes and so you can provide a list of nodes and library manages access to them automatically (selecting them based on their sync status)
* `local_pow`: `local_pow==True` (by default) means a `Proof-of-work` is done locally and not remotely
* `node_sync_disabled`: `node_sync_disabled==False` (by default) means that library checks for sync status of node(s) before use. `node_sync_disabled==True` means library also uses nodes that are not in sync with network. This parameter is usually used if one would like to interact with local test node that is not synced

If `node_pool_urls` is provided then the library periodically checks in some interval (argument `node_sync_interval`) whether node is in sync or not.

