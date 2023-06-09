---
description: Getting started with the official IOTA Client Library Rust library.
image: /img/logo/libraries.png
keywords:
- Rust
- install
- cargo
- system environment variables
- how to
---
# Getting Started With Rust

## Requirements

To use the library, you should update [Rust to the latest stable version](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date).
You can update your Rust installation by running the following command:

```bash
rustup update stable
```

The nightly version should also be fine, but some changes might not be compatible.

`no_std` is not currently supported.  We are working on it in [Bee](https://github.com/iotaledger/bee), and will provide 
it as feature once the new implementation is ready.

## Using the Library

To use the iota.rs library, you will simply need to add it as dependency in your `Cargo.toml`:

```bash
[dependencies]
iota-client = { git = "https://github.com/iotaledger/iota.rs", branch = "production" }
# asynchronous runtime
tokio = { version = "1.12.0", features = ["full"] }
```

After you have added it, you can use the library in your code with `use iota_client;`.

### Initialisation

You can use the following example to initialize the library and fetch node information.

```rust
use iota_client::Client;

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Create a client instance builder
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);
}
```
