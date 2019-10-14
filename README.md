# iota.rs

[![Build status](https://badge.buildkite.com/a4200bfaad6aa8ce4da6550c82dce3010e998437ecd9de93d8.svg)](https://buildkite.com/iota-foundation/iota-lib-rs)
[![Version](https://img.shields.io/crates/v/iota-lib-rs.svg)](https://crates.io/crates/iota-lib-rs)
[![Documentation](https://docs.rs/iota-lib-rs/badge.svg)](https://docs.rs/iota-lib-rs/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/iotaledger/iota-lib-rs/blob/master/LICENSE)

This is the **WIP** Rust client library, which allows you to do the following:
* Create transactions
* Sign transactions
* Generate addresses
* Interact with an IRI node

This is client library is still in the beta stage, so there may be performance and stability issues. As IOTA Foundation currently working on `bee`, we also decided to re-implement common libraries for security. This library is going to be feature freeze untill fundamental crates are done.
Please report any issues in our [issue tracker](https://github.com/iotaledger/iota.rs/issues).

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
iota-lib-rs = "0.4"
```

## Getting started

After you've [installed the library](#installing-the-library),  you can connect to an IRI node to send transactions to it and interact with the ledger.

To connect to a local IRI node, we provide a module `Client` :

```rust
use iota_lib_rs::prelude::*;

let mut iota = iota_client::Client::new("https://localhost");

println!("{:#?}", iota.get_node_info().unwrap);
```


## API reference

For details on all available API methods, see the [documentation](https://docs.rs/iota-lib-rs).

## Examples

```rust
use iota_client::options::SendTransferOptions;
use iota_lib_rs::prelude::*;
use iota_model::Transfer;
use iota_conversion::trytes_converter;

fn main() {
    let trytes =
        "HELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDD";
    let message = trytes_converter::to_trytes("Hello World").unwrap();
    let transfer = Transfer {
        address: trytes.to_string(),
        // Don't need to specify the field 
        // because the field and variable
        // have the same name
        message,
        // Populate the rest of the fields with default values
        ..Transfer::default()
    };
    let mut api = iota_client::Client::new("https://node01.iotatoken.nl");
    let tx = api
        .send_transfers(
            transfer,
            &trytes,
            SendTransferOptions {
                local_pow: true,
                threads: 2,
                ..SendTransferOptions::default()
            },
        )
        .unwrap();
    println!("{:?}", tx);
}
```

## Supporting the project

## Joining the discussion

If you want to get involved in the community, need help with getting setup, have any issues related with the library or just want to discuss IOTA, Distributed Ledger Technology (DLT) and IoT with other people, feel free to join our [Discord](https://discord.iota.org/).

## License

The MIT license can be found [here](LICENSE).
