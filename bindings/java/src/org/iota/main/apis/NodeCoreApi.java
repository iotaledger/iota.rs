package org.iota.main.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.main.types.*;
import org.iota.main.types.responses.ClientResponse;
import org.iota.main.types.responses.node_core_api.NodeInfoResponse;
import org.iota.main.types.responses.node_core_api.TreasuryResponse;
import org.iota.main.types.responses.node_core_api.UtxoChangesResponse;

import java.util.Map;

public class NodeCoreApi extends BaseApi {

    public NodeCoreApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public boolean getHealth(String nodeUrl) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("url", nodeUrl);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetHealth", o.toString()));
        Boolean responsePayload = response.getPayload().getAsBoolean();

        return responsePayload;
    }

    public NodeInfoResponse getNodeInfo() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetInfo"));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new NodeInfoResponse(responsePayload);
    }

    public BlockId[] getTips() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTips"));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        BlockId[] blockIds = new BlockId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            blockIds[i] = new BlockId(responsePayload.get(i).getAsString());

        return blockIds;
    }

    public BlockId postBlock(Block block) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("block", block.getJson());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PostBlock", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return new BlockId(responsePayload);
    }

    public Block getBlock(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlock", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new Block(responsePayload);
    }

    public byte[] getBlockRaw(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockRaw", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        byte[] blockBytes = new byte[responsePayload.size()];

        for (int i = 0; i < responsePayload.size(); i++) {
            blockBytes[i] = responsePayload.get(i).getAsByte();
        }

        return blockBytes;
    }

    public BlockMetadata getBlockMetadata(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockMetadata", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new BlockMetadata(responsePayload);
    }

    public Map.Entry<Output, OutputMetadata> getOutputWithMetadata(OutputId outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutput", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        Output output = new Output(responsePayload.get("output").getAsJsonObject());
        OutputMetadata metadata = new OutputMetadata(responsePayload.getAsJsonObject().get("metadata").getAsJsonObject());

        return Map.entry(output, metadata);
    }

    public OutputMetadata getOutputMetadata(OutputId outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputMetadata", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new OutputMetadata(responsePayload);
    }

    public Receipt[] getReceiptsMigratedAt(int milestoneIndex) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneIndex", milestoneIndex);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetReceiptsMigratedAt", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        Receipt[] receipts = new Receipt[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            receipts[i] = new Receipt(responsePayload.get(i).getAsJsonObject());
        }

        return receipts;
    }

    public Receipt[] getReceipts() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetReceipts"));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        Receipt[] receipts = new Receipt[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            receipts[i] = new Receipt(responsePayload.get(i).getAsJsonObject());
        }

        return receipts;
    }

    public TreasuryResponse getTreasury() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTreasury"));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new TreasuryResponse(responsePayload);
    }

    public Block getIncludedBlock(TransactionId transactionId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("transactionId", transactionId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetIncludedBlock", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new Block(responsePayload);
    }

    public Milestone getMilestoneById(MilestoneId milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneById", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new Milestone(responsePayload);
    }

    public Milestone getMilestoneByIndex(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIndex", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new Milestone(responsePayload);
    }

    public byte[] getMilestoneByIdRaw(MilestoneId milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIdRaw", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        byte[] milestoneBytes = new byte[responsePayload.size()];

        for (int i = 0; i < responsePayload.size(); i++) {
            milestoneBytes[i] = responsePayload.get(i).getAsByte();
        }

        return milestoneBytes;
    }

    public byte[] getMilestoneByIndexRaw(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIndexRaw", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        byte[] milestoneBytes = new byte[responsePayload.size()];

        for (int i = 0; i < responsePayload.size(); i++) {
            milestoneBytes[i] = responsePayload.get(i).getAsByte();
        }

        return milestoneBytes;
    }

    public UtxoChangesResponse getUtxoChangesById(MilestoneId milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetUtxoChangesById", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new UtxoChangesResponse(responsePayload);
    }

    public UtxoChangesResponse getUtxoChangesByIndex(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetUtxoChangesByIndex", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new UtxoChangesResponse(responsePayload);
    }

    public Peer[] getPeers() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetPeers"));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        Peer[] peers = new Peer[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            peers[i] = new Peer(responsePayload.get(i).getAsJsonObject());
        }

        return peers;
    }

}
