// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import com.google.gson.JsonObject;
import org.iota.apis.*;
import org.iota.types.*;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.NoFundsReceivedFromFaucetException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.*;
import org.iota.types.output_builder.AliasOutputBuilderParams;
import org.iota.types.output_builder.BasicOutputBuilderParams;
import org.iota.types.output_builder.FoundryOutputBuilderParams;
import org.iota.types.output_builder.NftOutputBuilderParams;
import org.iota.types.responses.NodeInfoResponse;
import org.iota.types.responses.OutputIdsResponse;
import org.iota.types.responses.ProtocolParametersResponse;
import org.iota.types.responses.TreasuryResponse;
import org.iota.types.responses.UtxoChangesResponse;
import org.iota.types.secret.BuildBlockOptions;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.SecretManager;

import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

public class Client extends NativeApi {

    private static final String TESTNET_FAUCET_URL = "http://localhost:8091/api/enqueue";

    private NodeCoreApi nodeCoreApi;
    private NodeIndexerApi nodeIndexerApi;
    private HighLevelApi highLevelApi;
    private UtilsApi utilsApi;
    private MiscellaneousApi miscellaneousApi;

    /**
     * Creates a new client instance with the given configuration.
     *
     * @param config The configuration for the client.
     * @throws InitializeClientException in case of initialization failure
     */
    public Client(ClientConfig config) throws InitializeClientException {
        super(config);
        nodeCoreApi = new NodeCoreApi(this);
        nodeIndexerApi = new NodeIndexerApi(this);
        highLevelApi = new HighLevelApi(this);
        utilsApi = new UtilsApi(this);
        miscellaneousApi = new MiscellaneousApi(this);
    }

    /**
     * Destroys this client instance and drops its database connection.
     * This client instance can no longer be used after.
     */
    public void destroyHandle() {
        super.destroyHandle();
    }

    // Node Core APIs

    /**
     * Get the health of a given node.
     *
     * @param nodeUrl The URL of the node to check.
     * @return True if the node is healthy, false otherwise.
     * @throws ClientException on error.
     */
    public boolean getHealth(String nodeUrl) throws ClientException {
        return nodeCoreApi.getHealth(nodeUrl);
    }

    /**
     * Get the node information of the given node.
     *
     * @return The node information of the given node.
     * @throws ClientException on error.
     */
    public NodeInfoResponse getNodeInfo() throws ClientException {
        return nodeCoreApi.getNodeInfo();
    }

    /**
     * Get the tips of the Tangle.
     *
     * @return The tips of the Tangle.
     * @throws ClientException on error.
     */
    public BlockId[] getTips() throws ClientException {
        return nodeCoreApi.getTips();
    }

    /**
     * Post a block to the node.
     *
     * @param block The block to be posted.
     * @return The block id of the block that was posted.
     * @throws ClientException on error.
     */
    public BlockId postBlock(Block block) throws ClientException {
        return nodeCoreApi.postBlock(block);
    }

    /**
     * Post a block to the node.
     *
     * @param blockBytes The raw bytes of the block to be posted.
     * @return The block id of the block that was just posted.
     * @throws ClientException on error.
     */
    public BlockId postBlockRaw(byte[] blockBytes) throws ClientException {
        return nodeCoreApi.postBlockRaw(blockBytes);
    }

    /**
     * Get a block by its block id
     *
     * @param blockId The block ID of the block you want to retrieve.
     * @return A block object.
     * @throws ClientException on error.
     */
    public Block getBlock(BlockId blockId) throws ClientException {
        return nodeCoreApi.getBlock(blockId);
    }

    /**
     * Get the raw bytes of a block
     *
     * @param blockId The block ID of the block you want to get.
     * @return The raw bytes of the block.
     * @throws ClientException on error.
     */
    public byte[] getBlockRaw(BlockId blockId) throws ClientException {
        return nodeCoreApi.getBlockRaw(blockId);
    }

    /**
     * Get the metadata of a block
     *
     * @param blockId The id of the block to get metadata for.
     * @return BlockMetadata
     * @throws ClientException on error.
     */
    public BlockMetadata getBlockMetadata(BlockId blockId) throws ClientException {
        return nodeCoreApi.getBlockMetadata(blockId);
    }

