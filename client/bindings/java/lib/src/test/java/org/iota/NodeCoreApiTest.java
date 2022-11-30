// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import org.iota.types.*;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.NoFundsReceivedFromFaucetException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.MilestoneId;
import org.iota.types.ids.OutputId;
import org.iota.types.responses.NodeInfoResponse;
import org.iota.types.responses.TreasuryResponse;
import org.iota.types.responses.UtxoChangesResponse;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import java.util.Base64;
import java.util.Map;

public class NodeCoreApiTest extends ApiTest {

    @Test
    public void testGetHealth() throws ClientException {
        boolean health = client.getHealth(DEFAULT_TESTNET_NODE_URL);
        System.out.println(health);
    }

    @Test
    public void testGetNodeInfo() throws ClientException {
        NodeInfoResponse r = client.getNodeInfo();
        System.out.println(r.getNodeInfo());
    }

    @Test
    public void testGetTips() throws ClientException {
        for (BlockId tip : client.getTips())
            System.out.println(tip);
    }

    @Test
    public void testPostBlock() throws ClientException {
        BlockId blockId = client.postBlock(setUpTaggedDataBlock());
        System.out.println(blockId);
    }

    @Test
    public void testGetBlock() throws ClientException {
        Block block = client.getBlock(client.postBlock(setUpTaggedDataBlock()));
        System.out.println(block);
    }

    @Test
    public void testGetBlockRaw() throws ClientException {
        byte[] blockBytes = client.getBlockRaw(client.postBlock(setUpTaggedDataBlock()));
        System.out.println(Base64.getEncoder().encodeToString(blockBytes));
    }

    @Test
    public void testGetBlockMetadata() throws ClientException {
        System.out.println(client.getBlockMetadata(client.postBlock(setUpTaggedDataBlock())));
    }

    @Test
    public void testGetOutput() throws ClientException, InitializeClientException, NoFundsReceivedFromFaucetException {
        Map.Entry<Output, OutputMetadata> r = client.getOutput(setupBasicOutput(generateAddress(client.generateMnemonic())));
        System.out.println(r.getKey());
        System.out.println(r.getValue());
    }

    @Test
    public void testGetOutputMetadata() throws ClientException, InitializeClientException, NoFundsReceivedFromFaucetException {
        OutputMetadata r = client.getOutputMetadata(setupBasicOutput(generateAddress(client.generateMnemonic())));
        System.out.println(r);
    }

    @Test
    public void testGetReceiptsMigratedAt() throws ClientException {
        Receipt[] receipts = client.getReceiptsMigratedAt(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        for (Receipt r : receipts)
            System.out.println(r);
    }

    @Test
    public void testGetReceipts() throws ClientException {
        Receipt[] receipts = client.getReceipts();
        for (Receipt r : receipts)
            System.out.println(r);
    }

    @Test
    public void testGetTreasury() throws ClientException {
        TreasuryResponse r = client.getTreasury();
        System.out.println(r);
    }

    @Test
    public void testGetIncludedBlock() throws ClientException, InitializeClientException, NoFundsReceivedFromFaucetException {
        System.out.println(client.getIncludedBlock(setUpTransactionId(generateAddress(client.generateMnemonic()))));
    }

    @Test
    public void testGetMilestoneById() throws ClientException {
        MilestoneId milestoneId = new MilestoneId(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("milestoneId").getAsString());
        MilestonePayload r = client.getMilestoneById(milestoneId);
        System.out.println(r);
    }

    @Test
    public void testGetMilestoneByIndex() throws ClientException {
        MilestonePayload r = client.getMilestoneByIndex(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        System.out.println(r);
    }

    @Test
    public void testGetMilestoneByIdRaw() throws ClientException {
        MilestoneId milestoneId = new MilestoneId(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("milestoneId").getAsString());
        byte[] milestoneBytes = client.getMilestoneByIdRaw(milestoneId);
        System.out.println(Base64.getEncoder().encodeToString(milestoneBytes));
    }

    @Test
    public void testGetMilestoneByIndexRaw() throws ClientException {
        byte[] milestoneBytes = client.getMilestoneByIndexRaw(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        System.out.println(Base64.getEncoder().encodeToString(milestoneBytes));
    }

    @Test
    public void testGetUtxoChangesId() throws ClientException {
        MilestoneId milestoneId = new MilestoneId(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("milestoneId").getAsString());
        UtxoChangesResponse r = client.getUtxoChangesById(milestoneId);
        for (OutputId consumed : r.getConsumedOutputs())
            System.out.println(consumed);
        for (OutputId created : r.getCreatedOutputs())
            System.out.println(created);
    }

    @Test
    public void testGetUtxoChangesIndex() throws ClientException {
        UtxoChangesResponse r = client.getUtxoChangesByIndex(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("latestMilestone").getAsJsonObject().get("index").getAsInt());
        for (OutputId consumed : r.getConsumedOutputs())
            System.out.println(consumed);
        for (OutputId created : r.getCreatedOutputs())
            System.out.println(created);
    }

    @Test
    @Disabled
    public void testGetPeers() throws ClientException {
        for (Peer peer : client.getPeers())
            System.out.println(peer);
    }

}