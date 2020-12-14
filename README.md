# iota.rs

🚧 This repository now targets Chrysalis part 2! It will have many breaking change along the way. Please use with caution! 🚧

This is the **alpha** version of official Rust library, which allows you to do the following:
* Create transactions
* Sign transactions
* Generate addresses
* Interact with an IOTA node

This client library is still in the alpha stage but it should cover most use cases. The main crate is under `iota-core`. The librarnamed `iota` re-exports fundamental crates from `bee` and also provides client features and utilities. API calls like `post_message` and `get_outputs` are supported. There may be some performance and stability issues. Please report any issues in our [issue tracker](https://github.com/iotaledger/iota.rs/issues).

| Table of contents                                 |
| :------------------------------------------------ |
| [Prerequisites](#prerequisites)                   |
| [Using the library](#installing-the-library)      |
| [API reference](#api-reference)                   |
| [Examples](#examples)                             |
| [Supporting the project](#supporting-the-project) |
| [Joining the discussion](#joining-the-discussion) |
| [License](#license)                               |

## Requirements

To use the library, we recommend you update Rust to latest stable version [`$ rustup update stable`](https://github.com/rust-lang/rustup.rs#keeping-rust-up-to-date). Nightly should be fine but some changes might not be compatible.

`no_std` is not currently supported, but we are working on it in [bee](https://github.com/iotaledger/bee), and will provide it as feature once new library implementation is ready.

### Dependencies

`cmake` and `openssl` are required to compile the MQTT dependencies.

### Windows

`cmake` can be downloaded on the [official website](https://cmake.org/download/) and `openssl` can be installed with [vcpkg](https://github.com/microsoft/vcpkg) or [chocolatey](https://chocolatey.org/).

- Installing `openssl` with `vcpkg`:

```
$ ./vcpkg.exe install openssl:x64-windows
$ ./vcpkg.exe integrate install
# you may want to add this to the system environment variables since you'll need it to compile the crate
$ set VCPKGRS_DYNAMIC=1
```

- Installing `openssl` with `chocolatey`:

```
$ choco install openssl
# you may need to set the OPENSSL_ROOT_DIR environment variable
$ set OPENSSL_ROOT_DIR="C:\Program Files\OpenSSL-Win64"
```

### macOS

`cmake` and `openssl` can be installed with `Homebrew`:

```
$ brew install cmake
$ brew install openssl@1.1
# you may want to add this to your .zshrc or .bashrc since you'll need it to compile the crate
$ OPENSSL_ROOT_DIR=$(brew --prefix openssl@1.1)
```

### Linux

Install `cmake` and `openssl` with your distro's package manager or download from their websites.

## Using the library

Using the library is fairly easy, just add it as dependancy in `Cargo.toml`:

```
[dependencies]
iota-core = { git = "https://github.com/iotaledger/iota.rs", branch = "dev" }
```

And then you can use the library in your code with `iota`.

## API reference

You can read the [API reference](https://docs.rs/iota-core) here, or generate the reference yourself.

If you'd like to explore the implementation in more depth, the following command generates docs for the whole crate, including private modules:

```
cargo doc --document-private-items --no-deps --open
```

## Examples

You can see the examples in the [examples](examples/) directory and try them with:

```
cargo run --example balance
```

## Supporting the project

## Joining the discussion

If you want to get involved in the community, need help with setting up, have any issues or just want to discuss IOTA with other people, feel free to join our [Discord](https://discord.iota.org/).

## License

The Apache 2.0 license can be found [here](LICENSE).
