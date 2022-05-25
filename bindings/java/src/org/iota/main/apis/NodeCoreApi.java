package org.iota.main.apis;

import org.iota.main.types.Block;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.SuccessResponse;
import org.iota.main.types.responses.*;

public class NodeCoreApi extends BaseApi {

    public NodeCoreApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public GetHealthResponse getHealth(String nodeUrl) throws ClientException {
        return (GetHealthResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetHealth", "{\"url\":\"" + nodeUrl + "\"}"));
    }

    public GetNodeInfoResponse getNodeInfo() throws ClientException {
        return (GetNodeInfoResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetInfo"));
    }

    public GetTipsResponse getTips() throws ClientException {
        return (GetTipsResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTips"));
    }

    public PostBlockResponse postBlock(Block block) throws ClientException {
        return (PostBlockResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PostBlock", "{\"block\":" + block.toString() + "}"));
    }

    public GetBlockResponse getBlock(String blockId) throws ClientException {
        return (GetBlockResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlock", "{\"blockId\":\"" + blockId + "\"}"));
    }

    public GetBlockRawResponse getBlockRaw(String blockId) throws ClientException {
        return (GetBlockRawResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockRaw", "{\"blockId\":\"" + blockId + "\"}"));
    }

    public GetBlockMetadataResponse getBlockMetadata(String blockId) throws ClientException {
        return (GetBlockMetadataResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockMetadata", "{\"blockId\":\"" + blockId + "\"}"));
    }

    public GetBlockChildrenResponse getBlockChildren(String blockId) throws ClientException {
        return (GetBlockChildrenResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockChildren", "{\"blockId\":\"" + blockId + "\"}"));
    }

    public SuccessResponse getOutput(String outputId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutput", "{\"outputId\":\"" + outputId + "\"}"));
    }

    public SuccessResponse getOutputMetadata(String outputId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputMetadata", "{\"outputId\":\"" + outputId + "\"}"));
    }

    public SuccessResponse getReceiptsMigratedAt(int milestoneIndex) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetReceiptsMigratedAt", "{\"milestoneIndex\":" + milestoneIndex + "}"));
    }

    public SuccessResponse getReceipts() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetReceipts"));
    }

    public SuccessResponse getTreasury() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTreasury"));
    }

    public SuccessResponse getIncludedBlock(String transactionId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetIncludedblock", "{\"transactionId\":\"" + transactionId + "\"}"));
    }

    public SuccessResponse getMilestoneById(String milestoneId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneById", "{\"milestoneId\":\"" + milestoneId + "\"}"));
    }

    public SuccessResponse getMilestoneByIndex(int milestoneIndex) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIndex", "{\"index\":" + milestoneIndex + "}"));
    }

    public SuccessResponse getMilestoneByIdRaw(String milestoneId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIdRaw", "{\"milestoneId\":" + milestoneId + "}"));
    }

    public SuccessResponse getUtxoChangesById(String milestoneId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetUtxoChangesById", "{\"milestoneId\":" + milestoneId + "}"));
    }

    public SuccessResponse getUtxoChangesByIndex(int milestoneIndex) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetUtxoChangesByIndex", "{\"index\":" + milestoneIndex + "}"));
    }

    public SuccessResponse getPeers() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetPeers"));
    }
}
