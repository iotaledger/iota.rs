// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.*;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.MilestoneId;
import org.iota.types.ids.OutputId;
import org.iota.types.ids.TransactionId;
import org.iota.types.responses.NodeInfoResponse;
import org.iota.types.responses.TreasuryResponse;
import org.iota.types.responses.UtxoChangesResponse;

import java.util.Map;

public class NodeCoreApi {

    private NativeApi nativeApi;

    public NodeCoreApi(NativeApi nativeApi) throws InitializeClientException {
        this.nativeApi = nativeApi;
    }

    public boolean getHealth(String nodeUrl) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("url", nodeUrl);

        Boolean responsePayload = nativeApi.sendCommand(new ClientCommand("getHealth", o)).getAsBoolean();

        return responsePayload;
    }

    public NodeInfoResponse getNodeInfo() throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getInfo"));
        return new NodeInfoResponse(responsePayload);
    }

    public BlockId[] getTips() throws ClientException {
        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("getTips"));

        BlockId[] blockIds = new BlockId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            blockIds[i] = new BlockId(responsePayload.get(i).getAsString());

        return blockIds;
    }

    public BlockId postBlock(Block block) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("block", block.toJson());

        String responsePayload = nativeApi.sendCommand(new ClientCommand("postBlock", o)).getAsString();
        return new BlockId(responsePayload);
    }

    public BlockId postBlockRaw(byte[] blockBytes) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("blockBytes", JsonUtils.toJson(blockBytes));

        String responsePayload = nativeApi.sendCommand(new ClientCommand("postBlockRaw", o)).getAsString();
        return new BlockId(responsePayload);
    }

    public Block getBlock(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getBlock", o));
        return new Block(responsePayload);
    }

    public byte[] getBlockRaw(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("getBlockRaw", o));

        byte[] blockBytes = new byte[responsePayload.size()];

        for (int i = 0; i < responsePayload.size(); i++) {
            blockBytes[i] = responsePayload.get(i).getAsByte();
        }

        return blockBytes;
    }

    public BlockMetadata getBlockMetadata(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getBlockMetadata", o));

        return new BlockMetadata(responsePayload);
    }

    public Map.Entry<Output, OutputMetadata> getOutput(OutputId outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId.toString());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getOutput", o));

        Output output = new Output(responsePayload.get("output").getAsJsonObject());
        OutputMetadata metadata = new OutputMetadata(responsePayload.getAsJsonObject().get("metadata").getAsJsonObject());

        return Map.entry(output, metadata);
    }

    public OutputMetadata getOutputMetadata(OutputId outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId.toString());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getOutputMetadata", o));

        return new OutputMetadata(responsePayload);
    }

    public Receipt[] getReceiptsMigratedAt(int milestoneIndex) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneIndex", milestoneIndex);

        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("getReceiptsMigratedAt", o));

        Receipt[] receipts = new Receipt[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            receipts[i] = new Receipt(responsePayload.get(i).getAsJsonObject());
        }

        return receipts;
    }

    public Receipt[] getReceipts() throws ClientException {
        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("getReceipts"));

        Receipt[] receipts = new Receipt[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            receipts[i] = new Receipt(responsePayload.get(i).getAsJsonObject());
        }

        return receipts;
    }

    public TreasuryResponse getTreasury() throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getTreasury"));

        return new TreasuryResponse(responsePayload);
    }

    public Block getIncludedBlock(TransactionId transactionId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("transactionId", transactionId.toString());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getIncludedBlock", o));

        return new Block(responsePayload);
    }

    public BlockMetadata getIncludedBlockMetadata(TransactionId transactionId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("transactionId", transactionId.toString());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getIncludedBlockMetadata", o));

        return new BlockMetadata(responsePayload);
    }

    public MilestonePayload getMilestoneById(MilestoneId milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId.toString());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getMilestoneById", o));

        return new MilestonePayload(responsePayload);
    }

    public MilestonePayload getMilestoneByIndex(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getMilestoneByIndex", o));

        return new MilestonePayload(responsePayload);
    }

    public byte[] getMilestoneByIdRaw(MilestoneId milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId.toString());

        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("getMilestoneByIdRaw", o));

        byte[] milestoneBytes = new byte[responsePayload.size()];

        for (int i = 0; i < responsePayload.size(); i++) {
            milestoneBytes[i] = responsePayload.get(i).getAsByte();
        }

        return milestoneBytes;
    }

    public byte[] getMilestoneByIndexRaw(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);

        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("getMilestoneByIndexRaw", o));

        byte[] milestoneBytes = new byte[responsePayload.size()];

        for (int i = 0; i < responsePayload.size(); i++) {
            milestoneBytes[i] = responsePayload.get(i).getAsByte();
        }

        return milestoneBytes;
    }

    public UtxoChangesResponse getUtxoChangesById(MilestoneId milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId.toString());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getUtxoChangesById", o));

        return new UtxoChangesResponse(responsePayload);
    }

    public UtxoChangesResponse getUtxoChangesByIndex(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getUtxoChangesByIndex", o));

        return new UtxoChangesResponse(responsePayload);
    }

    public Peer[] getPeers() throws ClientException {
        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("getPeers"));

        Peer[] peers = new Peer[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            peers[i] = new Peer(responsePayload.get(i).getAsJsonObject());
        }

        return peers;
    }

}
