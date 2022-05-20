package org.iota.tests;

import org.iota.main.Client;
import org.iota.main.types.ClientConfig;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class TestMessageInterface {

    private static final String nodeUrl = "https://api.alphanet.iotaledger.net";
    private Client client;

    @BeforeEach
    void setUp() {
        client = new Client(new ClientConfig("{ \"nodes\": [\"" + nodeUrl + "\" ], \"nodeSyncEnabled\": false}"));
    }

    @Test
    void testGetHealth() {
        String response = client.getHealth(nodeUrl);
        System.out.println(response);
    }

    @Test
    void testGetNodeInfo() {
        String response = client.getNodeInfo();
        System.out.println(response);
    }

}
