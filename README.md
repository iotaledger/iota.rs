# iota.rs

Official Rust library.

The goal of this library is to have `one source code of truth`, which means there is one implementation in Rust and [bindings](#bindings) to other programming languages.

For value transfers we recommend to use [wallet.rs](https://github.com/iotaledger/wallet.rs).

This library allows you to do the following:

* Create messages with indexation and transaction payloads
* Get messages and outputs
* Sign transactions
* Generate addresses
* Interact with an IOTA node

## Requirements

To use the library, we recommend you update Rust to latest stable version [`$ rustup update stable`](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date). Nightly should be fine but some changes might not be compatible.

`no_std` is not currently supported, but we are working on it in [bee](https://github.com/iotaledger/bee), and will provide it as feature once the new implementation is ready.

## Using the library

Using the library is easy, just add it as dependency in `Cargo.toml`:

```bash
[dependencies]
iota-client = "1.0"
```

or for the latest changes

```bash
[dependencies]
iota-client = { git = "https://github.com/iotaledger/iota.rs", branch = "dev" }
```

And then you can use the library in your code with `use iota_client;`.

When using the "MQTT" feature, connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.

## API reference

You can read the [API reference](https://docs.rs/iota-client) here, or generate it yourself.

If you'd like to explore the implementation in more depth, the following command generates docs for the whole crate, including private modules:

```bash
cargo doc --document-private-items --no-deps --open
```

## Examples

You can see the examples in the [examples](examples/) directory and try them like:

```bash
cargo run --example 01_get_info
```

For the examples where a seed is required you have to rename `.env.example` to `.env`.

## Bindings

Bindings to other programming languages.

* [Node.js binding](bindings/nodejs/)
* [Python binding](bindings/python/)
* [Java binding](bindings/java/)

## Joining the discussion

If you want to get involved in the community, need help with setting up, have any issues or just want to discuss IOTA with other people, feel free to join our [Discord](https://discord.iota.org/) in the #clients-dev and #clients-discussion channels.

## License

The Apache 2.0 license can be found [here](LICENSE).
