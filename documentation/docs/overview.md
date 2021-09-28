---
description: The wallet library is a stateful package with a standardized interface for developers to build applications involving IOTA value transactions.
image: /img/overview/layered_overview.svg
keywords:
- layered overview
- high level
- low level
- stronghold
- value transactions
---
# Overview

To communicate with the IOTA network, you have to connect and interact with a [node](https://wiki.iota.org/chrysalis-docs/node_software), the task that the iota.rs library is designed to simplify. It will generate addresses for you, send messages, sign and send transactions, and more.

Beyond establishing the initial connection to a node, iota.rs has no state. Operations use only the data that you pass during a call and have no effect on your software beyond returning a value. You are in full control of the data flow in your application.

This stateless approach makes iota.rs easier for you to use and understand. But since you are in full control of the data management, you also fully responsible for it, which could feel overwhelming if you have to handle complex or sensitive data. If you plan on managing funds in your application, take a look at our [wallet.rs](https://wiki.iota.org/wallet.rs/welcome) library instead. It allows you to safely manage your user's funds, and it already includes our best security practices. It uses [stronghold.rs](https://wiki.iota.org/stronghold.rs/welcome) to store sensitive data and iota.rs to communicate with the IOTA network. Unlike iota.rs, it has a state.

## Supported Languages

We have implemented the iota.rs library in Rust and prepared bindings for JavaScript, Python, Java, and Wasm, each carefully tailored as we adjust every binding for the language's conventions and best practices. For example, Python developers avoid the Builder programming pattern, so our Python binding uses named constructor arguments instead. We never change the meaning behind our API, however. The API stays equally powerful no matter the language you choose. 

## Your Application In the IOTA Network

Your application communicates with iota.rs either directly in Rust or through one of the language bindings. The iota.rs library turns your requests into REST API calls and sends them to a node through the Internet. The node, in turn, interacts with the rest of an IOTA network, which could be the main operational network (mainnet) or a network for testing purposes (devnet).

Different nodes could run on a different software, but they always expose the same interface to clients. For example, one node could be a [Hornet](https://hornet.docs.iota.org/) node and the other could be a [Bee](https://wiki.iota.org/bee/welcome) node, and they both would appear the same for any client.

![A diagram that illustrates the text above. It has three layers: the application layer that includes iota.rs and its bindings, communication layer (the Internet network), and IOTA network layer with nodes that operate on one of the IOTA networks.](/img/overview/layered_overview.svg "An overview of IOTA layers.")

## API Design

The iota.rs library exposes operations of two types. Clients interact with nodes by calling their REST API, and the first group of operations mirrors the available calls. When your program invokes such an operation, it directly translates it into a REST call to a node. For a full list, see the [node's REST API reference](https://editor.swagger.io/?url=https://raw.githubusercontent.com/rufsam/protocol-rfcs/master/text/0026-rest-api/rest-api.yaml). 

Operations from the first group tend to be too atomic and basic to use them in a convenient way. The second group provides you with higher-level helper functions. These functions represent an actual task and combine multiple basic operations internally. For example, you can get your token balance by calling `getBalance`. It first calls `getAddresses`, then it calls `getAddressBalances` for each address, and adds the results together to return the total balance. See the [full specification](./specs) for details.
