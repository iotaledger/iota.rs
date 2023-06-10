---
description: The BIP32 and BIP44 standards define a tree structure as a base for address and key space generation which is represented by a derivation path. 
image: /img/libraries/address_generation.svg 
keywords:
- explanation
- hierarchical deterministic wallet
- derivation path
- set of addresses
- account discovery
- accounts

---

# Address/Key Space

## BIP32 - Tree Structure

The [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) standard describes an approach to
_Hierarchical Deterministic Wallets_. The standard was improved
by [BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki).

This standard defines a tree structure as a base for address and key space generation which is represented by a
`derivation path`:

```plaintext
m / purpose / coin_type / account / change / address_index
```

* `m`: A master node (seed).
* `purpose`: A constant which is {44}.
* `coin_type`: A constant set for each crypto currency. IOTA = 4218, for instance.
* `account`: Account index. Zero-based increasing `int`. This level splits the address/key space into independent
  branches (ex. user identities) which each has own set of addresses/keys.
* `change`: Change index which is `{0, 1}`, also known as `wallet chain`.

  There are two independent chains of addresses or keys. `0` is reserved for public addresses (to receive coins) and `1` is
  reserved for internal addresses (also known as change) to which transaction change is returned. IOTA allows address
  reuse, and so it is, technically speaking, valid to return transaction change to the same originating address. It is
  up to developers whether to leverage it or not. The `iota.rs` library and its sibling `wallet.rs` help with either
  scenario.
  
* `address_index`: Address index. Zero-based increasing `int` that indicates an address index.

As outlined, there is quite a large address/key space that is secured by a single unique seed.

And there are few additional interesting notes:

* Each level defines a completely different subtree (subspace) of addresses/keys, and those are never mixed up.
* The hierarchy is ready to "absorb" addresses/keys for many coins at the same time (`coin_type`), and all those coins
  are secured by the same seed. This means any BIP32/44-compliant wallet is potentially able to manage any \
  BIP32/44-compliant coin(s).
* There may be also other `purposes` in the future. However, consider a single purpose for now. The constant `44` stands
  for BIP44.
* The standard was agreed upon by different crypto communities, although not all `derivation path` components are always in active use. For example, `account` is not always actively leveraged across the crypto space (if this is the case then `account=0` is usually used).
* Using different `accounts` may be useful to split addresses/keys into some independent spaces, and it is up to
  developers to implement. _Using different `accounts` may have a negative impact on a performance while you are on the
[account discovery](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki#account-discovery) phase. If you are
planning on using many multiple accounts then you may be interested in our stateful library
[wallet.rs](https://wiki.iota.org/wallet.rs/welcome) that incorporates all business logic needed to efficiently manage
independent accounts.
Our [exchange guide](https://wiki.iota.org/introduction/how_tos/exchange)
provides some useful tips on how different accounts may be leveraged.

![address_generation](/img/libraries/address_generation.svg)

In case of IOTA, the derivation path of address/key space is `[seed]/44/4218/{int}/{0,1}/{int}`. The levels `purpose`
and `coin_type` are given, the rest levels are up to developers to integrate.

## Related Examples

* [Generate Addresses](./../examples/generate_addresses.mdx)
