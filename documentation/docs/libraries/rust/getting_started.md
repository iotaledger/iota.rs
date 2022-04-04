---
description: Getting started with the official IOTA Client Library Rust library.
image: /img/logo/iota_mark_light.png
keywords:
- Rust
- install
- cargo
- system environment variables
---
# Getting Started with Rust

## Requirements

To use the library, we recommend you update Rust to latest stable version [`$ rustup update stable`](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date). Nightly should be fine but some changes might not be compatible.

`no_std` is not currently supported, but we are working on it in [bee](https://github.com/iotaledger/bee), and will provide it as feature once the new implementation is ready.

## Using the library

Using the library is easy, just add it as dependency in your `Cargo.toml`:

```bash
[dependencies]
iota-client = { git = "https://github.com/iotaledger/iota.rs", branch = "dev" }
# asynchronous runtime
tokio = { version = "1.12.0", features = ["full"] }
```

And then you can use the library in your code with `use iota_client;`.

### Initialisation

This example fetches node information

```rust
use iota_client::Client;

#[tokio::main]
async fn main() {
    let client = Client::builder() // Crate a client instance builder
        .with_node("http://localhost:14265")
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);
}
```
