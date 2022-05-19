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

    public String getMessage(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMessage", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public String getMessageRaw(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMessageRaw", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public String getMessageMetadata(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMessageMetadata", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public String getMessageChildren(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMessageChildren", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public String getOutput(String outputId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutput", "{\"outputId\":\"" + outputId + "\"}"));
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

    public String getIncludedMessage(String transactionId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetIncludedMessage", "{\"transactionId\":\"" + transactionId + "\"}"));
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
