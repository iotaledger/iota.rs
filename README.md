# iota.rs

[![Build status](https://badge.buildkite.com/a4200bfaad6aa8ce4da6550c82dce3010e998437ecd9de93d8.svg)](https://buildkite.com/iota-foundation/iota-lib-rs)
[![Version](https://img.shields.io/crates/v/iota-lib-rs.svg)](https://crates.io/crates/iota-lib-rs)
[![Documentation](https://docs.rs/iota-lib-rs/badge.svg)](https://docs.rs/iota-lib-rs/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/iotaledger/iota-lib-rs/blob/master/LICENSE)

This is the **alpha** version of official Rust library, which allows you to do the following:
* Create transactions
* Sign transactions
* Generate addresses
* Interact with an IRI node

TODO
This client library is still in the alpha stage but it should cover most usages. The main crate is under `iota-core` with library named as `iota` which re-exports fundamental crates from `bee` and also provide client features and utilities that users need. API calls like `send_transfers` and `traverse_bundle` are supported. But many modules are raw exported, so users might expect the interface is not that ergonomic yet. There may also be some performance and stability issues. Please report any issues in our [issue tracker](https://github.com/iotaledger/iota.rs/issues).

|Table of contents|
|:----|
| [Prerequisites](#prerequisites)|
| [Using the library](#installing-the-library)|
| [Getting started](#getting-started)|
| [API reference](#api-reference)
| [Examples](#examples)|
| [Supporting the project](#supporting-the-project)|
| [Joining the discussion](#joining-the-discussion)|
| [License](#license)|

## Prerequisites

To use the library, we recommend update your Rust to latest stable version [`rustup update stable`](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date). Nightly should be fine but you are expected some changes might not be compatable.

`no_std` is not supported currently, but we are working on it in [bee](https://github.com/iotaledger/bee), and will provide it as feature once new library implementation is ready.

## Using the library

Using the library is fairly easy, just add it as dependancy in `Cargo.toml`:

```
[dependencies]
iota-core = "0.1.0-alpha"
```

And the import the lbrary in your code:

```rust
use iota;
```

## Getting started

After you've [installed the library](#installing-the-library),  you can connect to an IRI node to send transactions to it and interact with the ledger.

To connect to a local IOTA node, we provide a module `Client` :

```rust
use iota::Client;

fn main() {
  let mut iota = iota::Client::new("https://nodes.comnet.thetangle.org");
  println!("{:#?}", iota.get_node_info().unwrap());
}
```


## API reference

You can read the [API reference](https://docs.rs/iota-core) here, or generate them on your own.

If you'd like to explore the implementation in more depth, the following command generates docs for the whole crate, including private modules:

```
cargo doc --document-private-items --no-deps --open
```

## Examples

You can see the examples in [examples](examples/) directory and try them with:

```
cargo run --example send-transfers
```

## Supporting the project

## Joining the discussion

If you want to get involved in the community, need help with setting up, have any issues related with the library or just want to discuss IOTA, Distributed Ledger Technology (DLT) and IoT with other people, feel free to join our [Discord](https://discord.iota.org/).

## License

The MIT license can be found [here](LICENSE).
