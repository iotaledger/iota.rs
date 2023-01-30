---
description: Official IOTA Client Library Java API reference.
image: /img/logo/iota_mark_light.png
keywords:

- api
- java

---

# API Reference

# Client.java

## `public boolean getHealth(String nodeUrl) throws ClientException`

Get the health of a given node.

* **Parameters:** `nodeUrl` — The URL of the node to check.
* **Returns:** True if the node is healthy, false otherwise.

## `public NodeInfoResponse getNodeInfo() throws ClientException`

Get the node information of the given node.

* **Returns:** The node information of the given node.

## `public BlockId[] getTips() throws ClientException`

Get the tips of the Tangle.

* **Returns:** The tips of the Tangle.

## `public BlockId postBlock(Block block) throws ClientException`

Post a block to the node.

* **Parameters:** `block` — The block to be posted.
* **Returns:** The block id of the block that was posted.

## `public BlockId postBlockRaw(byte[] blockBytes) throws ClientException`

Post a block to the node.

* **Parameters:** `blockBytes` — The raw bytes of the block to be posted.
* **Returns:** The block id of the block that was just posted.

## `public Block getBlock(BlockId blockId) throws ClientException`

Get a block by its block id

* **Parameters:** `blockId` — The block ID of the block you want to retrieve.
* **Returns:** A block object.

## `public byte[] getBlockRaw(BlockId blockId) throws ClientException`

Get the raw bytes of a block

* **Parameters:** `blockId` — The block ID of the block you want to get.
* **Returns:** The raw bytes of the block.

## `public BlockMetadata getBlockMetadata(BlockId blockId) throws ClientException`

Get the metadata of a block

* **Parameters:** `blockId` — The id of the block to get metadata for.
* **Returns:** BlockMetadata

## `public Map.Entry<Output, OutputMetadata> getOutput(OutputId outputId) throws ClientException`

Get the output with the given id

* **Parameters:** `outputId` — The id of the output you want to get.
* **Returns:** A map entry with the output and its metadata.

## `public OutputMetadata getOutputMetadata(OutputId outputId) throws ClientException`

Get the metadata of an output

* **Parameters:** `outputId` — The output ID of the output you want to get metadata for.
* **Returns:** OutputMetadata

## `public Receipt[] getReceiptsMigratedAt(int milestoneIndex) throws ClientException`

Returns the list of all the receipts that were migrated at the given milestone index

* **Parameters:** `milestoneIndex` — The index of the milestone to get the receipts for.
* **Returns:** Receipts

## `public Receipt[] getReceipts() throws ClientException`

Get all the receipts.

* **Returns:** An array of Receipt objects.

## `public TreasuryResponse getTreasury() throws ClientException`

Get the treasury balance

* **Returns:** TreasuryResponse

## `public Block getIncludedBlock(TransactionId transactionId) throws ClientException`

Returns the block that contains the transaction that was included in the ledger.

* **Parameters:** `transactionId` — The transaction that was included in the ledger.
* **Returns:** A block that contains the transaction.

## `public MilestonePayload getMilestoneById(MilestoneId milestoneId) throws ClientException`

Get the milestone payload for the given milestone id

* **Parameters:** `milestoneId` — The milestone ID of the milestone you want to get.
* **Returns:** A MilestonePayload object.

## `public MilestonePayload getMilestoneByIndex(int milestoneIndex) throws ClientException`

Get the milestone at the specified index

* **Parameters:** `milestoneIndex` — The index of the milestone you want to retrieve.
* **Returns:** A MilestonePayload object.

## `public byte[] getMilestoneByIdRaw(MilestoneId milestoneId) throws ClientException`

Returns the raw bytes of a milestone by its id

* **Parameters:** `milestoneId` — The milestone ID to get the raw data for.
* **Returns:** A byte array of the milestone data.

## `public byte[] getMilestoneByIndexRaw(int milestoneIndex) throws ClientException`

Returns the raw bytes of the milestone at the given index

* **Parameters:** `milestoneIndex` — The index of the milestone you want to retrieve.
* **Returns:** The raw bytes of the milestone at the given index.

## `public UtxoChangesResponse getUtxoChangesById(MilestoneId milestoneId) throws ClientException`

Returns the UTXO changes for the given milestone id

* **Parameters:** `milestoneId` — The milestone id of the milestone to get the UTXO changes for.
* **Returns:** The UTXO changes for the given milestone index.

