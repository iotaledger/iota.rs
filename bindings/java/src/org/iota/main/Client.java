package org.iota.main;

import org.iota.main.apis.*;
import org.iota.main.types.*;

public class Client {

    private NodeCoreApi nodeCoreApi;
    private NodeIndexerApi nodeIndexerApi;
    private HighLevelApi highLevelApi;
    private UtilsApi utilsApi;
    private MiscellaneousApi miscellaneousApi;

    public Client(ClientConfig config) {
        nodeCoreApi = new NodeCoreApi(config);
        nodeIndexerApi = new NodeIndexerApi(config);
        highLevelApi = new HighLevelApi(config);
        utilsApi = new UtilsApi(config);
        miscellaneousApi = new MiscellaneousApi(config);
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

    // Utils APIs

    public String bech32ToHex(String bech32) {
        return utilsApi.bech32ToHex(bech32);
    }

    public String hexToBech32(String hex, String bech32) {
        return utilsApi.hexToBech32(hex, bech32);
    }

    public String hexPublicKeyToBech32Address(String hex, String bech32) {
        return utilsApi.hexPublicKeyToBech32Address(hex, bech32);
    }

    public String parseBech32Address(String address) {
        return utilsApi.parseBech32Address(address);
    }

    public String isAddressValid(String address) {
        return utilsApi.isAddressValid(address);
    }

    public String generateMnemonic() {
        return utilsApi.generateMnemonic();
    }

    public String mnemonicToHexSeed(String mnemonic) {
        return utilsApi.mnemonicToHexSeed(mnemonic);
    }

    public String messageId(String message) {
        return utilsApi.messageId(message);
    }

    // Miscellaneous APIs

    public String generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) {
        return miscellaneousApi.generateAddresses(secretManager, generateAddressesOptions);
    }

    public String generateMessage(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) {
        return miscellaneousApi.generateMessage(secretManager, generateAddressesOptions);
    }


    public String getNode() {
        return miscellaneousApi.getNode();
    }

    public String getNetworkInfo() {
        return miscellaneousApi.getNetworkInfo();
    }

    public String getNetworkId() {
        return miscellaneousApi.getNetworkId();
    }

    public String getBech32Hrp() {
        return miscellaneousApi.getBech32Hrp();
    }

    public String getMinPoWScore() {
        return miscellaneousApi.getMinPoWScore();
    }

    public String getTipsInterval() {
        return miscellaneousApi.getTipsInterval();
    }

    public String getLocalPoW() {
        return miscellaneousApi.getLocalPoW();
    }

    public String getFallbackToLocalPoW() {
        return miscellaneousApi.getFallbackToLocalPoW();
    }

    public String getUsyncedNodes() {
        return miscellaneousApi.getUnsyncedNodes();
    }

    public String prepareTransaction(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) {
        return miscellaneousApi.prepareTransaction(secretManager, generateAddressesOptions);
    }

    public String signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) {
        return miscellaneousApi.signTransaction(secretManager, preparedTransactionData);
    }

    public String storeMnemonic(SecretManager secretManager, String mnemonic) {
        return miscellaneousApi.storeMnemonic(secretManager, mnemonic);
    }

    public String submitPayload(Payload payload) {
        return miscellaneousApi.submitPayload(payload);
    }

}