    /**
     * Get the output with the given id
     *
     * @param outputId The id of the output you want to get.
     * @return A map entry with the output and its metadata.
     * @throws ClientException on error.
     */
    public Map.Entry<Output, OutputMetadata> getOutput(OutputId outputId) throws ClientException {
        return nodeCoreApi.getOutput(outputId);
    }

    /**
     * Get the metadata of an output
     *
     * @param outputId The output ID of the output you want to get metadata for.
     * @return OutputMetadata
     * @throws ClientException on error.
     */
    public OutputMetadata getOutputMetadata(OutputId outputId) throws ClientException {
        return nodeCoreApi.getOutputMetadata(outputId);
    }

    /**
     * Returns the list of all the receipts that were migrated at the given milestone index
     *
     * @param milestoneIndex The index of the milestone to get the receipts for.
     * @return Receipts
     * @throws ClientException on error.
     */
    public Receipt[] getReceiptsMigratedAt(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getReceiptsMigratedAt(milestoneIndex);
    }

    /**
     * Get all the receipts.
     *
     * @return An array of Receipt objects.
     * @throws ClientException on error.
     */
    public Receipt[] getReceipts() throws ClientException {
        return nodeCoreApi.getReceipts();
    }

    /**
     * Get the treasury balance
     *
     * @return TreasuryResponse
     * @throws ClientException on error.
     */
    public TreasuryResponse getTreasury() throws ClientException {
        return nodeCoreApi.getTreasury();
    }

    /**
     * Returns the block that contains the transaction that was included in the ledger.
     *
     * @param transactionId The transaction that was included in the ledger.
     * @return A block that contains the transaction.
     * @throws ClientException on error.
     */
    public Block getIncludedBlock(TransactionId transactionId) throws ClientException {
        return nodeCoreApi.getIncludedBlock(transactionId);
    }

    /**
     * Returns the metadata of the block that contains the transaction that was included in the ledger.
     *
     * @param transactionId The transaction that was included in the ledger.
     * @return The metadata of the block that contains the transaction.
     * @throws ClientException on error.
     */
    public BlockMetadata getIncludedBlockMetadata(TransactionId transactionId) throws ClientException {
        return nodeCoreApi.getIncludedBlockMetadata(transactionId);
    }

    /**
     * Get the milestone payload for the given milestone id
     *
     * @param milestoneId The milestone ID of the milestone you want to get.
     * @return A MilestonePayload object.
     * @throws ClientException on error.
     */
    public MilestonePayload getMilestoneById(MilestoneId milestoneId) throws ClientException {
        return nodeCoreApi.getMilestoneById(milestoneId);
    }

    /**
     * Get the milestone at the specified index
     *
     * @param milestoneIndex The index of the milestone you want to retrieve.
     * @return A MilestonePayload object.
     * @throws ClientException on error.
     */
    public MilestonePayload getMilestoneByIndex(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getMilestoneByIndex(milestoneIndex);
    }

    /**
     * Returns the raw bytes of a milestone by its id
     *
     * @param milestoneId The milestone ID to get the raw data for.
     * @return A byte array of the milestone data.
     * @throws ClientException on error.
     */
    public byte[] getMilestoneByIdRaw(MilestoneId milestoneId) throws ClientException {
        return nodeCoreApi.getMilestoneByIdRaw(milestoneId);
    }

    /**
     * Returns the raw bytes of the milestone at the given index
     *
     * @param milestoneIndex The index of the milestone you want to retrieve.
     * @return The raw bytes of the milestone at the given index.
     * @throws ClientException on error.
     */
    public byte[] getMilestoneByIndexRaw(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getMilestoneByIndexRaw(milestoneIndex);
    }

    /**
     * Returns the UTXO changes for the given milestone id
     *
     * @param milestoneId The milestone id of the milestone to get the UTXO changes for.
     * @return The UTXO changes for the given milestone index.
     * @throws ClientException on error.
     */
    public UtxoChangesResponse getUtxoChangesById(MilestoneId milestoneId) throws ClientException {
        return nodeCoreApi.getUtxoChangesById(milestoneId);
    }