## `public UtxoChangesResponse getUtxoChangesByIndex(int milestoneIndex) throws ClientException`

Returns the UTXO changes for the given milestone index

* **Parameters:** `milestoneIndex` — The index of the milestone to get the UTXO changes for.
* **Returns:** The UTXO changes for the given milestone index.

## `public Peer[] getPeers() throws ClientException`

Get the list of peers connected to the node

* **Returns:** An array of Peer objects.

## `public OutputId[] getBasicOutputIds(NodeIndexerApi.QueryParams params) throws ClientException`

Returns the basic output ids that match the given query parameters

* **Parameters:** `params` — a QueryParams object that contains the following fields:
* **Returns:** An array of OutputIds.

## `public OutputId[] getAliasOutputIds(NodeIndexerApi.QueryParams params) throws ClientException`

Returns the alias output ids that match the given query parameters

* **Parameters:** `params` — a QueryParams object that contains the following fields:
* **Returns:** An array of OutputIds.

## `public OutputId[] getNftOutputIds(NodeIndexerApi.QueryParams params) throws ClientException`

Returns the NFT output ids that match the given query parameters

* **Parameters:** `params` — a QueryParams object that contains the following fields:
* **Returns:** An array of OutputIds.

## `public OutputId[] getFoundryOutputIds(NodeIndexerApi.QueryParams params) throws ClientException`

Returns the Foundry output ids that match the given query parameters

* **Parameters:** `params` — a QueryParams object that contains the following fields:
* **Returns:** An array of OutputIds.

## `public OutputId getAliasOutputIdByAliasId(AliasId aliasId) throws ClientException`

The aliasId of the alias you want to get the outputId for.eturns the output id of an alias by its alias id

* **Parameters:** `aliasId` — The aliasId of the alias you want to get the outputId for.
* **Returns:** OutputId

## `public OutputId getNftOutputIdByNftId(NftId nftId) throws ClientException`

The aliasId of the alias you want to get the outputId for.

* **Parameters:** `nftId` — The NFT Id of the NFT you want to get the outputId for.
* **Returns:** OutputId

## `public OutputId getFoundryOutputIdByFoundryId(FoundryId foundryId) throws ClientException`

The aliasId of the alias you want to get the outputId for.

* **Parameters:** `foundryId` — The id of the foundry you want to get the output id for.
* **Returns:** The output id of the foundry.

## `public List<Map.Entry<Output, OutputMetadata>> getOutputs(OutputId[] outputIds) throws ClientException`

Get the outputs for the given output IDs

* **Parameters:** `outputIds` — An array of OutputId objects.
* **Returns:** A list of entries, where each entry is a pair of an Output and its OutputMetadata.

## `public List<Map.Entry<Output, OutputMetadata>> tryGetOutputs(OutputId[] outputIds) throws ClientException`

Try get the outputs for the given output IDs

* **Parameters:** `outputIds` — An array of OutputId objects.
* **Returns:** A list of entries, where each entry is a pair of an output and its metadata.

## `public Block[] findBlocks(BlockId[] blockIds) throws ClientException`

Finds blocks by their ids

* **Parameters:** `blockIds` — An array of BlockId objects.
* **Returns:** An array of blocks.

## `public Map.Entry<BlockId, Block> retry(BlockId blockId) throws ClientException`

Retry a block that has failed to process

* **Parameters:** `blockId` — The id of the block to retry.
* **Returns:** A map entry with the block id and the block.

## `public LinkedHashMap<BlockId, Block> retryUntilIncluded(BlockId blockId, int interval, int maxAttempts) throws ClientException`

Retry until the block is included in the blockchain

* **Parameters:**
    * `blockId` — The block ID to retry.
    * `interval` — The interval in seconds between each attempt to retrieve the block.
    * `maxAttempts` — The maximum number of attempts to make before giving up.
* **Returns:** A LinkedHashMap of BlockId and Block.

## `public String consolidateFunds(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException`

This function will consolidate all funds of the wallet

* **Parameters:**
    * `secretManager` — The secret manager that holds the secret for the account you want to consolidate funds for.
    * `generateAddressesOptions` — This is an object that contains the following parameters:
* **Returns:** A String containing the transaction hash.

## `public UtxoInput[] findInputs(String[] addresses, int amount) throws ClientException`

Finds the inputs that are needed to create a transaction with the given amount

