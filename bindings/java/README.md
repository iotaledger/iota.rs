# IOTA Client Library - Java binding

Java binding to the iota.rs library.

## Requirements

Ensure you have first installed the required dependencies for the library [here](https://github.com/iotaledger/iota.rs/blob/dev/README.md).

## Installation

Clone project
```
$ git clone https://github.com/iotaledger/iota.rs
```

Build the rust library
```
$ cd iota.rs/bindings/java/native
$ cargo build
```

- Running an example using gradle
```
$ cd iota.rs/bindings/java
$ ./gradlew examples:basic-app:test --info
```

Make sure to make gradlew executable (`chmod +x gradlew`)

## Example for an external project
Project skeleton can be found [here](https://github.com/kwek20/iota-rs-java).

## Documentation

Documentation can be found [here](https://client-lib.docs.iota.org/overview/index.html).
