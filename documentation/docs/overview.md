---
description: The wallet library is a stateful package with a standardized interface for developers to build applications involving IOTA value transactions.
image: /img/Banner/banner_client_lib_overview.png
keywords:
- layered overview
- high level
- low level
- stronghold
- value transactions
- explanation
---
# Overview

![IOTA Client Library Overview](/img/Banner/banner_client_lib_overview.png)

The iota.rs library is designed to simplify how you connect and interact
with [nodes](https://wiki.iota.org/chrysalis-docs/node_software) in the IOTA network. You can use it to generate
addresses, send messages, sign and send transactions, and more.

Beyond establishing the initial connection to a node, iota.rs has no state. Operations use only the data you pass during
a call and have no effect on your software beyond returning a value. You are in complete control of the data flow in
your application.

This stateless approach makes iota.rs easy to use and understand. But since you are in full control of data management,
you are also fully responsible for it, which could feel overwhelming if you handle complex or sensitive data. If you
plan on managing funds in your application, take a look at
our [wallet.rs library](https://wiki.iota.org/wallet.rs/welcome) instead. It allows you to manage your user's funds
safely, and it already includes our best security practices. It
uses [stronghold.rs](https://wiki.iota.org/stronghold.rs/welcome) to store sensitive data and iota.rs to communicate
with the IOTA network and, unlike iota.rs, it has a state.

## Supported Languages

We implemented the iota.rs library in [Rust](./getting_started/rust.md) and prepared bindings
for [JavaScript](./getting_started/nodejs.md), [Python](./getting_started/python.mdx)
, [Java](./getting_started/java/getting_started.mdx), and [Wasm](./getting_started/wasm.md). Every binding is adjusted for the
language's conventions and best practices. For example, Python developers avoid the Builder programming pattern, so our
Python binding uses named constructor arguments. However, we never change the meaning behind our API, which is equally
powerful no matter which language you choose.

## Your Application In the IOTA Network

Your application communicates with iota.rs either directly in Rust or through one of the language bindings. The iota.rs
library turns your requests into REST API calls and sends them to a node through the Internet. The node, in turn,
interacts with the rest of an IOTA network, which could be
the [main operational network (mainnet)](https://wiki.iota.org/introduction/reference/networks/mainnet) or
a [network for testing purposes (devnet)](https://wiki.iota.org/introduction/reference/networks/devnet).

Different nodes can run on different software, but they always expose the same interface to clients. For example, one
node could be a [Hornet](https://wiki.iota.org/hornet/welcome) node, and the other could be
a [Bee](https://wiki.iota.org/bee/welcome) node, and they both would appear the same for any client.

![A diagram that illustrates the text above. It has three layers: the application layer that includes iota.rs and its bindings, communication layer (the Internet network), and IOTA network layer with nodes that operate on one of the IOTA networks.](/img/overview/layered_overview.svg "An overview of IOTA layers.")

## API Design

The iota.rs library exposes operations of two types. Clients interact with nodes by calling their REST API, and the
first group of operations mirrors the available calls. When your program invokes such an operation, it directly
translates it into a REST call to a node. See
the [node's REST API reference](https://editor.swagger.io/?url=https://raw.githubusercontent.com/rufsam/protocol-rfcs/master/text/0026-rest-api/0026-rest-api.yaml)
for a complete list of available endpoints.

Operations from the first group tend to be too atomic and basic to use conveniently. The second group provides you with
higher-level helper functions. These functions represent an actual task and combine multiple basic operations
internally. For example, you can get your token balance by calling `getBalance`. It will first call `getAddresses`, then
call `getAddressBalances` for each address, and add the results together to return the total balance. See
the [full specification](./specs.mdx) for details.
