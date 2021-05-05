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


- Making a library jar using gradle:
```
$ cd iota.rs/bindings/java/lib
$ ./gradlew assemble
```

- Running an example using gradle
```
$ cd iotaf.rs/bindings/java
$ ./gradlew examples:basic-app:test --info
```

Make sure to make gradlew executable (`chmod +x gradlew`)

## Documentation

Documentation can be found [here](https://client-lib.docs.iota.org/overview/index.html).
