package org.iota.tests;

import org.iota.main.types.ClientException;
import org.iota.main.types.Peer;
import org.iota.main.types.Receipt;
import org.iota.main.types.responses.node_core_api.*;
import org.junit.jupiter.api.Test;

public class NodeCoreApiTest extends ApiTest {

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
    public void testGetOutput() throws ClientException {
        OutputResponse r = client.getOutputWithMetadata(setupOutputId());
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
        for (String consumed : r.getConsumedOutputs())
            System.out.println(consumed);
        for (String created : r.getCreatedOutputs())
            System.out.println(created);
    }

    @Test
    public void testGetUtxoChangesIndex() throws ClientException {
        UtxoChangesResponse r = client.getUtxoChangesByIndex(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        for (String consumed : r.getConsumedOutputs())
            System.out.println(consumed);
        for (String created : r.getCreatedOutputs())
            System.out.println(created);
    }

    @Test
    public void testGetPeers() throws ClientException {
        PeersResponse r = client.getPeers();
        for (Peer peer : r.getPeers())
            System.out.println(peer);
    }

}
