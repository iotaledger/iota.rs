---
description: IOTA uses the `Ed25519` signature scheme and addresses are usually represented by Bech32 (checksummed base32) format string of 64 characters. 
image: /img/libraries/messages_in_tangle.svg 
keywords:
- explanation
- seeds
- ED2519
- signature scheme
- random seed
- seed generator
- private key
---
# Seeds

Since the IOTA network is permission-less, anybody is able to use it and interact with it. No central authority is
required at any stage. So anybody is able to generate their own `seed` and then deterministically generate the
respective private keys/addresses.

IOTA uses the `Ed25519` signature scheme and addresses are usually represented by Bech32 (checksummed base32) format
string of 64 characters.

A root of the `Ed25519` signature scheme is basically a `32-byte (256-bit)` uniformly randomly generated seed on which
all private keys and corresponding addresses are generated. In the examples below, the seed is represented by a string
of 64 characters using the `[0-9a-f]` alphabet (32 bytes encoded in hexadecimal).

:::info
In modern wallet implementations such as the [wallet.rs library](https://wiki.iota.org/wallet.rs/welcome) and the
[firefly wallet](https://blog.iota.org/firefly-beta-release/), the seed is usually generated from a `seed mnemonic`
(`seed phrase`), using [BIP39 standard](https://en.bitcoin.it/wiki/BIP_0039), to be better memorized/stored by humans.
It is based on a randomly generated list of english words and later used to generate the seed. The seed is a root for
all generated private keys and addresses.
:::