package org.iota.apis;

import org.iota.*;

public class NodeApi extends BaseApi {

    public NodeApi(ClientConfig config) {
        super(config);
    }

    public String getHealth(String nodeUrl) {
        return RustApi.call(super.config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetHealth\", \"data\": { \"url\": \"" + nodeUrl + "\" }}"));
    }

    public String getNodeInfo() {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetInfo\" }"));
    }

    public String getTips() {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetTips\" }"));
    }

    public String getMessage(String messageId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMessage\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getMessageRaw(String messageId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMessageRaw\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getMessageMetadata(String messageId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMessageMetadata\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getMessageChildren(String messageId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMessageChildren\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getOutput(String outputId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetOutput\", \"data\": { \"outputId\": \"" + outputId + "\" }}"));
    }

    public String getReceiptsMigratedAt(int milestoneIndex) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetReceiptsMigratedAt\", \"data\": { \"milestoneIndex\": " + milestoneIndex + " }}"));
    }

    public String getReceipts() {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetReceipts\" }"));
    }

    public String getTreasury() {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetTreasury\" }"));
    }

    public String getIncludedMessage(String transactionId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetIncludedMessage\", \"data\": { \"transactionId\": \"" + transactionId + "\" }}"));
    }

    public String getMilestoneById(String milestoneId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMilestoneById\", \"data\": { \"milestoneId\": \"" + milestoneId + "\" }}"));
    }

    public String getMilestoneByIndex(int milestoneIndex) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMilestoneByIndex\", \"data\": { \"index\": " + milestoneIndex + " }}"));
    }

    public String getMilestoneByIdRaw(String milestoneId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMilestoneByIdRaw\", \"data\": { \"milestoneId\": " + milestoneId + " }}"));
    }

}
