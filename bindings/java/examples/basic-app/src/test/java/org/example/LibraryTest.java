package org.example;

import java.util.Arrays;

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
            System.out.println("seed: " + seed);

            // Send to the first address here using faucet
            System.out.println("Addresses old: ");
            String[] addresses_old = GetAddressesBuilder.fromOld(seed).withClient(node).withRange(0, 1).finish();
            System.out.println(Arrays.toString(addresses_old));
            
            // Receive the migration on address here
            System.out.println("Addresses new: ");
            String[] addresses = GetAddressesBuilder.from(seed).withClient(node).withRange(0, 1).finish();
            System.out.println(Arrays.toString(addresses));
            
            long accountIndex = 0, seedIndex = 0;
            boolean public_address = true;
            if (node.shouldMigrate(seed, accountIndex, seedIndex, public_address)) {
                System.out.println("Migrating balance...");
                String address_to_migrate_to = addresses[0];
                Message message = node.migrate(seed, accountIndex, seedIndex, public_address, address_to_migrate_to);
                System.out.println("Message: " + message);
            } else {
                System.out.println("Did not need to migrate seed index");
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
