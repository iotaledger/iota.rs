---
description: Official IOTA Client Library Java API examples.
image: /img/logo/iota_mark_light.png
keywords:
- api
- Java
- examples
- type
- node
- client
---
# Examples

It's possible to send transactions with iota.rs, but we strongly recommend to use official `wallet.rs` library together with `stronghold.rs` enclave for value-based transfers. This combination incorporates the best security practices while dealing with seeds, related addresses and `UTXO`. See more information on [wallet docs](https://chrysalis.docs.iota.org/libraries/wallet).

```bash
git clone https://github.com/iotaledger/iota.rs
```

```bash
cd iota.rs/bindings/java
```

Examples are all collected in a sample project. By default it runs a node info example, but there are many more.

Run the example like:

Gradle: `./gradlew examples:basic-app:test --info`

Maven: `cd examples/basic-app && mvn test`


For the rest of the examples in this document we will be using the `node()` method below:
```java
private static Client node() {
    String nodeUrl = "https://chrysalis-nodes.iota.cafe:443";
    Client iota = Client.Builder()
        // Insert your node URL here
        .withNode(nodeUrl) 
        // Or instead here but with authentication
        .withNodeAuth("https://somechrysalisiotanode.com", "jwt_or_null", "name_or_null", "password_or_null")
        // Choose pow mode
        .withLocalPow(true)
        // You can also set a time-out in seconds for the API calls
        .withRequestTimeout(5)
        //Then create the Client instance
        .finish();
    return iota;
}
```

***

The most basic example is creating a client, and then requesting the information about the node. 
```java
Client iota = node();

System.out.println("Node healthy: " + iota.getHealth());

// Get information about our node
NodeInfoWrapper info = iota.getInfo();
// This returns the URL from the node the info as gotten from (in case of a quorum or multipe nodes)
System.out.println("Node url: " + info.url());
// And the actual information
System.out.println("Node Info: " + info.nodeInfo());
```

Example output of the code would be:
```bash
Node healthy: true
Node url: https://chrysalis-nodes.iota.cafe
Node Info: name=HORNET, version=1.0.1, is_healthy=true, network_id=chrysalis-mainnet, bech32_hrp=iota, min_pow_score=4000
            , messages_per_second=19.9, referenced_messages_per_second=20.2, referenced_rate=101.5075376884422
            , latest_milestone_timestamp=1627055424, latest_milestone_index=739379, confirmed_milestone_index=739379
            , pruning_index=678884, features=(["PoW"])
```

***

Generate a seed and use that to get addresses.
You can of course use any other method to generate a seed, but SecretKey is used in iota client.
```java
// Secret keys have more use than just beeing a seed, but you can call toString to get the hex representation
SecretKey secret_key = SecretKey.generate();

Client iota = node();
String[] addresses = new GetAddressesBuilder(secret_key.toString()).withClient(iota).withRange(0, 10).finish();
System.out.println(Arrays.toString(addresses));
```

In this example we send a very simple, empty, message and get the metadata of that one.
Then we send a message by index, and search for that index on the node again to find messageIds.
```java
Client iota = node();
// Make and send an empty message
Message messageToSend = iota.message().finish();

// getMessage.metadata() returns message metadata from the MessageId we supplied
MessageMetadata metadata = iota.getMessage().metadata(message.id());
System.out.println("Message metadata: " + metadata);

// Now we send a message by index "Hello". The message itself will contain "Tangle" as data here, but this coulld be anything.
Message message = iota.message().withIndexString("Hello").withDataString("Tangle").finish();
System.out.println("Message sent https://explorer.iota.org/mainnet/message/" + message.id());

// Lets find all messages with the "Hello" index.
// This will include the message we just send
MessageId[] fetched_message_ids = iota.getMessage().indexString("Hello");

// With these ids, we could look up the content on a per-id bases
System.out.println("Messages with Hello index: " + Arrays.toString(fetched_message_ids));
```

Example output of the code would be:
```bash
Message metadata: message_id=adb62e03b420aa323b40a5fc341c9c51cf2dd2031d52618cfa389ecb404bb3ab, parent_message_ids=(["105e11f8d23eeaee9797e1fa4a78ffe887e1f8f1ee4df741decf3f15ef1695f3", "ddc2a9a986682bc2cc735979c6e0fdf2952513ecd84c02242fbb084d1492c819", "f1370ee1207a6e3b2ed1d3cbe7f68757f076c42df87165672d3598737736855b", "fb633fe598d58d3287a9fcdeea1134fec83858ed28c549f2725898e0030d9ae5"]), is_solid=true, referenced_by_milestone_index=None, milestone_index=None, ledger_inclusion_state=None, conflict_reason=None, should_promote=Some(false), should_reattach=Some(false)

Message sent https://explorer.iota.org/mainnet/message/32e75774837b2f26f8fee1f2a1f22076fd80b555a6e2515f4f48e8259234e81d

Messages with Hello index: [32e75774837b2f26f8fee1f2a1f22076fd80b555a6e2515f4f48e8259234e81d, 65a68b9ae0e138b13db11a5bad642ecf2cb5cc0f5d31b6396f68e0cfa5ef2d33]
```

***

You can find more advanced examples in the [examples](https://github.com/iotaledger/iota.rs/tree/dev/bindings/java/examples/basic-app) folder.
