---
description: Getting started with the official IOTA Client Library Java binding.
image: /img/logo/iota_mark_light.png
keywords:

- Java
- Rust
- jar
- Gradle

---

# Getting Started with Java

To use the IOTA Java Client Library in your Java project, you must first build the library JAR for your operating
system.

## Build the JAR for your operating system (Linux, macOS, Windows)

**To build your JAR, you must ensure that you have the latest stable version of Rust installed.
Visit [Install Rust](https://www.rust-lang.org/tools/install) for installing Rust.
In addition, make sure you have the latest Java Development Kit (JDK) installed.**

1. Clone the repository: `git clone https://github.com/iotaledger/iota.rs`
2. Change directory: `cd iota.rs/bindings/java/iota-client-java`
3. If needed make `gradlew` executable: `chmod +x gradlew`
4. Build your JAR: `./gradlew jar`
5. Find the produced JAR in: `build/libs/`
6. Add the JAR as a library to your Java project.

After you linked the library, you can create a Client instance and interface with it.

```java
import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.responses.NodeInfoResponse;

public class HelloWorld {
    public static void main(String[] args) throws ClientException {
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