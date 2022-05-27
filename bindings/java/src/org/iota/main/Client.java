package org.iota.main;

import org.iota.main.apis.*;
import org.iota.main.types.*;
import org.iota.main.types.responses.*;
import org.iota.main.types.secret.GenerateAddressesOptions;
import org.iota.main.types.secret.GenerateBlockOptions;
import org.iota.main.types.secret.SecretManager;

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

    public HealthResponse getHealth(String nodeUrl) throws ClientException {
        return nodeCoreApi.getHealth(nodeUrl);
    }

    public NodeInfoResponse getNodeInfo() throws ClientException {
        return nodeCoreApi.getNodeInfo();
    }

    public TipsResponse getTips() throws ClientException {
        return nodeCoreApi.getTips();
    }

    public PostBlockResponse postBlock(Block block) throws ClientException {
        return nodeCoreApi.postBlock(block);
    }

    public BlockResponse getBlock(String blockId) throws ClientException {
        return nodeCoreApi.getBlock(blockId);
    }

    public BlockRawResponse getBlockRaw(String blockId) throws ClientException {
        return nodeCoreApi.getBlockRaw(blockId);
    }

    public BlockMetadataResponse getBlockMetadata(String blockId) throws ClientException {
        return nodeCoreApi.getBlockMetadata(blockId);
    }

    public BlockChildrenResponse getBlockChildren(String blockId) throws ClientException {
        return nodeCoreApi.getBlockChildren(blockId);
    }

    public OutputResponse getOutput(String outputId) throws ClientException {
        return nodeCoreApi.getOutput(outputId);
    }

    public OutputMetadataResponse getOutputMetadata(String outputId) throws ClientException {
        return nodeCoreApi.getOutputMetadata(outputId);
    }

    public ReceiptsMigratedAtResponse getReceiptsMigratedAt(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getReceiptsMigratedAt(milestoneIndex);
    }

    public SuccessResponse getReceipts() throws ClientException {
        return nodeCoreApi.getReceipts();
    }

    public SuccessResponse getTreasury() throws ClientException {
        return nodeCoreApi.getTreasury();
    }

    public SuccessResponse getIncludedBlock(String transactionId) throws ClientException {
        return nodeCoreApi.getIncludedBlock(transactionId);
    }

    public SuccessResponse getMilestoneById(String milestoneId) throws ClientException {
        return nodeCoreApi.getMilestoneById(milestoneId);
    }

    public SuccessResponse getMilestoneByIndex(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getMilestoneByIndex(milestoneIndex);
    }

    public SuccessResponse getMilestoneByIdRaw(String milestoneId) throws ClientException {
        return nodeCoreApi.getMilestoneByIdRaw(milestoneId);
    }

    public SuccessResponse getUtxoChangesById(String milestoneId) throws ClientException {
        return nodeCoreApi.getMilestoneById(milestoneId);
    }

    public SuccessResponse getUtxoChangesByIndex(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getMilestoneByIndex(milestoneIndex);
    }

    public SuccessResponse getPeers() throws ClientException {
        return nodeCoreApi.getPeers();
    }

    // Node Indexer APIs

    public SuccessResponse getBasicOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getBasicOutputIds(params);
    }

    public SuccessResponse getAliasOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getAliasOutputIds(params);
    }

    public SuccessResponse getAliasOutputId(String aliasId) throws ClientException {
        return nodeIndexerApi.getAliasOutputId(aliasId);
    }

    public SuccessResponse getNftOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getNftOutputIds(params);
    }

    public SuccessResponse getNftOutputId(String nftId) throws ClientException {
        return nodeIndexerApi.getNftOutputId(nftId);
    }

    public SuccessResponse getFoundryOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getFoundryOutputIds(params);
    }

    public SuccessResponse getFoundryOutputId(String foundryId) throws ClientException {
        return nodeIndexerApi.getFoundryOutputId(foundryId);
    }

    // High level APIs

    public SuccessResponse getOutputs(String[] outputIds) throws ClientException {
        return highLevelApi.getOutputs(outputIds);
    }

    public SuccessResponse tryGetOutputs(String[] outputIds) throws ClientException {
        return highLevelApi.tryGetOutputs(outputIds);
    }

    public SuccessResponse findMessages(String[] messageIds) throws ClientException {
        return highLevelApi.findMessages(messageIds);
    }

    public SuccessResponse retry(String messageId) throws ClientException {
        return highLevelApi.retry(messageId);
    }

    public SuccessResponse retryUntilIncluded(String messageId, int interval, int maxAttempts) throws ClientException {
        return highLevelApi.retryUntilIncluded(messageId, interval, maxAttempts);
    }

    public SuccessResponse consolidateFunds(SecretManager secretManager, int accountIndex, int addressRange) throws ClientException {
        return highLevelApi.consolidateFunds(secretManager, accountIndex, addressRange);
    }

    public SuccessResponse findInputs(String[] addresses, int amount) throws ClientException {
        return highLevelApi.findInputs(addresses, amount);
    }

    public SuccessResponse findOutputs(String[] outputs, String[] addresses) throws ClientException {
        return highLevelApi.findOutputs(outputs, addresses);
    }

    public SuccessResponse reattach(String messageId) throws ClientException {
        return highLevelApi.reattach(messageId);
    }

    public SuccessResponse reattachUnchecked(String messageId) throws ClientException {
        return highLevelApi.reattachUnchecked(messageId);
    }

    public SuccessResponse promote(String messageId) throws ClientException {
        return highLevelApi.promote(messageId);
    }

    public SuccessResponse promoteUnchecked(String messageId) throws ClientException {
        return highLevelApi.promoteUnchecked(messageId);
    }

    // Utils APIs

    public Bech32ToHexResponse bech32ToHex(String bech32) throws ClientException {
        return utilsApi.bech32ToHex(bech32);
    }

    public SuccessResponse hexToBech32(String hex, String bech32) throws ClientException {
        return utilsApi.hexToBech32(hex, bech32);
    }

    public SuccessResponse hexPublicKeyToBech32Address(String hex, String bech32) throws ClientException {
        return utilsApi.hexPublicKeyToBech32Address(hex, bech32);
    }

    public SuccessResponse parseBech32Address(String address) throws ClientException {
        return utilsApi.parseBech32Address(address);
    }

    public SuccessResponse isAddressValid(String address) throws ClientException {
        return utilsApi.isAddressValid(address);
    }

    public SuccessResponse generateMnemonic() throws ClientException {
        return utilsApi.generateMnemonic();
    }

    public SuccessResponse mnemonicToHexSeed(String mnemonic) throws ClientException {
        return utilsApi.mnemonicToHexSeed(mnemonic);
    }

    public SuccessResponse getBlockId(String block) throws ClientException {
        return utilsApi.getBlockId(block);
    }

    public TransactionIdResponse getTransactionId(BlockPayload payload) throws ClientException {
        return utilsApi.getTransactionId(payload);
    }

    // Miscellaneous APIs

    public GenerateAddressesResponse generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        return miscellaneousApi.generateAddresses(secretManager, generateAddressesOptions);
    }

    public BlockResponse generateBlock(SecretManager secretManager, GenerateBlockOptions options) throws ClientException {
        return miscellaneousApi.generateBlock(secretManager, options);
    }


    public SuccessResponse getNode() throws ClientException {
        return miscellaneousApi.getNode();
    }

    public SuccessResponse getNetworkInfo() throws ClientException {
        return miscellaneousApi.getNetworkInfo();
    }

    public SuccessResponse getNetworkId() throws ClientException {
        return miscellaneousApi.getNetworkId();
    }

    public SuccessResponse getBech32Hrp() throws ClientException {
        return miscellaneousApi.getBech32Hrp();
    }

    public SuccessResponse getMinPoWScore() throws ClientException {
        return miscellaneousApi.getMinPoWScore();
    }

    public SuccessResponse getTipsInterval() throws ClientException {
        return miscellaneousApi.getTipsInterval();
    }

    public SuccessResponse getLocalPoW() throws ClientException {
        return miscellaneousApi.getLocalPoW();
    }

    public SuccessResponse getFallbackToLocalPoW() throws ClientException {
        return miscellaneousApi.getFallbackToLocalPoW();
    }

    public SuccessResponse getUsyncedNodes() throws ClientException {
        return miscellaneousApi.getUnsyncedNodes();
    }

    public SuccessResponse prepareTransaction(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        return miscellaneousApi.prepareTransaction(secretManager, generateAddressesOptions);
    }

    public SuccessResponse signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) throws ClientException {
        return miscellaneousApi.signTransaction(secretManager, preparedTransactionData);
    }

    public SuccessResponse storeMnemonic(SecretManager secretManager, String mnemonic) throws ClientException {
        return miscellaneousApi.storeMnemonic(secretManager, mnemonic);
    }

    public BlockResponse submitBlockPayload(BlockPayload payload) throws ClientException {
        return miscellaneousApi.submitBlockPayload(payload);
    }

}