    /**
     * Returns the UTXO changes for the given milestone index
     *
     * @param milestoneIndex The index of the milestone to get the UTXO changes for.
     * @return The UTXO changes for the given milestone index.
     * @throws ClientException on error.
     */
    public UtxoChangesResponse getUtxoChangesByIndex(int milestoneIndex) throws ClientException {
        return nodeCoreApi.getUtxoChangesByIndex(milestoneIndex);
    }

    /**
     * Get the list of peers connected to the node
     *
     * @return An array of Peer objects.
     * @throws ClientException on error.
     */
    public Peer[] getPeers() throws ClientException {
        return nodeCoreApi.getPeers();
    }

    // Node Indexer APIs

    /**
     * Returns the basic output ids that match the given query parameters
     *
     * @param params a QueryParams object that contains the following fields:
     * @return OutputIdsResponse.
     * @throws ClientException on error.
     */
    public OutputIdsResponse getBasicOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getBasicOutputIds(params);
    }

    /**
     * Returns the alias output ids that match the given query parameters
     *
     * @param params a QueryParams object that contains the following fields:
     * @return OutputIdsResponse.
     * @throws ClientException on error.
     */
    public OutputIdsResponse getAliasOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getAliasOutputIds(params);
    }

    /**
     * Returns the NFT output ids that match the given query parameters
     *
     * @param params a QueryParams object that contains the following fields:
     * @return OutputIdsResponse.
     * @throws ClientException on error.
     */
    public OutputIdsResponse getNftOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getNftOutputIds(params);
    }

    /**
     * Returns the Foundry output ids that match the given query parameters
     *
     * @param params a QueryParams object that contains the following fields:
     * @return OutputIdsResponse.
     * @throws ClientException on error.
     */
    public OutputIdsResponse getFoundryOutputIds(NodeIndexerApi.QueryParams params) throws ClientException {
        return nodeIndexerApi.getFoundryOutputIds(params);
    }

    /**
     * The aliasId of the alias you want to get the outputId for.eturns the output id of an alias by its alias id
     *
     * @param aliasId The aliasId of the alias you want to get the outputId for.
     * @return OutputId
     * @throws ClientException on error.
     */
    public OutputId getAliasOutputIdByAliasId(AliasId aliasId) throws ClientException {
        return nodeIndexerApi.getAliasOutputIdByAliasId(aliasId);
    }

    /**
     * The aliasId of the alias you want to get the outputId for.
     *
     * @param nftId The NFT Id of the NFT you want to get the outputId for.
     * @return OutputId
     * @throws ClientException on error.
     */
    public OutputId getNftOutputIdByNftId(NftId nftId) throws ClientException {
        return nodeIndexerApi.getNftOutputIdByNftId(nftId);
    }

    /**
     * The aliasId of the alias you want to get the outputId for.
     *
     * @param foundryId The id of the foundry you want to get the output id for.
     * @return The output id of the foundry.
     * @throws ClientException on error.
     */
    public OutputId getFoundryOutputIdByFoundryId(FoundryId foundryId) throws ClientException {
        return nodeIndexerApi.getFoundryOutputIdByFoundryId(foundryId);
    }

    // High level APIs

    /**
     * Get the outputs for the given output IDs
     *
     * @param outputIds An array of OutputId objects.
     * @return A list of entries, where each entry is a pair of an Output and its OutputMetadata.
     * @throws ClientException on error.
     */
    public List<Map.Entry<Output, OutputMetadata>> getOutputs(OutputId[] outputIds) throws ClientException {
        return highLevelApi.getOutputs(outputIds);
    }

    /**
     * Try get the outputs for the given output IDs
     *
     * @param outputIds An array of OutputId objects.
     * @return A list of entries, where each entry is a pair of an output and its metadata.
     * @throws ClientException on error.
     */
    public List<Map.Entry<Output, OutputMetadata>> tryGetOutputs(OutputId[] outputIds) throws ClientException {
        return highLevelApi.tryGetOutputs(outputIds);
    }

    /**
     * Finds blocks by their ids
     *
     * @param blockIds An array of BlockId objects.
     * @return An array of blocks.
     * @throws ClientException on error.
     */
    public Block[] findBlocks(BlockId[] blockIds) throws ClientException {
        return highLevelApi.findBlocks(blockIds);
    }

    /**
     * Retry a block that has failed to process
     *
     * @param blockId The id of the block to retry.
     * @return A map entry with the block id and the block.
     * @throws ClientException on error.
     */
    public Map.Entry<BlockId, Block> retry(BlockId blockId) throws ClientException {
        return highLevelApi.retry(blockId);
    }

    /**
     * Retry until the block is included in the blockchain
     *
     * @param blockId The block ID to retry.
     * @param interval The interval in seconds between each attempt to retrieve the block.
     * @param maxAttempts The maximum number of attempts to make before giving up.
     * @return A LinkedHashMap of BlockId and Block.
     * @throws ClientException on error.
     */
    public LinkedHashMap<BlockId, Block> retryUntilIncluded(BlockId blockId, int interval, int maxAttempts) throws ClientException {
        return highLevelApi.retryUntilIncluded(blockId, interval, maxAttempts);
    }

    /**
     * This function will consolidate all funds of the wallet
     *
     * @param secretManager The secret manager that holds the secret for the account you want to consolidate funds for.
     * @param generateAddressesOptions This is an object that contains the following parameters:
     * @return A String containing the transaction hash.
     * @throws ClientException on error.
     */
    public String consolidateFunds(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        return highLevelApi.consolidateFunds(secretManager, generateAddressesOptions);
    }

    /**
     * Finds the inputs that are needed to create a transaction with the given amount
     *
     * @param addresses An array of addresses to search for inputs.
     * @param amount The amount you want to spend.
     * @return An array of UtxoInput objects.
     * @throws ClientException on error.
     */
    public UtxoInput[] findInputs(String[] addresses, int amount) throws ClientException {
        return highLevelApi.findInputs(addresses, amount);
    }

    /**
     * Finds outputs by their ids and addresses
     *
     * @param outputIds An array of OutputId objects.
     * @param addresses The addresses to search for outputs.
     * @return A list of outputs and their metadata.
     * @throws ClientException on error.
     */
    public List<Map.Entry<Output, OutputMetadata>> findOutputs(OutputId[] outputIds, String[] addresses) throws ClientException {
        return highLevelApi.findOutputs(outputIds, addresses);
    }

    /**
     * Reattaches a block to the blockchain
     *
     * @param blockId The id of the block to reattach.
     * @return A map entry with the block id and the block.
     * @throws ClientException on error.
     */
    public Map.Entry<BlockId, Block> reattach(BlockId blockId) throws ClientException {
        return highLevelApi.reattach(blockId);
    }

    /**
     * ReattachUnchecked() reattaches a block to the tangle, without checking if it's necessary to.
     *
     * @param blockId The block id of the block to reattach.
     * @return A map entry with the block id and the block.
     * @throws ClientException on error.
     */
    public Map.Entry<BlockId, Block> reattachUnchecked(BlockId blockId) throws ClientException {
        return highLevelApi.reattachUnchecked(blockId);
    }

    /**
     * Promote a block
     *
     * @param blockId The id of the block to promote.
     * @return A map entry with the block id and the block.
     * @throws ClientException on error.
     */
    public Map.Entry<BlockId, Block> promote(BlockId blockId) throws ClientException {
        return highLevelApi.promote(blockId);
    }

    /**
     * Promote a block, without checking if it's necessary to.
     *
     * @param blockId The id of the block to promote.
     * @return A map entry with the block id and the block.
     * @throws ClientException on error.
     */
    public Map.Entry<BlockId, Block> promoteUnchecked(BlockId blockId) throws ClientException {
        return highLevelApi.promoteUnchecked(blockId);
    }

    // Utils APIs

    /**
     * Converts a bech32 address to a hex address
     *
     * @param bech32 bech32 address
     * @return A hex string.
     * @throws ClientException on error.
     */
    public String bech32ToHex(String bech32) throws ClientException {
        return utilsApi.bech32ToHex(bech32);
    }

    /**
     * Converts a hex string to a bech32 string
     *
     * @param hex The hexadecimal string to be converted.
     * @param bech32 The bech32 to use.
     * @return The bech32 address.
     * @throws ClientException on error.
     */
    public String hexToBech32(String hex, String bech32) throws ClientException {
        return utilsApi.hexToBech32(hex, bech32);
    }

    /**
     * Converts an alias id to a bech32 address.
     *
     * @param aliasId The alias id to be converted.
     * @param bech32Hrp The bech 32 human readable part to use.
     * @return The bech32 address.
     * @throws ClientException on error.
     */
    public String aliasIdToBech32(AliasId aliasId, String bech32Hrp) throws ClientException {
        return utilsApi.aliasIdToBech32(aliasId, bech32Hrp);
    }

    /**
     * Converts an nft id to a bech32 address.
     *
     * @param nftId The nft id to be converted.
     * @param bech32Hrp The bech 32 human readable part to use.
     * @return The bech32 address.
     * @throws ClientException on error.
     */
    public String nftIdToBech32(NftId nftId, String bech32Hrp) throws ClientException {
        return utilsApi.nftIdToBech32(nftId, bech32Hrp);
    }

    /**
     * Converts a hex public key to a bech32 address
     *
     * @param hex The public key in hexadecimal format.
     * @param bech32 The bech32 prefix
     * @return The bech32 address.
     * @throws ClientException on error.
     */
    public String hexPublicKeyToBech32Address(String hex, String bech32) throws ClientException {
        return utilsApi.hexPublicKeyToBech32Address(hex, bech32);
    }

    /**
     * This function is used to parse the Bech32 address
     *
     * @param address The address to be parsed.
     * @return The address in hex format.
     * @throws ClientException on error.
     */
    public String parseBech32Address(String address) throws ClientException {
        return utilsApi.parseBech32Address(address);
    }

    /**
     * Checks if the given address is valid
     *
     * @param address The address to validate.
     * @return A boolean value.
     * @throws ClientException on error.
     */
    public boolean isAddressValid(String address) throws ClientException {
        return utilsApi.isAddressValid(address);
    }

    /**
     * Generate a mnemonic
     *
     * @return The mnemonic string.
     * @throws ClientException on error.
     */
    public String generateMnemonic() throws ClientException {
        return utilsApi.generateMnemonic();
    }

    /**
     * Converts a mnemonic to a hex seed
     *
     * @param mnemonic The mnemonic to convert to a hex seed.
     * @return A hex seed.
     * @throws ClientException on error.
     */
    public String mnemonicToHexSeed(String mnemonic) throws ClientException {
        return utilsApi.mnemonicToHexSeed(mnemonic);
    }

    /**
     * This function computes the block id of a block
     *
     * @param block The block to compute the block id from.
     * @return The block id of the block.
     * @throws ClientException on error.
     */
    public BlockId getBlockId(Block block) throws ClientException {
        return utilsApi.computeBlockId(block);
    }

    /**
     * This function returns a transaction ID for a given transaction payload
     *
     * @param payload The payload of the transaction.
     * @return A TransactionId object.
     * @throws ClientException on error.
     */
    public TransactionId getTransactionId(TransactionPayload payload) throws ClientException {
        return utilsApi.getTransactionId(payload);
    }

    /**
     * Computes the alias id for the given alias output id
     *
     * @param aliasOutputId The output ID of the alias.
     * @return The alias id of the output id.
     * @throws ClientException on error.
     */
    public AliasId computeAliasId(OutputId aliasOutputId) throws ClientException {
        return utilsApi.computeAliasId(aliasOutputId);
    }

    /**
     * Computes the NFT ID from the NFT output ID
     *
     * @param nftOutputId The output id of the NFT.
     * @return The NFT ID of the NFT output.
     * @throws ClientException on error.
     */
    public NftId computeNftId(OutputId nftOutputId) throws ClientException {
        return utilsApi.computeNftId(nftOutputId);
    }

    /**
     * Computes the Foundry ID for a given alias address, serial number, and token scheme
     *
     * @param aliasAddress The alias address of the device.
     * @param serialNumber The serial number of the token.
     * @param tokenScheme The token scheme to use. This is a value from the TokenScheme enum.
     * @return A FoundryId object.
     * @throws ClientException on error.
     */
    public FoundryId computeFoundryId(String aliasAddress, int serialNumber, int tokenScheme) throws ClientException {
        return utilsApi.computeFoundryId(aliasAddress, serialNumber, tokenScheme);
    }

    // Miscellaneous APIs

    /**
     * Builds an alias output
     *
     * @param params AliasOutputBuilderParams
     * @return An output object.
     * @throws ClientException on error.
     */
    public Output buildAliasOutput(
            AliasOutputBuilderParams params
    ) throws ClientException {
        return miscellaneousApi.buildAliasOutput(params);
    }

    /**
     * Builds a basic output
     *
     * @param params The parameters for the request.
     * @return An output object.
     * @throws ClientException on error.
     */
    public Output buildBasicOutput(
            BasicOutputBuilderParams params
    ) throws ClientException {
        return miscellaneousApi.buildBasicOutput(params);
    }

    /**
     * Builds a Foundry output object from the given parameters
     *
     * @param params FoundryOutputBuilderParams
     * @return An output object.
     * @throws ClientException on error.
     */
    public Output buildFoundryOutput(
            FoundryOutputBuilderParams params
    ) throws ClientException {
        return miscellaneousApi.buildFoundryOutput(params);
    }

    /**
     * Builds an NFT output
     *
     * @param params NftOutputBuilderParams
     * @return Output
     * @throws ClientException on error.
     */
    public Output buildNftOutput(
            NftOutputBuilderParams params
    ) throws ClientException {
        return miscellaneousApi.buildNftOutput(params);
    }

    /**
     * Generate addresses for a given secret manager
     *
     * @param secretManager The secret manager to use for generating the addresses.
     * @param generateAddressesOptions The options for generating addresses.
     * @return A list of addresses
     * @throws ClientException on error.
     */
    public String[] generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        return miscellaneousApi.generateAddresses(secretManager, generateAddressesOptions);
    }

    /**
     * Builds a block and posts it to the network
     *
     * @param secretManager The secret manager to use for signing the block.
     * @param options The options for building the block.
     * @return A map entry with the block id and the block itself.
     * @throws ClientException on error.
     */
    public Map.Entry<BlockId, Block> buildAndPostBlock(SecretManager secretManager, BuildBlockOptions options) throws ClientException {
        return miscellaneousApi.buildAndPostBlock(secretManager, options);
    }

    /**
     * Get a node
     *
     * @return The node object.
     * @throws ClientException on error.
     */
    public Node getNode() throws ClientException {
        return miscellaneousApi.getNode();
    }

    /**
     * Get network information
     *
     * @return A JsonObject
     * @throws ClientException on error.
     */
    public JsonObject getNetworkInfo() throws ClientException {
        return miscellaneousApi.getNetworkInfo();
    }

    /**
     * Get the network ID of the current network
     *
     * @return The network ID of the current network.
     * @throws ClientException on error.
     */
    public int getNetworkId() throws ClientException {
        return miscellaneousApi.getNetworkId();
    }

    /**
     * Get the Bech32 HRP for the current network
     *
     * @return The Bech32 Human Readable Part (HRP) for addresses.
     * @throws ClientException on error.
     */
    public String getBech32Hrp() throws ClientException {
        return miscellaneousApi.getBech32Hrp();
    }

    /**
     * Get the minimum PoW score required for a transaction to be accepted
     *
     * @return The minimum PoW score.
     * @throws ClientException on error.
     */
    public float getMinPowScore() throws ClientException {
        return miscellaneousApi.getMinPowScore();
    }

    /**
     * Get the tips interval in seconds
     *
     * @return The interval in seconds.
     * @throws ClientException on error.
     */
    public int getTipsInterval() throws ClientException {
        return miscellaneousApi.getTipsInterval();
    }

    /**
     * Returns true if the local PoW is enabled, otherwise false.
     *
     * @return true if the local PoW is enabled, otherwise false.
     * @throws ClientException on error.
     */
    public boolean getLocalPow() throws ClientException {
        return miscellaneousApi.getLocalPow();
    }

    /**
     * Returns true if the client is configured to fallback to local proof of work if the remote proof of work service is
     * unavailable
     *
     * @return true if the client is configured to fallback to local proof of work, else returns false.
     * @throws ClientException on error.
     */
    public boolean getFallbackToLocalPow() throws ClientException {
        return miscellaneousApi.isFallbackToLocalPow();
    }

    /**
     * Get the list of nodes that are not healthy
     *
     * @return The array of nodes that are not healthy.
     * @throws ClientException on error.
     */
    public Node[] getUnhealthyNodes() throws ClientException {
        return miscellaneousApi.getUnhealthyNodes();
    }

    /**
     * Get the status of the Ledger Nano device
     *
     * @param isSimulator true if you want to use the simulator, false if you want to use the real device.
     * @return LedgerNanoStatus
     * @throws ClientException on error.
     */
    public LedgerNanoStatus getLedgerNanoStatus(boolean isSimulator) throws ClientException {
        return miscellaneousApi.getLedgerNanoStatus(isSimulator);
    }

    /**
     * Prepares a transaction for signing
     *
     * @param secretManager The secret manager that holds the secret for the account.
     * @param buildBlockOptions This is the object that contains the transaction details.
     * @return The prepared transaction.
     * @throws ClientException on error.
     */
    public PreparedTransactionData prepareTransaction(SecretManager secretManager, BuildBlockOptions buildBlockOptions) throws ClientException {
        return miscellaneousApi.prepareTransaction(secretManager, buildBlockOptions);
    }

    /**
     * Signs the given prepared transaction using the secret manager
     *
     * @param secretManager The secret manager that holds the secret key for the account that will sign the transaction.
     * @param preparedTransactionData The transaction data that was prepared by the prepareTransaction method.
     * @return The signed transaction.
     * @throws ClientException on error.
     */
    public TransactionPayload signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) throws ClientException {
        return miscellaneousApi.signTransaction(secretManager, preparedTransactionData);
    }

    /**
     * Stores the mnemonic in the secret manager
     *
     * @param secretManager The secret manager to use.
     * @param mnemonic The mnemonic to store.
     * @throws ClientException on error.
     */
    public void storeMnemonic(SecretManager secretManager, String mnemonic) throws ClientException {
        miscellaneousApi.storeMnemonic(secretManager, mnemonic);
    }

    /**
     * Automatically builds a block containing the given block payload and broadcasts it to the network.
     *
     * @param payload The payload to be posted.
     * @return A map entry of the posted block id and the block.
     * @throws ClientException on error.
     */
    public Map.Entry<BlockId, Block> postBlockPayload(BlockPayload payload) throws ClientException {
        return miscellaneousApi.postBlockPayload(payload);
    }

    /**
     * Returns the token supply.
     *
     * @return The token supply.
     * @throws ClientException on error.
     */
    public String getTokenSupply() throws ClientException {
        return getProtocolParameters().getTokenSupply();
    }

    /**
     * Returns the protocol parameters.
     *
     * @return The protocol parameters.
     * @throws ClientException on error.
     */
    public ProtocolParametersResponse getProtocolParameters() throws ClientException {
        return miscellaneousApi.getProtocolParameters();
    }

    /**
     * Asks the faucet for the testnet funds.
     *
     * @throws NoFundsReceivedFromFaucetException when the faucet didn't fund the address.
     */
    public void requestTestFundsFromFaucet(String address) throws ClientException, NoFundsReceivedFromFaucetException {
        int maxAttempts = 120;
        for(int i = 0; i < maxAttempts; i++) {
            // Request every 30 seconds
            if (i%30 == 0){
                String response2 = utilsApi.requestFundsFromFaucet(TESTNET_FAUCET_URL, address);
                System.out.println(response2);
            }
            if(getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address)).getItems().length == 0) {
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            } else
                return;
        }

        if(getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address)).getItems().length == 0)
            throw new NoFundsReceivedFromFaucetException();
    }

}