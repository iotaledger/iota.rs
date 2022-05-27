package org.iota.main.apis;

import com.google.gson.JsonObject;
import org.iota.main.types.Block;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.responses.*;

public class NodeCoreApi extends BaseApi {

    public NodeCoreApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public HealthResponse getHealth(String nodeUrl) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("url", nodeUrl);
        return (HealthResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetHealth", o.toString()));
    }

    public NodeInfoResponse getNodeInfo() throws ClientException {
        return (NodeInfoResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetInfo"));
    }

    public TipsResponse getTips() throws ClientException {
        return (TipsResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTips"));
    }

    public PostBlockResponse postBlock(Block block) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("block", block.getJson());
        return (PostBlockResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PostBlock", o.toString()));
    }

    public BlockResponse getBlock(String blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId);
        return (BlockResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlock", o.toString()));
    }

    public BlockRawResponse getBlockRaw(String blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId);
        return (BlockRawResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockRaw", o.toString()));
    }

    public BlockMetadataResponse getBlockMetadata(String blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId);
        return (BlockMetadataResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockMetadata", o.toString()));
    }

    public BlockChildrenResponse getBlockChildren(String blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId);
        return (BlockChildrenResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockChildren", o.toString()));
    }

    public OutputResponse getOutput(String outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId);
        return (OutputResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutput", o.toString()));
    }

    public OutputMetadataResponse getOutputMetadata(String outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId);
        return (OutputMetadataResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputMetadata", o.toString()));
    }

    public ReceiptsMigratedAtResponse getReceiptsMigratedAt(int milestoneIndex) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneIndex", milestoneIndex);
        return (ReceiptsMigratedAtResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetReceiptsMigratedAt", o.toString()));
    }

    public ReceiptsResponse getReceipts() throws ClientException {
        return (ReceiptsResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetReceipts"));
    }

    public TreasuryResponse getTreasury() throws ClientException {
        return (TreasuryResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTreasury"));
    }

    public BlockResponse getIncludedBlock(String transactionId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("transactionId", transactionId);
        return (BlockResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetIncludedBlock", o.toString()));
    }

    public MilestoneResponse getMilestoneById(String milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId);
        return (MilestoneResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneById", o.toString()));
    }

    public MilestoneResponse getMilestoneByIndex(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);
        return (MilestoneResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIndex", o.toString()));
    }

    public MilestoneRawResponse getMilestoneByIdRaw(String milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId);
        return (MilestoneRawResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIdRaw", o.toString()));
    }

    public MilestoneRawResponse getMilestoneByIndexRaw(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);
        return (MilestoneRawResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIndexRaw", o.toString()));
    }

    public UtxoChangesResponse getUtxoChangesById(String milestoneId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("milestoneId", milestoneId);
        return (UtxoChangesResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetUtxoChangesById", o.toString()));
    }

    public UtxoChangesResponse getUtxoChangesByIndex(int index) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("index", index);
        return (UtxoChangesResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetUtxoChangesByIndex", o.toString()));
    }

    public PeersResponse getPeers() throws ClientException {
        return (PeersResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetPeers"));
    }
}
