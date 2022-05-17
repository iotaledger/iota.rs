package org.iota;

import org.iota.apis.NodeApi;

public class Client {

    private NodeApi nodeApi;

    public Client(ClientConfig config) {
        nodeApi = new NodeApi(config);
    }

    public String getHealth(String nodeUrl) {
        return nodeApi.getHealth(nodeUrl);
    }

    public String getNodeInfo() {
        return nodeApi.getNodeInfo();
    }

    public String getTips() {
        return nodeApi.getTips();
    }

    public String getMessage(String messageId) {
        return nodeApi.getMessage(messageId);
    }

    public String getMessageRaw(String messageId) {
        return nodeApi.getMessageRaw(messageId);
    }

    public String getMessageMetadata(String messageId) {
        return nodeApi.getMessageMetadata(messageId);
    }

    public String getMessageChildren(String messageId) {
        return nodeApi.getMessageChildren(messageId);
    }

    public String getOutput(String outputId) {
        return nodeApi.getOutput(outputId);
    }

    public String getReceiptsMigratedAt(int milestoneIndex) {
        return nodeApi.getReceiptsMigratedAt(milestoneIndex);
    }

    public String getReceipts() {
        return nodeApi.getReceipts();
    }

    public String getTreasury() {
        return nodeApi.getTreasury();
    }

    public String getIncludedMessage(String transactionId) {
        return nodeApi.getIncludedMessage(transactionId);
    }

    public String getMilestoneById(String milestoneId) {
        return nodeApi.getMilestoneById(milestoneId);
    }

    public String getMilestoneByIndex(int milestoneIndex) {
        return nodeApi.getMilestoneByIndex(milestoneIndex);
    }

    public String getMilestoneByIdRaw(String milestoneId) {
        return nodeApi.getMilestoneByIdRaw(milestoneId);
    }


}