# Overview

To communicate with the IOTA network, you have to connect and interact with a [node](https://chrysalis.docs.iota.org/node-software/node-software.html). This library, the IOTA Client, wraps routine tasks and minute details into its tidy API. It will generate addresses for you, send messages, sign and send transactions, and more.

Beyond establishing the initial connection to a node, Client has no state. Operations use only the data that you pass during the call and have no effect on your code or the client instance beyond returning a value. You are in full control of the data flow in your application.

This approach makes Client easier for you to use and understand, and for us to develop and maintain. But since you are in full control of the data management, you also fully responsible for it. It could feel tedious or overwhelming if you have to manage complex or sensitive data.

If you plan on managing funds in your application, take a look at our [IOTA wallet](https://wallet-lib.docs.iota.org/) library instead. It already includes our best security practices and can build and store wallets and manage token transactions. Wallet uses Client to communicate with the IOTA network, and [Stronghold](https://stronghold.docs.iota.org/) to store sensitive data. Unlike Client, it has a state.

## Supported Languages

We have implemented the IOTA client library in Rust and prepared bindings for JavaScript and Python. The API stays the same no matter the language you choose, as we aim to make our libraries equally simple to use across all supported languages. You can read more on reasoning behind this design in this [blog post](https://blog.iota.org/the-new-iota-client-libraries-harder-better-faster-stronger/).

## Your Application In the IOTA Network

Your application communicates with the IOTA client library either directly or through a node.js or python binding. Client turns your requests into REST API calls and sends them to a node through the Internet. The node, in turn, interacts with the rest of an IOTA network, which could be the main operational network (mainnet) or a network for testing purposes (devnet).

Different nodes could run on a different software, but they always expose the same interface to clients. For example, one node could be a [Hornet](https://hornet.docs.iota.org/) node and the other could be a [Bee](https://bee.docs.iota.org/) node, and they both would appear the same for any client.

![An illustration for the text above.](./layered_overview.svg "An overview of IOTA layers.")

## API Design

The IOTA client library exposes operations of two types. Clients interact with nodes by calling their REST API, and the first group of operations mirrors the available calls. As you invoke an operation, it directly translates into a REST call to a node. For a full list, see the [node's REST API reference](https://editor.swagger.io/?url=https://raw.githubusercontent.com/rufsam/protocol-rfcs/master/text/0026-rest-api/rest-api.yaml). 

Operations from the first group tend to be too atomic and basic to use them in a convenient way. The second group battles that by providing you with helper functions. These functions represent an actual task and combine multiple basic operations internally. For example, you can get your token balance by calling `getBalance(seed)`. It first calls `getAddresses(seed)`, then it calls `getAddressBalances` over associated addresses, and sums the results to return the total balance. See the [full specification](../specs) for details.