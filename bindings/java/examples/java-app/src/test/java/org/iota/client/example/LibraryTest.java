package org.iota.client.example;

import java.util.Arrays;

import static org.junit.Assert.*;	
import org.junit.Test;

import org.iota.client.*;
import org.iota.client.local.*;

public class LibraryTest {

    static {
        NativeAPI.verifyLink();
    }

    private static final String NODE = ExampleApp.NODE;

    @Test
    public void testNodeInfo() {
        ExampleApp.nodeInfo();
    }

    @Test
    public void testMigrateOldSeedUsage() {
        try {
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
        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
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