* **Parameters:**
    * `addresses` — An array of addresses to search for inputs.
    * `amount` — The amount you want to spend.
* **Returns:** An array of UtxoInput objects.

## `public List<Map.Entry<Output, OutputMetadata>> findOutputs(OutputId[] outputIds, String[] addresses) throws ClientException`

Finds outputs by their ids and addresses

* **Parameters:**
    * `outputIds` — An array of OutputId objects.
    * `addresses` — The addresses to search for outputs.
* **Returns:** A list of outputs and their metadata.

## `public Map.Entry<BlockId, Block> reattach(BlockId blockId) throws ClientException`

Reattaches a block to the blockchain

* **Parameters:** `blockId` — The id of the block to reattach.
* **Returns:** A map entry with the block id and the block.

## `public Map.Entry<BlockId, Block> reattachUnchecked(BlockId blockId) throws ClientException`

ReattachUnchecked() reattaches a block to the tangle, without checking if it's necessary to.

* **Parameters:** `blockId` — The block id of the block to reattach.
* **Returns:** A map entry with the block id and the block.

## `public Map.Entry<BlockId, Block> promote(BlockId blockId) throws ClientException`

Promote a block

* **Parameters:** `blockId` — The id of the block to promote.
* **Returns:** A map entry with the block id and the block.

## `public Map.Entry<BlockId, Block> promoteUnchecked(BlockId blockId) throws ClientException`

Promote a block, without checking if it's necessary to.

* **Parameters:** `blockId` — The id of the block to promote.
* **Returns:** A map entry with the block id and the block.

## `public String bech32ToHex(String bech32) throws ClientException`

Converts a bech32 address to a hex address

* **Parameters:** `bech32` — bech32 address
* **Returns:** A hex string.

## `public String hexToBech32(String hex, String bech32) throws ClientException`

Converts a hex string to a bech32 string

* **Parameters:**
    * `hex` — The hexadecimal string to be converted.
    * `bech32` — The bech32 to use.
* **Returns:** The bech32 address.

## `public String hexPublicKeyToBech32Address(String hex, String bech32) throws ClientException`

Converts a hex public key to a bech32 address

* **Parameters:**
    * `hex` — The public key in hexadecimal format.
    * `bech32` — The bech32 prefix
* **Returns:** The bech32 address.

## `public String parseBech32Address(String address) throws ClientException`

This function is used to parse the Bech32 address

* **Parameters:** `address` — The address to be parsed.
* **Returns:** The address in hex format.

## `public boolean isAddressValid(String address) throws ClientException`

Checks if the given address is valid

* **Parameters:** `address` — The address to validate.
* **Returns:** A boolean value.

## `public String generateMnemonic() throws ClientException`

Generate a mnemonic

* **Returns:** The mnemonic string.

## `public String mnemonicToHexSeed(String mnemonic) throws ClientException`

Converts a mnemonic to a hex seed

* **Parameters:** `mnemonic` — The mnemonic to convert to a hex seed.
* **Returns:** A hex seed.

## `public BlockId getBlockId(Block block) throws ClientException`

This function computes the block id of a block

* **Parameters:** `block` — The block to compute the block id from.
* **Returns:** The block id of the block.

## `public TransactionId getTransactionId(TransactionPayload payload) throws ClientException`

This function returns a transaction ID for a given transaction payload

* **Parameters:** `payload` — The payload of the transaction.
* **Returns:** A TransactionId object.

## `public AliasId computeAliasId(OutputId aliasOutputId) throws ClientException`

Computes the alias id for the given alias output id

* **Parameters:** `aliasOutputId` — The output ID of the alias.
* **Returns:** The alias id of the output id.

## `public NftId computeNftId(OutputId nftOutputId) throws ClientException`

Computes the NFT ID from the NFT output ID

* **Parameters:** `nftOutputId` — The output id of the NFT.
* **Returns:** The NFT ID of the NFT output.

## `public FoundryId computeFoundryId(String aliasAddress, int serialNumber, int tokenScheme) throws ClientException`

Computes the Foundry ID for a given alias address, serial number, and token scheme

* **Parameters:**
    * `aliasAddress` — The alias address of the device.
    * `serialNumber` — The serial number of the token.
    * `tokenScheme` — The token scheme to use. This is a value from the TokenScheme enum.
* **Returns:** A FoundryId object.

## `public Output buildAliasOutput( AliasOutputBuilderParams params ) throws ClientException`

