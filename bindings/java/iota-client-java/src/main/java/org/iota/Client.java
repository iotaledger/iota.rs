package org.iota;

import com.google.gson.JsonObject;
import org.iota.apis.*;
import org.iota.types.*;
import org.iota.types.responses.node_core_api.NodeInfoResponse;
import org.iota.types.responses.node_core_api.TreasuryResponse;
import org.iota.types.responses.node_core_api.UtxoChangesResponse;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.GenerateBlockOptions;
import org.iota.types.secret.Range;
import org.iota.types.secret.SecretManager;

import java.util.LinkedHashMap;
import java.util.Map;

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

    public boolean getHealth(String nodeUrl) throws ClientException {
        return nodeCoreApi.getHealth(nodeUrl);
    }

    public NodeInfoResponse getNodeInfo() throws ClientException {
        return nodeCoreApi.getNodeInfo();
    }

    public BlockId[] getTips() throws ClientException {
        return nodeCoreApi.getTips();
    }

    public BlockId postBlock(Block block) throws ClientException {
        return nodeCoreApi.postBlock(block);
    }

    public Block getBlock(BlockId blockId) throws ClientException {
        return nodeCoreApi.getBlock(blockId);
    }

    public byte[] getBlockRaw(BlockId blockId) throws ClientException {
        return nodeCoreApi.getBlockRaw(blockId);
    }

    public BlockMetadata getBlockMetadata(BlockId blockId) throws ClientException {
        return nodeCoreApi.getBlockMetadata(blockId);
    }

    public Map.Entry<Output, OutputMetadata> getOutputWithMetadata(OutputId outputId) throws ClientException {
        return nodeCoreApi.getOutputWithMetadata(outputId);
    }

    public OutputMetadata getOutputMetadata(OutputId outputId) throws ClientException {
        return nodeCoreApi.getOutputMetadata(outputId);
    }

    public Receipt[] getReceiptsMigratedAt(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getReceiptsMigratedAt(milestoneIndex);
    }

    public Receipt[] getReceipts() throws ClientException {
        return nodeCoreApi.getReceipts();
    }

    public TreasuryResponse getTreasury() throws ClientException {
        return nodeCoreApi.getTreasury();
    }

    public Block getIncludedBlock(TransactionId transactionId) throws ClientException {
        return nodeCoreApi.getIncludedBlock(transactionId);
    }

    public Milestone getMilestoneById(MilestoneId milestoneId) throws ClientException {
        return nodeCoreApi.getMilestoneById(milestoneId);
    }

    public Milestone getMilestoneByIndex(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getMilestoneByIndex(milestoneIndex);
    }

    public byte[] getMilestoneByIdRaw(MilestoneId milestoneId) throws ClientException {
        return nodeCoreApi.getMilestoneByIdRaw(milestoneId);
    }

    public byte[] getMilestoneByIndexRaw(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getMilestoneByIndexRaw(milestoneIndex);
    }

    public UtxoChangesResponse getUtxoChangesById(MilestoneId milestoneId) throws ClientException {
        return nodeCoreApi.getUtxoChangesById(milestoneId);
    }

    public UtxoChangesResponse getUtxoChangesByIndex(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getUtxoChangesByIndex(milestoneIndex);
    }

    public Peer[] getPeers() throws ClientException {
        return nodeCoreApi.getPeers();
    }

    // Node Indexer APIs

    public OutputId[] getBasicOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getBasicOutputIds(params);
    }

    public OutputId[] getAliasOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getAliasOutputIds(params);
    }

    public OutputId[] getNftOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getNftOutputIds(params);
    }

    public OutputId[] getFoundryOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getFoundryOutputIds(params);
    }

    public OutputId getAliasOutputIdByAliasId(AliasId aliasId) throws ClientException {
        return nodeIndexerApi.getAliasOutputIdByAliasId(aliasId);
    }

    public OutputId getNftOutputIdByNftId(NftId nftId) throws ClientException {
        return nodeIndexerApi.getNftOutputIdByNftId(nftId);
    }


    public OutputId getFoundryOutputIdByFoundryId(FoundryId foundryId) throws ClientException {
        return nodeIndexerApi.getFoundryOutputIdByFoundryId(foundryId);
    }

    // High level APIs

    public Output[] getOutputs(OutputId[] outputIds) throws ClientException {
        return highLevelApi.getOutputs(outputIds);
    }

    public Output[] tryGetOutputs(OutputId[] outputIds) throws ClientException {
        return highLevelApi.tryGetOutputs(outputIds);
    }

    public Block[] findBlocks(BlockId[] blockIds) throws ClientException {
        return highLevelApi.findBlocks(blockIds);
    }

    public Map.Entry<BlockId, Block> retry(BlockId blockId) throws ClientException {
        return highLevelApi.retry(blockId);
    }

    public LinkedHashMap<BlockId, Block> retryUntilIncluded(BlockId blockId, int interval, int maxAttempts) throws ClientException {
        return highLevelApi.retryUntilIncluded(blockId, interval, maxAttempts);
    }

    public String consolidateFunds(SecretManager secretManager, int accountIndex, Range addressRange) throws ClientException {
        return highLevelApi.consolidateFunds(secretManager, accountIndex, addressRange);
    }

    public OutputId[] findInputs(String[] addresses, int amount) throws ClientException {
        return highLevelApi.findInputs(addresses, amount);
    }

    public OutputId[] findOutputs(OutputId[] outputIds, String[] addresses) throws ClientException {
        return highLevelApi.findOutputs(outputIds, addresses);
    }

    public Map.Entry<BlockId, Block> reattach(BlockId blockId) throws ClientException {
        return highLevelApi.reattach(blockId);
    }

    public Map.Entry<BlockId, Block> reattachUnchecked(BlockId blockId) throws ClientException {
        return highLevelApi.reattachUnchecked(blockId);
    }

    public Map.Entry<BlockId, Block> promote(BlockId blockId) throws ClientException {
        return highLevelApi.promote(blockId);
    }

    public Map.Entry<BlockId, Block> promoteUnchecked(BlockId blockId) throws ClientException {
        return highLevelApi.promoteUnchecked(blockId);
    }

    // Utils APIs

    public String bech32ToHex(String bech32) throws ClientException {
        return utilsApi.bech32ToHex(bech32);
    }

    public String hexToBech32(String hex, String bech32) throws ClientException {
        return utilsApi.hexToBech32(hex, bech32);
    }

    public String hexPublicKeyToBech32Address(String hex, String bech32) throws ClientException {
        return utilsApi.hexPublicKeyToBech32Address(hex, bech32);
    }

    public String parseBech32Address(String address) throws ClientException {
        return utilsApi.parseBech32Address(address);
    }

    public boolean isAddressValid(String address) throws ClientException {
        return utilsApi.isAddressValid(address);
    }

    public String generateMnemonic() throws ClientException {
        return utilsApi.generateMnemonic();
    }

    public String mnemonicToHexSeed(String mnemonic) throws ClientException {
        return utilsApi.mnemonicToHexSeed(mnemonic);
    }

    public BlockId getBlockId(Block block) throws ClientException {
        return utilsApi.computeBlockId(block);
    }

    public TransactionId getTransactionId(TransactionPayload payload) throws ClientException {
        return utilsApi.getTransactionId(payload);
    }

    public AliasId computeAliasId(OutputId aliasOutputId) throws ClientException {
        return utilsApi.computeAliasId(aliasOutputId);
    }

    public NftId computeNftId(OutputId nftOutputId) throws ClientException {
        return utilsApi.computeNftId(nftOutputId);
    }

    public FoundryId computeFoundryId(String aliasAddress, int serialNumber, int tokenScheme) throws ClientException {
        return utilsApi.computeFoundryId(aliasAddress, serialNumber, tokenScheme);
    }

    // Miscellaneous APIs

    public String[] generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        return miscellaneousApi.generateAddresses(secretManager, generateAddressesOptions);
    }

    public Block generateBlock(SecretManager secretManager, GenerateBlockOptions options) throws ClientException {
        return miscellaneousApi.generateBlock(secretManager, options);
    }


    public Node getNode() throws ClientException {
        return miscellaneousApi.getNode();
    }

    public JsonObject getNetworkInfo() throws ClientException {
        return miscellaneousApi.getNetworkInfo();
    }

    public int getNetworkId() throws ClientException {
        return miscellaneousApi.getNetworkId();
    }

    public String getBech32Hrp() throws ClientException {
        return miscellaneousApi.getBech32Hrp();
    }

    public float getMinPoWScore() throws ClientException {
        return miscellaneousApi.getMinPoWScore();
    }

    public int getTipsInterval() throws ClientException {
        return miscellaneousApi.getTipsInterval();
    }

    public boolean isLocalPow() throws ClientException {
        return miscellaneousApi.isLocalPow();
    }

    public boolean isFallbackToLocalPoW() throws ClientException {
        return miscellaneousApi.isFallbackToLocalPoW();
    }

    public Node[] getUsyncedNodes() throws ClientException {
        return miscellaneousApi.getUnsyncedNodes();
    }

    public PreparedTransactionData prepareTransaction(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        return miscellaneousApi.prepareTransaction(secretManager, generateAddressesOptions);
    }

    public BlockPayload signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) throws ClientException {
        return miscellaneousApi.signTransaction(secretManager, preparedTransactionData);
    }

    public void storeMnemonic(SecretManager secretManager, String mnemonic) throws ClientException {
        miscellaneousApi.storeMnemonic(secretManager, mnemonic);
    }

    public Block submitBlockPayload(TransactionPayload payload) throws ClientException {
        return miscellaneousApi.postBlockPayload(payload);
    }

}

