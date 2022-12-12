# IOTA Client Java Library

Get started with the official IOTA Client Java Library.

## Requirements

Minimum Java version: Java 8

## Use in your Android project (Android Studio)

1. Download the `iota-client-1.0.0-rc.1.jar` file from the [GitHub release](https://github.com/iotaledger/iota.rs/releases/tag/iota-client-java-1.0.0-rc.1) and add it as a library to your project.
2. Download the `iota-client-1.0.0-rc.1-android.zip` file from the [GitHub release](https://github.com/iotaledger/iota.rs/releases/tag/iota-client-java-1.0.0-rc.1), unzip it and add the `jniLibs` folder with its contents to your Android Studio project as shown below:

```
project/
├──libs/
|  └── *.jar <-- if your library has jar files, they go here
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