# Examples

> Please note: In is not recommended to store passwords/seeds on host's environment variables or in the source code in a production setup! Please make sure you follow our [backup and security](https://chrysalis.docs.iota.org/guides/backup_security.html) recommendations for production use!

## Connecting to node(s)
All features of `iota.rs` library are accessible via an instance of `Client` class that provides high-level abstraction to all interactions with IOTA network (Tangle). This class has to be instantiated before starting any interactions with the library, or more precisely with [IOTA nodes](https://chrysalis.docs.iota.org/node-software/node-software.html) that power IOTA network.

You may be familiar with a fact that in case of IOTA 1.0 network one had to know an address of IOTA node to start participating to the network. It is no longer needed in IOTA 1.5 (Chrysalis) world. The library is designed to automatically choose a starting IOTA node based on the network type one would like to participate in: `testnet` or `mainnet`.

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
* `is_healthy`: indicates whether the given node is in sync with the network and so it is safe to use it. Even if a node is up and running it may not be fully prepared to process your API calls properly. The node should be "synced", meaning should be aware of all TXs in the Tangle. It is better to avoid not fully synced nodes. A node healthiness can be alternatively obtained also with a method `Client.get_health()`
* `bech32_hrp`: it indicates whether the given node is a part of testnet (`atoi`) or mainnet (`iota`). See more info regarding [IOTA address format](../../welcome.md#iota-15-address-anatomy)

_Please note, when using node load balancers then mentioned health check may be quite useless since follow-up API calls may be served by different node behind the load balancer that may have not been actually checked. One should be aware of this fact and trust the given load balancer participates only with nodes that are in healthy state. `iota.rs` library additionally supports a management of internal node pool and so load-balancer-like behavior can be mimicked using this feature locally._

Needless to say, the `Client` class constructor provides several parameters via which the process can be closely managed.
The most common ones:
* `network`: can be `Testnet` or `Mainnet`. It instructs the library whether to automatically select testnet nodes or mainnet nodes
* `node`: specify address of actual running IOTA node that should be used to communicate with (in format `https://node:port`), for ex: https://api.lb-0.testnet.chrysalis2.com:443
* `node_pool_urls`: library also supports a management of pool of nodes. You can provide a list of nodes and library manages access to them automatically (selecting them based on their sync status)
* `local_pow`: `local_pow==True` (by default) means a `Proof-of-work` is done locally and not remotely
* `node_sync_disabled`: `node_sync_disabled==False` (by default) means that library checks for sync status of node(s) periodically before its use. `node_sync_disabled==True` means library also uses nodes that _are not_ in sync with network. This parameter is usually useful if one would like to interact with local test node that is not fully synced. This parameter should not be used in production

If `node_pool_urls` is provided then the library periodically checks in some interval (argument `node_sync_interval`) whether node is in sync or not.

## Generating seed and addresses

Since the IOTA network is permission-less type of network, anybody is able to use it and interact with it. No central authority is required at any stage. So anybody is able to generate own `seed` and then deterministically generate respective private key/address.

> Please note, it is highly recommended to NOT use online seed generators at all. The seed is the only key to the given addresses. Anyone who owns the seed owns also all funds related to respective IOTA addresses (all of them).

> We strongly recommend to use official `wallet.rs` library together with `stronghold.rs` enclave for value-based transfers. This combination incorporates the best security practices while dealing with seeds and related addresses. See more information on [Chrysalis docs](https://chrysalis.docs.iota.org/libraries/wallet.html).

IOTA 1.5 (Chrysalis) uses Ed25519 signature scheme and address is usually represented by Bech32 (checksummed base32) format string of 64 characters.

A root of `Ed25519` signature scheme is basically a `32-byte (256-bit)` uniformly randomly generated seed based on which all private keys and corresponding addresses are generated. In the examples below, the seed is represented by a string of 64 characters using `[0-9a-f]` alphabet (32 bytes encoded in hexadecimal).

Seed can be for example generated using SHA256 algorithm on some random input generated by cryptographically secure pseudo-random generator, such as `os.urandom()`:
```python
import os
import hashlib

rnd_seed = hashlib.sha256(os.urandom(256)).hexdigest()
print(rnd_seed)
```

Seed examples (a single seed per line):
```plaintext
4892e2265c45734d07f220294b1697244a8ab5beb38ba9a7d57aeebf36b6e84a
37c4aab22a5883595dbc77907c1626c1be39d104df39c5d5708423c0286aea89
e94346bce41402155ef120e2525fad2d0bf30b10a89e4b93fd8471df1e6a0981
...
```

> In modern wallet implementations, such as our [wallet.rs library](https://chrysalis.docs.iota.org/libraries/wallet.html) and [firefly wallet](https://blog.iota.org/firefly-beta-release/), the seed is usually generated from a `seed mnemonic` (`seed phrase`), using [BIP39 standard](https://en.bitcoin.it/wiki/BIP_0039), to be better memorized/stored by humans. It is based on randomly generated list of english words and later used to generate the seed. Either way, the seed is a root for all generated private keys and addresses

### Address/key space
Before an actual address generation process, let's quickly focus on [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) standard that describes an approach to _Hierarchical Deterministic Wallets_. The standard was improved by [BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) lately.

These standards define a tree structure as a base for address and key space generation which is represented by a derivation path:
```plaintext
m / purpose / coin_type / account / change / address_index
```
* `m`: a master node (seed)
* `purpose`: constant which is {44}
* `coin_type`: a constant set for each crypto currency. IOTA = 4218
* `account`: account index. Zero-based increasing `int`. This level splits the address/key space into independent branches (ex. user identities) which each has own set of addresses/keys
* `change`: change index which is `{0, 1}`.<br />
There are two independent chain of addresses/keys. `0` is reserved for public addresses (for coin receival) and `1` is reserved for internal (also known as change) addresses to which transaction change is returned
* `address_index`: address index. Zero-based increasing `int` that indicates the address index

As outlined, there is a quite large address/key space that is secured by a single unique seed. And there are few additional interesting notes:
* Each level defines a completely different subtree (subspace) of addresses/keys and those are never mixed up
* The hierarchy is ready to "absorb" addresses/keys for many different coins at the same time (`coin_type`), and all those coins are secured by the same seed.<br />(So basically any BIP32/44-compliant wallet is potentially able to manage any BIP32/44-compliant coin(s))
* There may be also other `purposes` in the future however let's consider a single purpose as of now
* Using different `accounts` may be useful to split addresses/key into some independent spaces and it is up to developers to implement.<br />
_Please note, it may have a negative impact on a performance while [account discovery](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki#account-discovery) phase. So if you are after using many multiple accounts then you may be interested in our stateful library [wallet.rs](https://chrysalis.docs.iota.org/libraries/wallet.html) that incorporates all business logic needed to efficiently manage independent accounts. Also our [exchange guide](https://chrysalis.docs.iota.org/guides/exchange_guide.html) provides some useful tips how different accounts may be leveraged_

So in case of IOTA 1.5 (Chrysalis), the derivation path of address/key space is `seed/44/4218/{int}/{0,1}/{int}`. The Levels `purpose` and `coin_type` are given, the rest levels are up to developers to leverage.

## Generating addresses with library

IOTA addresses are generated via `Client.get_addresses()` function that returns a list of tuples with generated addresses. Considering the previous chapter about individual address/key spaces, it becomes quite clear what all used input function arguments are for:

```python
import iota_client
client = iota_client.Client()

address_changed_list = client.get_addresses(
    seed="b3d7092195c36d47133ff786d4b0a1ef2ee6a0052f6e87b6dc337935c70c531e",
    account_index=0,
    input_range_begin=0,
    input_range_end=10,
    get_all=True
)
print(address_changed_list)
```

Output example:
```json
[('atoi1qp9427varyc05py79ajku89xarfgkj74tpel5egr9y7xu3wpfc4lkpx0l86', False),
 ('atoi1qzfvkkp398v7hhvu89fu88hxctf7snwc9sf3a3nd7msfv77jk7qk2ah07s3', True),
 ('atoi1qq4t98j5y8wxkaujue99mjwqcp6jvvmsd5lv0755sz7dtjdz3p2lydv76sy', False),
 ('atoi1qrhzhjxc4z8vpwjt3hafs5xpdng5katqe890p0h95mc0l273j8yzxn7r4hc', True),
 ('atoi1qputu0yvfvxd7g39wf4rc67e0f0dyhl6enxu9jxnsrjqmemh067tw7qelyc', False),
 ('atoi1qptg5w2x47qwjf3gpqt3h7d2ey5x7xf8v7qtt29gkxt4mjfjfc28sutvd8a', True),
 ('atoi1qprvelq9paakh72fgm6j2kf8kexadw3t5xljer9dpsep5c7wx5mjwdxch6z', False),
 ('atoi1qrwk37tz47ddng9kpxfflkpz5tplcq7ll56v4acam04307xk70l7uf6wg8j', True),
 ('atoi1qper3zr5xe9x0wqs35ytwh622870g44frkyygdhs0ds8yejle3xujhq7dx3', False),
 ('atoi1qq6lkr9hucfylqjaqphu0stvk8pcmsx98r7ukuq40asszwmqytlnc058thk', True),
 ('atoi1qzpn7se3ryhscmqg404pycxzvfpt8v4xn8aul0tqdh00xsncgnxu7na7zjj', False),
 ('atoi1qz4qqakty9qytw8fk9shelt9lwlvv83s5ggt3wjag9fkgcc74z78w4l86y5', True),
 ('atoi1qp20uddchglqry0l5qnjg5aln8d5rk2v5l45hwrxv9z0daxs7u6xcsh4077', False),
 ('atoi1qrlqm2u5txxxnjx22fxq0jfjzk6l4nwnue6ht5pepk65m2f4xmxqynmxu2m', True),
 ('atoi1qqydc70mpjdvl8l2wyseaseqwzhmedzzxrn4l9g2c8wdcsmhldz0ulwjxpz', False),
 ('atoi1qrkjennxyl2xcqem6x69ya65sasma33z0ux872k846lqft0s3qf7k6lqpft', True),
 ('atoi1qr4yuekp30ff7mnnnjwy9tdhynxmlmkpuxf70qurtwudp2zpf3jeyw4uh37', False),
 ('atoi1qp6m5sz5ayjtccfxapdk5lp4qkheyfg0emzntmulyxzftps730vcul8dmqr', True),
 ('atoi1qzrwhkzhu67fqltfffwljejawdcghedukpgu9x6tzevwlnq89gmfjtayhgz', False),
 ('atoi1qpehxcp24z947dgupjqc9ktkn5ylmdxqqnx83m7xlajnf8005756u4n7z77', True)]
```
* Each tuple contains `address` and `bool` indicating the given address is a `change` address or not.<br />
`True` means the given address is a change address (internal). So basically we've got two independent sets of addresses (10 items per each)
* This behavior is controlled via `get_all` argument. `get_all=False` (default) means to generate only public addresses

IOTA address is represented by a checksumed base 32 string and you can see a detailed explanation on [Chrysalis docs](https://chrysalis.docs.iota.org/guides/index.html#iota-15-address-anatomy).
Just a recap:
* If an address starts with `atoi` then it means it is related to `testnet`. `iota` stands for mainnet
* Number `1` at 5<sup>th</sup> position is just a separator
* The last 6 characters are reserved for a checksum

To quickly validate any address there is a convenience function `Client.is_address_valid()` that returns `bool` indication. So definitely performing a sanity check of an address before its use is an advisable practice.

