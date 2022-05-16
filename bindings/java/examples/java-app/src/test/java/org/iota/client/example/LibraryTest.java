package org.iota.client.example;

import java.util.Arrays;
import java.util.Date;
import java.util.UUID;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;

import static org.junit.Assert.*;	
import org.junit.Test;
import org.junit.Before;

import org.iota.client.*;
import org.iota.client.local.*;

public class LibraryTest {

    static {
        NativeAPI.verifyLink();
    }

    private static final String NODE = ExampleApp.NODE;

    private Client iota;

    @Before
    public void init() {
        iota = node();
    }

    @Test
    public void testNodeInfo() {
        NodeInfoWrapper info = iota.getInfo();

        assertEquals(info.url(), NODE);
        assertNotNull(info.nodeInfo());
    }

    @Test
    public void mqtt() throws InterruptedException {
        AtomicBoolean received = new AtomicBoolean(false);
        MqttListener listener = new MqttListener() {
            @Override
            public void onEvent(TopicEvent event){ 
                Message message = Message.deserialize(event.payload());

                received.set(true);
            }
        };

        iota.subscriber().withTopic(Topic.from("messages")).subscribe(listener);

        while (!received.get()) {
            TimeUnit.SECONDS.sleep(1);
        }

        // unsubscribe from 'messages' topic
        iota.subscriber().withTopic(Topic.from("messages")).unsubscribe();
    }

    @Test
    public void testIndexable() {
        String index = "Hello " + new Date().toString();
        String data = "Iota.rs java test";
        Message message1 = iota.message().withIndexString(index).withDataString(data).finish();

        // Fetch message Ids
        MessageId[] fetched_message_ids = iota.getMessage().indexString(index);
        assertTrue(fetched_message_ids.length == 1);
        assertEquals(fetched_message_ids[0], message1.id());

        // get message data
        Message message1Remote = iota.getMessage().data(fetched_message_ids[0]);
        assertTrue(message1Remote.payload().isPresent());
        verifyPayload(message1Remote.payload().get().asIndexation(), index, data);

        // Make index from raw index payload
        Message message2 = iota.message().finishIndex(IndexationPayload.fromStrings(index, data));
        // Fetch message Ids
        fetched_message_ids = iota.getMessage().indexString(index);
        assertTrue(fetched_message_ids.length == 2);
        
        // get message data
        message1Remote = iota.getMessage().data(fetched_message_ids[0]);
        assertTrue(message1Remote.payload().isPresent());
        verifyPayload(message1Remote.payload().get().asIndexation(), index, data);

        String serialised = message1Remote.payload().get().asIndexation().serialize();
        IndexationPayload deserialised = IndexationPayload.deserialize(serialised);
        verifyPayload(deserialised, index, data);
    }

    private void verifyPayload(IndexationPayload payload, String index, String data){
        java.nio.charset.Charset charset = java.nio.charset.Charset.forName("UTF-8");

        assertEquals(payload.indexString(), index);
        assertArrayEquals(payload.index(), index.getBytes(charset));
        assertEquals(new String(payload.index(), charset), index);

        assertEquals(payload.dataString(), data);
        assertArrayEquals(payload.data(), data.getBytes(charset));
        assertEquals(new String(payload.data(), charset), data);
    }

    public void testMigrateOldSeedUsage() {
        Client node = node();

        SecretKey secret_key = SecretKey.generate();
        String seed = RustHex.encode("NONSECURE_USE_OF_DEVELOPMENT_SEED_1");
        assertEquals(seed.toString(), "4e4f4e5345435552455f5553455f4f465f444556454c4f504d454e545f534545445f31");
        System.out.println("seed: " + seed);

        // Send to the first address here using faucet
        System.out.println("Addresses old: ");
        String[] addresses_old = GetAddressesBuilder.fromOld(seed).withClient(node).withRange(0, 1).finish();
        assertEquals(addresses_old[0], "atoi1qzzj3wa2c0m0mpe6s2v004037sjhyk7zgr7hj3umwgnanr9xy6c92qyz3c8");
        System.out.println(Arrays.toString(addresses_old));
        
        // Receive the migration on address here
        System.out.println("Addresses new: ");
        String[] addresses = GetAddressesBuilder.from(seed).withClient(node).withRange(0, 1).finish();
        assertEquals(addresses[0], "atoi1qp5dzudmpxxz7xxlzez8w5ttefeanhpf9rju48ds5y2ellp6aauuztf0dyd");
        System.out.println(Arrays.toString(addresses));
        
        // Try to migrate the first 50 public addresses
        long maxAddressIndex= 50;
        long accountIndex = 0;
        boolean internal_address = true;

        for (long addressIndex = 0; addressIndex < maxAddressIndex; addressIndex++) {
            if (node.shouldMigrate(seed, accountIndex, addressIndex, internal_address)) {
                System.out.println("Migrating balance...");
                String address_to_migrate_to = addresses[0];
                Message message = node.migrate(seed, accountIndex, addressIndex, internal_address, address_to_migrate_to);
                System.out.println("Message: " + message);
            }
        }
    }

    @Test
    public void testSeedGen() {
        String seed = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";
        AddressStringPublicWrapper[] addresses = GetAddressesBuilder.from(seed)
            .withBech32Hrp("atoi")
            .withAccountIndex(0)
            .withRange(0, 1)
            .getAll();

        assertEquals(addresses[0].address(), "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r");
        assertTrue(!addresses[0].isPublic());
        assertEquals(addresses[1].address(), "atoi1qprxpfvaz2peggq6f8k9cj8zfsxuw69e4nszjyv5kuf8yt70t2847shpjak");
        assertTrue(addresses[1].isPublic());
    }

    @Test
    public void public_key_to_address() {
        Client iota = Client.Builder().withOfflineMode().finish();
    
        String hex_public_key = "2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a";
        String public_key_address = iota
            .hexPublicKeyToBech32Address(hex_public_key, "atoi");
        assertEquals(
            public_key_address,
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r");
    }

    private static Client node() {
        String nodeUrl = NODE;
        Client iota = Client.Builder().withNode(nodeUrl) // Insert your node URL here
                // .withNodeAuth("https://somechrysalisiotanode.com", "jwt_or_null",
                // "name_or_null", "password_or_null") //
                // Optional authentication
                .withLocalPow(true)
                .finish();
        return iota;
    }
}
