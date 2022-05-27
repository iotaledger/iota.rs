package org.iota.tests;

import org.iota.main.Client;
import org.iota.main.apis.UtilsApi;
import org.iota.main.types.*;
import org.iota.main.types.responses.*;
import org.iota.main.types.secret.GenerateAddressesOptions;
import org.iota.main.types.secret.GenerateBlockOptions;
import org.iota.main.types.secret.MnemonicSecretManager;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class TestMessageInterface {

    private static final String DEFAULT_DEVNET_NODE_URL = "https://api.alphanet.iotaledger.net";
    private static final String DEFAULT_DEVNET_FAUCET_URL = "https://faucet.alphanet.iotaledger.net";
    private static final String DEFAULT_DEVELOPMENT_MNEMONIC = "hidden enroll proud copper decide negative orient asset speed work dolphin atom unhappy game cannon scheme glow kid ring core name still twist actor";

    private Client client;
    private ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_DEVNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    @BeforeEach
    void setUp() {
        client = new Client(config);
    }

    void requestFundsFromFaucet(String address) throws ClientException {
        new UtilsApi(config).requestFundsFromFaucet(DEFAULT_DEVNET_FAUCET_URL, address);
    }

    Block setUpTaggedDataBlock() throws ClientException {
        BlockResponse r = client.submitBlockPayload(new BlockPayload("{ \"type\": 5, \"tag\": \"0x68656c6c6f20776f726c64\", \"data\": \"0x5370616d6d696e6720646174612e0a436f756e743a203037323935320a54696d657374616d703a20323032312d30322d31315431303a32333a34392b30313a30300a54697073656c656374696f6e3a203934c2b573\" }"));
        return r.getBlock();
    }

    Block setUpTransactionBlock() throws ClientException {
        String address = client.generateAddresses(new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC), new GenerateAddressesOptions().withRange(0, 1)).getAddresses()[0];
        requestFundsFromFaucet(address);
        try {
            Thread.sleep(1000 * 10);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        Block b = client.generateBlock(new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC), new GenerateBlockOptions().withOutputHex(new GenerateBlockOptions.ClientBlockBuilderOutputAddress(client.bech32ToHex(address).getHexAddress(), "10000000"))).getBlock();
        try {
            Thread.sleep(1000 * 10);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        return b;
    }

    String setUpTransactionId() throws ClientException {
        Block b = setUpTransactionBlock();
        String transactionId = client.getTransactionId(new BlockPayload(b.getJson().get("payload").getAsJsonObject())).getTransactionId();
        return transactionId;
    }

    String setupOutputId() throws ClientException {
        return setUpTransactionId() + "0000";
    }

    // Node Core API tests

    @Test
    public void testGetHealth() throws ClientException {
        HealthResponse r = client.getHealth(DEFAULT_DEVNET_NODE_URL);
        System.out.println(r.isHealthy());
    }

    @Test
    public void testGetNodeInfo() throws ClientException {
        NodeInfoResponse r = client.getNodeInfo();
        System.out.println(r.getNodeInfo());
    }

    @Test
    public void testGetTips() throws ClientException {
        TipsResponse r = client.getTips();
        for (String tip : r.getTips())
            System.out.println(tip);
    }

    @Test
    public void testPostBlock() throws ClientException {
        PostBlockResponse r = client.postBlock(setUpTaggedDataBlock());
        System.out.println(r.getBlockId());
    }

    @Test
    public void testGetBlock() throws ClientException {
        BlockResponse r = client.getBlock(client.postBlock(setUpTaggedDataBlock()).getBlockId());
        System.out.println(r.getBlock());
    }

    @Test
    public void testGetBlockRaw() throws ClientException {
        BlockRawResponse r = client.getBlockRaw(client.postBlock(setUpTaggedDataBlock()).getBlockId());
        System.out.println(r);
    }

    @Test
    public void testGetBlockMetadata() throws ClientException {
        BlockMetadataResponse r = client.getBlockMetadata(client.postBlock(setUpTaggedDataBlock()).getBlockId());
        System.out.println(r);
    }

    @Test
    public void testGetBlockChildren() throws ClientException {
        BlockChildrenResponse r = client.getBlockChildren(client.postBlock(setUpTaggedDataBlock()).getBlockId());
        for (String child : r.getBlockChildren())
            System.out.println(child);
    }

    @Test
    public void testGetOutput() throws ClientException {
        OutputResponse r = client.getOutput(setupOutputId());
        System.out.println(r);
    }

    @Test
    public void testGetOutputMetadata() throws ClientException {
        OutputMetadataResponse r = client.getOutputMetadata(setupOutputId());
        System.out.println(r);
    }

    @Test
    public void testGetReceiptsMigratedAt() throws ClientException {
        ReceiptsMigratedAtResponse r = client.getReceiptsMigratedAt(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        for (Receipt receipt : r.getReceipts())
            System.out.println(receipt);

    }

    @Test
    public void testGetReceipts() throws ClientException {
        ReceiptsResponse r = client.getReceipts();
        for (Receipt receipt : r.getReceipts())
            System.out.println(receipt);

    }

    @Test
    public void testGetTreasury() throws ClientException {
        TreasuryResponse r = client.getTreasury();
        System.out.println(r.getMilestoneId());
        System.out.println(r.getAmount());
    }

    @Test
    public void testGetIncludedBlock() throws ClientException {
        BlockResponse r = client.getIncludedBlock(setUpTransactionId());
        System.out.println(r.getBlock());
    }

    @Test
    public void testGetMilestoneById() throws ClientException {
        MilestoneResponse r = client.getMilestoneById(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("milestoneId").getAsString());
        System.out.println(r.getMilestone());
    }

    @Test
    public void testGetMilestoneByIndex() throws ClientException {
        MilestoneResponse r = client.getMilestoneByIndex(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        System.out.println(r.getMilestone());
    }

    @Test
    public void testGetMilestoneByIdRaw() throws ClientException {
        MilestoneRawResponse r = client.getMilestoneByIdRaw(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("milestoneId").getAsString());
        System.out.println(r.getMilestoneBytes());
    }

    @Test
    public void testGetMilestoneByIndexRaw() throws ClientException {
        MilestoneRawResponse r = client.getMilestoneByIndexRaw(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        System.out.println(r.getMilestoneBytes());
    }

    @Test
    public void testGetUtxoChangesId() throws ClientException {
        UtxoChangesResponse r = client.getUtxoChangesById(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("milestoneId").getAsString());
        for(String consumed: r.getConsumedOutputs())
            System.out.println(consumed);
        for(String created: r.getCreatedOutputs())
            System.out.println(created);
    }

    @Test
    public void testGetUtxoChangesIndex() throws ClientException {
        UtxoChangesResponse r = client.getUtxoChangesByIndex(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        for(String consumed: r.getConsumedOutputs())
            System.out.println(consumed);
        for(String created: r.getCreatedOutputs())
            System.out.println(created);
    }

    @Test
    public void testGetPeers() throws ClientException {
        PeersResponse r = client.getPeers();
        for(Peer peer: r.getPeers())
            System.out.println(peer);
    }



}
