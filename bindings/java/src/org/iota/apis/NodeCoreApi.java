package org.iota.apis;

import org.iota.*;

public class NodeCoreApi extends BaseApi {

    public NodeCoreApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String getHealth(String nodeUrl) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetHealth\", \"data\": { \"url\": \"" + nodeUrl + "\" }}"));
    }

    public String getNodeInfo() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetInfo\" }"));
    }

    public String getTips() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetTips\" }"));
    }

    public String getMessage(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetMessage\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getMessageRaw(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetMessageRaw\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getMessageMetadata(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetMessageMetadata\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getMessageChildren(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetMessageChildren\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getOutput(String outputId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetOutput\", \"data\": { \"outputId\": \"" + outputId + "\" }}"));
    }

    public String getReceiptsMigratedAt(int milestoneIndex) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetReceiptsMigratedAt\", \"data\": { \"milestoneIndex\": " + milestoneIndex + " }}"));
    }

    public String getReceipts() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetReceipts\" }"));
    }

    public String getTreasury() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetTreasury\" }"));
    }

    public String getIncludedMessage(String transactionId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetIncludedMessage\", \"data\": { \"transactionId\": \"" + transactionId + "\" }}"));
    }

    public String getMilestoneById(String milestoneId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetMilestoneById\", \"data\": { \"milestoneId\": \"" + milestoneId + "\" }}"));
    }

    public String getMilestoneByIndex(int milestoneIndex) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetMilestoneByIndex\", \"data\": { \"index\": " + milestoneIndex + " }}"));
    }

    public String getMilestoneByIdRaw(String milestoneId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetMilestoneByIdRaw\", \"data\": { \"milestoneId\": " + milestoneId + " }}"));
    }

    public String getUtxoChangesById(String milestoneId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetUtxoChangesById\", \"data\": { \"milestoneId\": " + milestoneId + " }}"));
    }

    public String getUtxoChangesByIndex(int milestoneIndex) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetUtxoChangesByIndex\", \"data\": { \"index\": " + milestoneIndex + " }}"));
    }

    public String getPeers() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "{ \"name\": \"GetPeers\" }"));
    }

}
