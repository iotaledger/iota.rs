# iota.rs

Official Rust library.  

> **🚧 Alpha Version 🚧**
>
> This repository now targets [Chrysalis part 2](https://roadmap.iota.org/chrysalis) It will have many breaking change along the way. Please use with caution! 🚧

The goal of this library is to have `one source code of truth`, which means there is one implementation in Rust and [bindings](#bindings) to other programming languages.

For value transfers we recommend to use [wallet.rs](https://github.com/iotaledger/wallet.rs).

This **alpha** version allows you to do the following:

* Create messages and transactions
* Sign transactions
* Generate addresses
* Interact with an IOTA node

Please report any issues in our [issue tracker](https://github.com/iotaledger/iota.rs/issues).

| Table of contents                                 |
| :------------------------------------------------ |
| [Prerequisites](#prerequisites)                   |
| [Using the library](#installing-the-library)      |
| [API reference](#api-reference)                   |
| [Examples](#examples)                             |
| [Bindings](#bindings)                             |
| [Supporting the project](#supporting-the-project) |
| [Joining the discussion](#joining-the-discussion) |
| [License](#license)                               |

## Requirements

To use the library, we recommend you update Rust to latest stable version [`$ rustup update stable`](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date). Nightly should be fine but some changes might not be compatible.

`no_std` is not currently supported, but we are working on it in [bee](https://github.com/iotaledger/bee), and will provide it as feature once the new implementation is ready.

## Using the library

Using the library is easy, just add it as dependancy in `Cargo.toml`:

```bash
[dependencies]
iota-core = { git = "https://github.com/iotaledger/iota.rs", branch = "dev" }
```

And then you can use the library in your code with `iota`.

When using the "MQTT" feature, connecting to a MQTT broker using raw ip doesn't work. This is a limitation of rustls.

## API reference

You can read the [API reference](https://docs.rs/iota-lib-rs/) here, or generate it yourself.

If you'd like to explore the implementation in more depth, the following command generates docs for the whole crate, including private modules:

```bash
cargo doc --document-private-items --no-deps --open
```

## Examples

You can see the examples in the [examples](examples/) directory and try them with:

```bash
cargo run --example balance
```

For the examples where a seed is required you have to rename `.env.example` to `.env`.

## Bindings

Bindings to other programming languages.

* [Node.js binding](bindings/nodejs/)
* [Python binding](bindings/python/)

## Supporting the project

## Joining the discussion

If you want to get involved in the community, need help with setting up, have any issues or just want to discuss IOTA with other people, feel free to join our [Discord](https://discord.iota.org/).

## License

The Apache 2.0 license can be found [here](LICENSE).
