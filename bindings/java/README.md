# IOTA Client Library - Java binding

Java binding to the iota.rs library.

## Requirements

Ensure you have first installed the required dependencies for the library [here](https://github.com/iotaledger/iota.rs/blob/dev/README.md).

## Installation

Clone project
```
git clone https://github.com/iotaledger/iota.rs
```

Build the rust library
```
cd iota.rs/bindings/java
cargo build --release
```

- Running an example using gradle
```
cd iota.rs/bindings/java
./gradlew examples:basic-app:test --info
```

Make sure to make gradlew executable (`chmod +x gradlew`)

- Running an example using maven
```
cd iota.rs/bindings/java/examples/basic-app
mvn test
```

## Example for an external project
Project skeleton can be found [here](https://github.com/kwek20/iota-rs-java).

## Documentation

Documentation can be found [here](https://client-lib.docs.iota.org/overview/index.html).

## Migration from old iota.rs java bindings seed

Due to a fault conversion from Java to Rust, were created different.
For more details see iota.rs PR [800](https://github.com/iotaledger/iota.rs/pull/800).

A migration example has been created, which can be found at `LibraryTest.testMigrateOldSeedUsage()` (`iota.rs/bindings/java/examples/basic-app/src/test/java/org/example/LibraryTest.java`)
