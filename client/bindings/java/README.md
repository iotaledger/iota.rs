---
description: Get started with the official IOTA Client Java library.
image: /img/logo/iota_mark_light.png
keywords:

- Java
- jar
- Maven
- Gradle

---
# IOTA Client Java Library

Get started with the official IOTA Client Java Library.

## Requirements

* Make sure you have the latest [Java Development Kit (JDK)](https://www.oracle.com/java/technologies/downloads/) installed.

## Install the Library with Maven

#### linux-x86_64
```xml
<dependency>
    <groupId>org.iota</groupId>
    <artifactId>iota-client</artifactId>
    <version>1.0.0-rc.1</version>
    <type>jar</type>
    <classifier>linux-x86_64</classifier>
</dependency>
```

#### windows-x86_64
```xml
<dependency>
    <groupId>org.iota</groupId>
    <artifactId>iota-client</artifactId>
    <version>1.0.0-rc.1</version>
    <type>jar</type>
    <classifier>windows-x86_64</classifier>
</dependency>
```

#### aarch64-apple-darwin
```xml
<dependency>
    <groupId>org.iota</groupId>
    <artifactId>iota-client</artifactId>
    <version>1.0.0-rc.1</version>
    <type>jar</type>
    <classifier>aarch64-apple-darwin</classifier>
</dependency>
```

#### osx-x86_64
```xml
<dependency>
    <groupId>org.iota</groupId>
    <artifactId>iota-client</artifactId>
    <version>1.0.0-rc.1</version>
    <type>jar</type>
    <classifier>osx-x86_64</classifier>
</dependency>
```

## Use the Library

In order to use the library, you need to create a _Client_:

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
its [how-to guides](../../../documentation/docs/how_tos/00_run_how_tos.mdx) and the
repository's [code examples](https://github.com/iotaledger/iota.rs/tree/develop/client/bindings/java/iota-client-java/examples/src).