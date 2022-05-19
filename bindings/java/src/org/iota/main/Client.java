package org.iota.main;

import org.iota.main.apis.HighLevelApi;
import org.iota.main.apis.NodeCoreApi;
import org.iota.main.apis.NodeIndexerApi;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.SecretManager;

public class Client {

    private NodeCoreApi nodeCoreApi;
    private NodeIndexerApi nodeIndexerApi;
    private HighLevelApi highLevelApi;

    public Client(ClientConfig config) {
        nodeCoreApi = new NodeCoreApi(config);
        nodeIndexerApi = new NodeIndexerApi(config);
        highLevelApi = new HighLevelApi(config);
    }

    // Node Core APIs

    public String getHealth(String nodeUrl) {
        return nodeCoreApi.getHealth(nodeUrl);
    }

    public String getNodeInfo() {
        return nodeCoreApi.getNodeInfo();
    }

    public String getTips() {
        return nodeCoreApi.getTips();
    }

    public String getMessage(String messageId) {
        return nodeCoreApi.getMessage(messageId);
    }

    public String getMessageRaw(String messageId) {
        return nodeCoreApi.getMessageRaw(messageId);
    }

    public String getMessageMetadata(String messageId) {
        return nodeCoreApi.getMessageMetadata(messageId);
    }

    public String getMessageChildren(String messageId) {
        return nodeCoreApi.getMessageChildren(messageId);
    }

    public String getOutput(String outputId) {
        return nodeCoreApi.getOutput(outputId);
    }

    public String getReceiptsMigratedAt(int milestoneIndex) {
        return nodeCoreApi.getReceiptsMigratedAt(milestoneIndex);
    }

    public String getReceipts() {
        return nodeCoreApi.getReceipts();
    }

    public String getTreasury() {
        return nodeCoreApi.getTreasury();
    }

    public String getIncludedMessage(String transactionId) {
        return nodeCoreApi.getIncludedMessage(transactionId);
    }

    public String getMilestoneById(String milestoneId) {
        return nodeCoreApi.getMilestoneById(milestoneId);
    }

    public String getMilestoneByIndex(int milestoneIndex) {
        return nodeCoreApi.getMilestoneByIndex(milestoneIndex);
    }

    public String getMilestoneByIdRaw(String milestoneId) {
        return nodeCoreApi.getMilestoneByIdRaw(milestoneId);
    }

    public String getUtxoChangesById(String milestoneId) {
        return nodeCoreApi.getMilestoneById(milestoneId);
    }

    public String getUtxoChangesByIndex(int milestoneIndex) {
        return nodeCoreApi.getMilestoneByIndex(milestoneIndex);
    }

    public String getPeers() {
        return nodeCoreApi.getPeers();
    }

    // Node Indexer APIs

    public String getBasicOutputIds(NodeIndexerApi.QueryParams params) {
        return nodeIndexerApi.getBasicOutputIds(params);
    }

    public String getAliasOutputIds(NodeIndexerApi.QueryParams params) {
        return nodeIndexerApi.getAliasOutputIds(params);
    }

    public String getAliasOutputId(String aliasId) {
        return nodeIndexerApi.getAliasOutputId(aliasId);
    }

    public String getNftOutputIds(NodeIndexerApi.QueryParams params) {
        return nodeIndexerApi.getNftOutputIds(params);
    }

    public String getNftOutputId(String nftId) {
        return nodeIndexerApi.getNftOutputId(nftId);
    }

    public String getFoundryOutputIds(NodeIndexerApi.QueryParams params) {
        return nodeIndexerApi.getFoundryOutputIds(params);
    }

    public String getFoundryOutputId(String foundryId) {
        return nodeIndexerApi.getFoundryOutputId(foundryId);
    }

    // High level APIs

    public String getOutputs(String[] outputIds) {
        return highLevelApi.getOutputs(outputIds);
    }

    public String tryGetOutputs(String[] outputIds) {
        return highLevelApi.tryGetOutputs(outputIds);
    }

    public String findMessages(String[] messageIds) {
        return highLevelApi.findMessages(messageIds);
    }

    public String retry(String messageId) {
        return highLevelApi.retry(messageId);
    }

    public String retryUntilIncluded(String messageId, int interval, int maxAttempts) {
        return highLevelApi.retryUntilIncluded(messageId, interval, maxAttempts);
    }

    public String consolidateFunds(SecretManager secretManager, int accountIndex, int addressRange) {
        return highLevelApi.consolidateFunds(secretManager, accountIndex, addressRange);
    }

    public String findInputs(String[] addresses, int amount) {
        return highLevelApi.findInputs(addresses, amount);
    }

    public String findOutputs(String[] outputs, String[] addresses) {
        return highLevelApi.findOutputs(outputs, addresses);
    }

    public String reattach(String messageId) {
        return highLevelApi.reattach(messageId);
    }

    public String reattachUnchecked(String messageId) {
        return highLevelApi.reattachUnchecked(messageId);
    }

    public String promote(String messageId) {
        return highLevelApi.promote(messageId);
    }

    public String promoteUnchecked(String messageId) {
        return highLevelApi.promoteUnchecked(messageId);
    }

}

