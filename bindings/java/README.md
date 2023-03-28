# IOTA Client Library - Java binding

Java binding to the iota.rs library.

## Requirements

Ensure you have first installed the required dependencies for the library [here](https://github.com/iotaledger/iota.rs/blob/dev/README.md).

- A valid C compiler
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) to clone the repository
- [Java](https://openjdk.org/install/) JDK and JRE, Java 8 or higher
- [Rust](https://www.rust-lang.org/tools/install) installation on your path
- [Gradle](https://gradle.org/install/) v4 or higher or [Maven](https://maven.apache.org/download.cgi) installed

## Installation

Clone project
```
git clone -b production https://github.com/iotaledger/iota.rs
```

Build the rust library (This generates the java source code and JNI library file)
```
cd iota.rs/bindings/java
cargo build --release
```

Source code will be generated under `iota.rs/bindings/java/native/src/main/java/org/iota/client`

Binaries can be found at `iota.rs/bindings/java/target/release`

Once this step succeeds we need to generate the jar file containing the newly generated Java source files.
### Gradle

Make `gradlew` executable (`chmod +x gradlew`) if needed, then run
```
cd iota.rs/bindings/java
./gradlew jar
```

### Maven
```
cd iota.rs/bindings/java
mvn install
```

The jar will be found at `iota.rs/bindings/java/native/build/libs/native.jar`

## Running the Java example

### Gradle
```
./gradlew examples:java-app:test --info
```

### Maven
```
mvn exec:exec
```

## Running the Android example
The Android app needs further compilation instructions.

Specific instructions in `iota.rs/bindings/java/examples/android-app/README.md`

## Example for an external project
Project skeleton can be found [here](https://github.com/kwek20/iota-rs-java).

## Documentation

Documentation can be found [here](https://client-lib.docs.iota.org/overview/index.html).

## Migration from old iota.rs java bindings seed

Due to a fault conversion from Java to Rust, were created different.
For more details see iota.rs PR [800](https://github.com/iotaledger/iota.rs/pull/800).

A migration example has been created, which can be found at `LibraryTest.testMigrateOldSeedUsage()` (`iota.rs/bindings/java/examples/basic-app/src/test/java/org/example/LibraryTest.java`)
