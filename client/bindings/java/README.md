# IOTA Client Java Library

Get started with the official IOTA Client Java library.

## Requirements

minimum Java version >= 8

## Use in your Android project (Android Studio)

1. Download the `iota-client-1.0.0-rc.1.jar` file from the [GitHub release](https://github.com/iotaledger/iota.rs/releases/tag/iota-client-java-1.0.0-rc.1) and add it as a library to your project.
2. Download the `iota-client-1.0.0-rc.1-android.zip` file from the [GitHub release](https://github.com/iotaledger/iota.rs/releases/tag/iota-client-java-1.0.0-rc.1), unzip it and add the `jniLibs` folder with its contents to your Android Studio project as shown below:

```
project/
├──src/
   └── main/
       ├── AndroidManifest.xml
       ├── java/
       └── jniLibs/ 
           ├── arm64-v8a/           <-- ARM 64bit
           │   └── libiota-client.so
           │   └── libc++_shared.so
           ├── armeabi-v7a/         <-- ARM 32bit
           │   └── libiota-client.so
           │   └── libc++_shared.so
           │── x86/                 <-- Intel 32bit
           │  └── libiota-client.so
           │  └── libc++_shared.so
           └── x86_64/              <-- Intel 64bit
              └── libiota-client.so
              └── libc++_shared.so
```

## Use in your Java project (Linux, macOS, Windows)

Depending on your operating system, add one of the following dependencies to your `build.gradle` file:

#### linux-x86_64
```
implementation 'org.iota:iota-client:1.0.0-rc.1:linux-x86_64'
```

#### windows-x86_64
```
implementation 'org.iota:iota-client:1.0.0-rc.1:windows-x86_64'
```

#### aarch64-apple-darwin
```
implementation 'org.iota:iota-client:1.0.0-rc.1:aarch64-apple-darwin'
```

#### osx-x86_64
```
implementation 'org.iota:iota-client:1.0.0-rc.1:osx-x86_64'
```

## Use the Library

In order to use the library, you need to create a `Client` instance as illustrated below:

```java
import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.expections.ClientException;
import org.iota.types.responses.NodeInfoResponse;

public class HelloWorld {
    public static void main(String[] args) throws InitializeClientException, ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Get the node information for a given node.
        NodeInfoResponse response = client.getNodeInfo();

        // Print the URL of the node that was requested.
        System.out.println(response.getNodeUrl());

        // Print the node information for the requested node.
        System.out.println(response.getNodeInfo());
    }
}
```

## What's Next?

Now that you are up and running, you can get acquainted with the library using
its [how-to guides](https://wiki.iota.org/shimmer/iota.rs/how_tos/run_how_tos/) and the
repository's [code examples](https://github.com/iotaledger/iota.rs/tree/develop/client/bindings/java/examples/src).

## Instead, build everything from scratch yourself:

If you don't like to use the provided libraries and instead want to build everything yourself from scratch:

### Build for Android:

Requirements:

- minimum Java version >= 8
- Android Studio with NDK
- latest stable version of Rust
- cargo-ndk (https://github.com/bbqsrc/cargo-ndk)

1. Generate the JAR:
```
git clone https://github.com/iotaledger/iota.rs
cd iota.rs/client/bindings/java
./gradlew jarWithoutNativeLibs
```

2. You will find the built JAR in the `lib/build/libs` directory. Add it as a library to your Android project.

3. Install the Android targets you want to support:
```
 rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

4. Build the native library for your Android targets:
```
cd lib/native
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 -o ./jniLibs build --release
```

5. On success, you will find the built native libraries in the `jniLibs/` directory like:
```
── jniLibs/ 
    ├── arm64-v8a/           <-- ARM 64bit
    │   └── libiota-client.so
    ├── armeabi-v7a/         <-- ARM 32bit
    │   └── libiota-client.so
    │── x86/                 <-- Intel 32bit
    │  └── libiota-client.so
    └── x86_64/              <-- Intel 64bit
        └── libiota-client.so
```

6. Each folder is missing its `libc++_shared.so`. You can find them in the configured Android NDK folder like:
```
find $ANDROID_NDK_HOME -name "libc++_shared.so"
```

7. Copy the found `libc++_shared.so` files to their respective folder inside the `jniLibs` directory:
```
── jniLibs/ 
    ├── arm64-v8a/           <-- ARM 64bit
    │   └── libiota-client.so
    │   └── libc++_shared.so
    ├── armeabi-v7a/         <-- ARM 32bit
    │   └── libiota-client.so
    │   └── libc++_shared.so
    │── x86/                 <-- Intel 32bit
    │  └── libiota-client.so
    │  └── libc++_shared.so
    └── x86_64/              <-- Intel 64bit
        └── libiota-client.so
        └── libc++_shared.so
```

8. Add the `jniLibs` folder with its contents to your Android Studio project as shown below:
```
project/
├──src/
   └── main/
       ├── AndroidManifest.xml
       ├── java/
       └── jniLibs/ 
           ├── arm64-v8a/           <-- ARM 64bit
           │   └── libiota-client.so
           │   └── libc++_shared.so
           ├── armeabi-v7a/         <-- ARM 32bit
           │   └── libiota-client.so
           │   └── libc++_shared.so
           │── x86/                 <-- Intel 32bit
           │  └── libiota-client.so
           │  └── libc++_shared.so
           └── x86_64/              <-- Intel 64bit
              └── libiota-client.so
              └── libc++_shared.so
```

### Build for Linux, macOS, Windows

Please note, following instructions build the library for your host OS/architecture only.

Requirements:

- minimum Java version >= 8
- latest stable version of Rust

1. Generate the JAR:
```
git clone https://github.com/iotaledger/iota.rs
cd iota.rs/client/bindings/java
./gradlew jar
```

2. You will find the built JAR in the `lib/build/libs` directory. Add it as a library to your Java project.