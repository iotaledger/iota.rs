---
description: There are Node.js, Python and Java bindings for the iota.rs client Rust library.
image: /img/logo/iota_mark_light.png
keywords:
- Rust
- Node.js
- Python 
- Java
- bindings
---
# IOTA Client libraries

There are currently available the following official bindings to `iota.rs`:

- [Rust](rust/getting_started.md) 
- [Node.js](nodejs/getting_started.md) 
- [Python](python/getting_started.md) 
- [Java](java/getting_started.md) 

## Getting Started
It is a recommended approach to start your interactions with IOTA on a `testnet` network. API load balancer: https://api.lb-0.h.chrysalis-devnet.iota.cafe:443

Network explorer is available at [IOTA Tangle Explorer](https://explorer.iota.org/devnet).

In order to properly test value-based transactions on testnet network, you are going to need some tokens! You can get some testnet tokens using the [faucet](https://faucet.chrysalis-devnet.iota.cafe/). However, we strongly recommend to leverage official [wallet.rs](https://wallet-lib.docs.iota.org/) library in case of value-based IOTA transfers.