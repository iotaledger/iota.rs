# Getting Started with Rust

## Requirements

To use the library, we recommend you update Rust to latest stable version [`$ rustup update stable`](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date). Nightly should be fine but some changes might not be compatible.

`no_std` is not currently supported, but we are working on it in [bee](https://github.com/iotaledger/bee), and will provide it as feature once the new implementation is ready.

## Using the library

Using the library is easy, just add it as dependency in your `Cargo.toml`:

```bash
[dependencies]
iota-core = { git = "https://github.com/iotaledger/iota.rs", branch = "dev" }
```

And then you can use the library in your code with `use iota;`.

### Initialisation

This example fetches node information

```rust
use iota::Client;

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.lb-0.testnet.chrysalis2.com")
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);
}
```