Builds an alias output

* **Parameters:** `params` — AliasOutputBuilderParams
* **Returns:** An output object.

## `public Output buildBasicOutput( BasicOutputBuilderParams params ) throws ClientException`

Builds a basic output

* **Parameters:** `params` — The parameters for the request.
* **Returns:** An output object.

## `public Output buildFoundryOutput( FoundryOutputBuilderParams params ) throws ClientException`

Builds a Foundry output object from the given parameters

* **Parameters:** `params` — FoundryOutputBuilderParams
* **Returns:** An output object.

## `public Output buildNftOutput( NftOutputBuilderParams params ) throws ClientException`

Builds an NFT output

* **Parameters:** `params` — NftOutputBuilderParams
* **Returns:** Output

## `public String[] generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException`

Generate addresses for a given secret manager

* **Parameters:**
    * `secretManager` — The secret manager to use for generating the addresses.
    * `generateAddressesOptions` — The options for generating addresses.
* **Returns:** A list of addresses

## `public Map.Entry<BlockId, Block> buildAndPostBlock(SecretManager secretManager, BuildBlockOptions options) throws ClientException`

Builds a block and posts it to the network

* **Parameters:**
    * `secretManager` — The secret manager to use for signing the block.
    * `options` — The options for building the block.
* **Returns:** A map entry with the block id and the block itself.

## `public Node getNode() throws ClientException`

Get a node

* **Returns:** The node object.

## `public JsonObject getNetworkInfo() throws ClientException`

Get network information

* **Returns:** A JsonObject

## `public int getNetworkId() throws ClientException`

Get the network ID of the current network

* **Returns:** The network ID of the current network.

## `public String getBech32Hrp() throws ClientException`

Get the Bech32 HRP for the current network

* **Returns:** The Bech32 Human Readable Part (HRP) for addresses.

## `public float getMinPowScore() throws ClientException`

Get the minimum PoW score required for a transaction to be accepted

* **Returns:** The minimum PoW score.

## `public int getTipsInterval() throws ClientException`

Get the tips interval in seconds

* **Returns:** The interval in seconds.

## `public boolean getLocalPow() throws ClientException`

Returns true if the local PoW is enabled, otherwise false.

* **Returns:** true if the local PoW is enabled, otherwise false.

## `public boolean getFallbackToLocalPow() throws ClientException`

Returns true if the client is configured to fallback to local proof of work if the remote proof of work service is unavailable

* **Returns:** true if the client is configured to fallback to local proof of work, else returns false.

## `public Node[] getUnhealthyNodes() throws ClientException`

Get the list of nodes that are not healthy

* **Returns:** The array of nodes that are not healthy.

## `public LedgerNanoStatus getLedgerNanoStatus(boolean isSimulator) throws ClientException`

Get the status of the Ledger Nano device

* **Parameters:** `isSimulator` — true if you want to use the simulator, false if you want to use the real device.
* **Returns:** LedgerNanoStatus

## `public PreparedTransactionData prepareTransaction(SecretManager secretManager, BuildBlockOptions buildBlockOptions) throws ClientException`

Prepares a transaction for signing

* **Parameters:**
    * `secretManager` — The secret manager that holds the secret for the account.
    * `buildBlockOptions` — This is the object that contains the transaction details.
* **Returns:** The prepared transaction.

## `public TransactionPayload signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) throws ClientException`

Signs the given prepared transaction using the secret manager

* **Parameters:**
    * `secretManager` — The secret manager that holds the secret key for the account that will sign the transaction.
    * `preparedTransactionData` — The transaction data that was prepared by the prepareTransaction method.
* **Returns:** The signed transaction.

## `public void storeMnemonic(SecretManager secretManager, String mnemonic) throws ClientException`

Stores the mnemonic in the secret manager

* **Parameters:**
    * `secretManager` — The secret manager to use.
    * `mnemonic` — The mnemonic to store.

## `public Map.Entry<BlockId, Block> postBlockPayload(BlockPayload payload) throws ClientException`

Automatically builds a block containing the given block payload and broadcasts it to the network.

* **Parameters:** `payload` — The payload to be posted.
* **Returns:** A map entry of the posted block id and the block.

## `public String getTokenSupply() throws ClientException`

Returns the token supply

* **Returns:** The token supply.

## `public String getProtocolParameters() throws ClientException`

Returns the protocol parameters.

* **Returns:** The protocol parameters.