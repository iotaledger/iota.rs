# Getting Started with Rust

## Requirements

To use the library, we recommend you update Rust to latest stable version [`$ rustup update stable`](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date). Nightly should be fine but some changes might not be compatible.

`no_std` is not currently supported, but we are working on it in [bee](https://github.com/iotaledger/bee), and will provide it as feature once the new implementation is ready.

### Dependencies

`cmake` and `openssl` are required. In order to run the build process succesfully using Cargo you might need install additional build tools on your system. 

### Windows

`cmake` can be downloaded on the [official website](https://cmake.org/download/) and `openssl` can be installed with [vcpkg](https://github.com/microsoft/vcpkg) or [chocolatey](https://chocolatey.org/).

* Installing `openssl` with `vcpkg`:

```bash
$ ./vcpkg.exe install openssl:x64-windows
$ ./vcpkg.exe integrate install
# you may want to add this to the system environment variables since you'll need it to compile the crate
$ set VCPKGRS_DYNAMIC=1
```

* Installing `openssl` with `chocolatey`:

```bash
$ choco install openssl
# you may need to set the OPENSSL_DIR environment variable
$ set OPENSSL_DIR="C:\Program Files\OpenSSL-Win64"
```

### macOS

`cmake` and `openssl` can be installed with `Homebrew`:

```bash
$ brew install cmake openssl@1.1
```

### Linux

Install `cmake` and `openssl` with your distro's package manager or download from their websites. On Debian and Ubuntu you will also need `build-essential`.

## Using the library

Using the library is easy, just add it as dependancy in `Cargo.toml`:

```bash
[dependencies]
iota-core = { git = "https://github.com/iotaledger/iota.rs", branch = "dev" }
```

And then you can use the library in your code with `iota`.

### Initialisation

This example fetches node information with  the
```rust
    let iota = Client::builder() // Crate a client instance builder
    .node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let node_info = iota.get_info().await.unwrap();
    println!("{:#?}", node_info);
```