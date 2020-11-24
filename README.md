# iota.rs

🚧 This repository is currently moving to Chrysalis part 2! It will have many breaking change along the way, and binding crates are not working at the moment. Please use with caution! 🚧

This is the **alpha** version of official Rust library, which allows you to do the following:
* Create transactions
* Sign transactions
* Generate addresses
* Interact with an IOTA node

This client library is still in the alpha stage but it should cover most usages. The main crate is under `iota-core` with library named as `iota` which re-exports fundamental crates from `bee` and also provide client features and utilities that users need. API calls like `post_message` and `get_outputs` are supported. But many modules are raw exported, so users might expect the interface is not that ergonomic yet. There may also be some performance and stability issues. Please report any issues in our [issue tracker](https://github.com/iotaledger/iota.rs/issues).

|Table of contents|
|:----|
| [Prerequisites](#prerequisites)|
| [Using the library](#installing-the-library)|
| [API reference](#api-reference)
| [Examples](#examples)|
| [Supporting the project](#supporting-the-project)|
| [Joining the discussion](#joining-the-discussion)|
| [License](#license)|

## Prerequisites

To use the library, we recommend update your Rust to latest stable version [`rustup update stable`](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date). Nightly should be fine but you are expected some changes might not be compatable.

`no_std` is not supported currently, but we are working on it in [bee](https://github.com/iotaledger/bee), and will provide it as feature once new library implementation is ready.

### macOS

To compile this crate on macOS, a `cmake` installation is required and the `OPENSSL_ROOT_DIR` environment variable must be set.

```
$ export OPENSSL_ROOT_DIR=/usr/local/opt/openssl
```

## Using the library

Using the library is fairly easy, just add it as dependancy in `Cargo.toml`:

```
[dependencies]
iota-core = { git = "https://github.com/iotaledger/iota.rs", branch = "dev" }
```

And then you can use the library in your code with `iota`

## API reference

You can read the [API reference](https://docs.rs/iota-core) here, or generate them on your own.

If you'd like to explore the implementation in more depth, the following command generates docs for the whole crate, including private modules:

```
cargo doc --document-private-items --no-deps --open
```

## Examples

You can see the examples in [examples](examples/) directory and try them with:

```
cargo run --example balance
```

## Supporting the project

## Joining the discussion

If you want to get involved in the community, need help with setting up, have any issues related with the library or just want to discuss IOTA, Distributed Ledger Technology (DLT) and IoT with other people, feel free to join our [Discord](https://discord.iota.org/).

## License

The Apache 2.0 license can be found [here](LICENSE).
