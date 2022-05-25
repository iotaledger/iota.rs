package org.iota.tests;

import org.iota.main.Client;
import org.iota.main.types.Block;
import org.iota.main.types.BlockPayload;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.responses.*;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class TestMessageInterface {

    private static final String DEFAULT_DEVNET_NODE_URL = "https://api.alphanet.iotaledger.net";
    private static final String DEFAULT_DEVNET_FAUCET_URL = "https://faucet.alphanet.iotaledger.net";

    private Client client;

    @BeforeEach
    void setUp() {
        client = new Client(new ClientConfig("{ \"nodes\": [\"" + DEFAULT_DEVNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}"));
    }

    Block setupTaggedDataBlock() throws ClientException {
        GetBlockResponse r = client.submitBlockPayload(new BlockPayload("{ \"type\": 5, \"tag\": \"0x68656c6c6f20776f726c64\", \"data\": \"0x5370616d6d696e6720646174612e0a436f756e743a203037323935320a54696d657374616d703a20323032312d30322d31315431303a32333a34392b30313a30300a54697073656c656374696f6e3a203934c2b573\" }"));
        return r.getBlock();
    }

    // Node Core API tests

    @Test
    public void testGetHealth() throws ClientException {
        GetHealthResponse r = client.getHealth(DEFAULT_DEVNET_NODE_URL);
        System.out.println(r.isHealthy());
    }

    @Test
    public void testGetNodeInfo() throws ClientException {
        GetNodeInfoResponse r = client.getNodeInfo();
        System.out.println(r.getNodeInfo());
    }

    @Test
    public void testGetTips() throws ClientException {
        GetTipsResponse r = client.getTips();
        for (String tip : r.getTips())
            System.out.println(tip);
    }

    @Test
    public void testPostBlock() throws ClientException {
        PostBlockResponse r = client.postBlock(setupTaggedDataBlock());
        System.out.println(r.getBlockId());
    }

    @Test
    public void testGetBlock() throws ClientException {
        GetBlockResponse r = client.getBlock(client.postBlock(setupTaggedDataBlock()).getBlockId());
        System.out.println(r);
    }

    @Test
    public void testGetBlockRaw() throws ClientException {
        GetBlockRawResponse r = client.getBlockRaw(client.postBlock(setupTaggedDataBlock()).getBlockId());
        System.out.println(r);
    }

    @Test
    public void testGetBlockMetadata() throws ClientException {
        GetBlockMetadataResponse r = client.getBlockMetadata(client.postBlock(setupTaggedDataBlock()).getBlockId());
        System.out.println(r);
    }

    @Test
    public void testGetBlockChildren() throws ClientException {
        GetBlockChildrenResponse r = client.getBlockChildren(client.postBlock(setupTaggedDataBlock()).getBlockId());
        for (String child : r.getBlockChildren())
            System.out.println(child);
    }

}
