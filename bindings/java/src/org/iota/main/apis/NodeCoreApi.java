package org.iota.main.apis;

import org.iota.main.types.ClientConfig;

public class NodeCoreApi extends BaseApi {

    public NodeCoreApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String getHealth(String nodeUrl) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetHealth", "{\"url\":\"" + nodeUrl + "\"}"));
    }

    public String getNodeInfo() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetInfo"));
    }

    public String getTips() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTips"));
    }

    public String getBlock(String blockId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlock", "{\"blockId\":\"" + blockId + "\"}"));
    }

    public String getBlockRaw(String blockId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockRaw", "{\"blockId\":\"" + blockId + "\"}"));
    }

    public String getBlockMetadata(String blockId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockMetadata", "{\"blockId\":\"" + blockId + "\"}"));
    }

    public String getBlockChildren(String blockId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBlockChildren", "{\"blockId\":\"" + blockId + "\"}"));
    }

    public String getOutput(String outputId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutput", "{\"outputId\":\"" + outputId + "\"}"));
    }

    public String getOutputMetadata(String outputId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputMetadata", "{\"outputId\":\"" + outputId + "\"}"));
    }

    public String getReceiptsMigratedAt(int milestoneIndex) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetReceiptsMigratedAt", "{\"milestoneIndex\":" + milestoneIndex + "}"));
    }

    public String getReceipts() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetReceipts"));
    }

    public String getTreasury() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTreasury"));
    }

    public String getIncludedBlock(String transactionId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetIncludedblock", "{\"transactionId\":\"" + transactionId + "\"}"));
    }

    public String getMilestoneById(String milestoneId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneById", "{\"milestoneId\":\"" + milestoneId + "\"}"));
    }

    public String getMilestoneByIndex(int milestoneIndex) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIndex", "{\"index\":" + milestoneIndex + "}"));
    }

    public String getMilestoneByIdRaw(String milestoneId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMilestoneByIdRaw", "{\"milestoneId\":" + milestoneId + "}"));
    }

    public String getUtxoChangesById(String milestoneId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetUtxoChangesById", "{\"milestoneId\":" + milestoneId + "}"));
    }

    public String getUtxoChangesByIndex(int milestoneIndex) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetUtxoChangesByIndex", "{\"index\":" + milestoneIndex + "}"));
    }

    public String getPeers() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetPeers"));
    }
}
