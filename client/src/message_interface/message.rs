// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_types::block::{
    address::AliasAddress,
    output::{
        dto::{AliasIdDto, NativeTokenDto, NftIdDto, TokenSchemeDto},
        feature::dto::FeatureDto,
        unlock_condition::dto::UnlockConditionDto,
        AliasId, FoundryId, NftId, OutputId,
    },
    payload::{
        dto::PayloadDto,
        milestone::MilestoneId,
        transaction::{
            dto::{TransactionEssenceDto, TransactionPayloadDto},
            TransactionId,
        },
    },
    BlockDto, BlockId,
};
use serde::Deserialize;

#[cfg(feature = "mqtt")]
use crate::mqtt::Topic;
use crate::{
    api::{
        ClientBlockBuilderOptions as BuildBlockOptions, GetAddressesBuilderOptions as GenerateAddressesOptions,
        PreparedTransactionDataDto, RemainderDataDto,
    },
    node_api::indexer::query_parameters::QueryParameter,
    node_manager::node::NodeAuth,
    secret::{types::InputSigningDataDto, SecretManagerDto},
};

/// Each public client method.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
pub enum Message {
    /// Build an AliasOutput.
    /// Expected response: [`BuiltOutput`](crate::message_interface::Response::BuiltOutput)
    #[allow(missing_docs)]
    BuildAliasOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        #[serde(rename = "nativeTokens")]
        native_tokens: Option<Vec<NativeTokenDto>>,
        #[serde(rename = "aliasId")]
        alias_id: AliasIdDto,
        #[serde(rename = "stateIndex")]
        state_index: Option<u32>,
        #[serde(rename = "stateMetadata")]
        state_metadata: Option<String>,
        #[serde(rename = "foundryCounter")]
        foundry_counter: Option<u32>,
        #[serde(rename = "unlockConditions")]
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        #[serde(rename = "immutableFeatures")]
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Build a BasicOutput.
    /// Expected response: [`BuiltOutput`](crate::message_interface::Response::BuiltOutput)
    #[allow(missing_docs)]
    BuildBasicOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        #[serde(rename = "nativeTokens")]
        native_tokens: Option<Vec<NativeTokenDto>>,
        #[serde(rename = "unlockConditions")]
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
    },
    /// Build a FoundryOutput.
    /// Expected response: [`BuiltOutput`](crate::message_interface::Response::BuiltOutput)
    #[allow(missing_docs)]
    BuildFoundryOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        #[serde(rename = "nativeTokens")]
        native_tokens: Option<Vec<NativeTokenDto>>,
        #[serde(rename = "serialNumber")]
        serial_number: u32,
        #[serde(rename = "tokenScheme")]
        token_scheme: TokenSchemeDto,
        #[serde(rename = "unlockConditions")]
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        #[serde(rename = "immutableFeatures")]
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Build an NftOutput.
    /// Expected response: [`BuiltOutput`](crate::message_interface::Response::BuiltOutput)
    #[allow(missing_docs)]
    BuildNftOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        #[serde(rename = "nativeTokens")]
        native_tokens: Option<Vec<NativeTokenDto>>,
        #[serde(rename = "nftId")]
        nft_id: NftIdDto,
        #[serde(rename = "unlockConditions")]
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        #[serde(rename = "immutableFeatures")]
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Removes all listeners for the provided topics.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    ClearListeners {
        /// Topics for which listeners should be removed.
        topics: Vec<Topic>,
    },
    /// Generate addresses.
    GenerateAddresses {
        /// Create secret manager from json
        #[serde(rename = "secretManager")]
        secret_manager: SecretManagerDto,
        /// Addresses generation options
        options: GenerateAddressesOptions,
    },
    /// Build and post a block
    BuildAndPostBlock {
        /// Secret manager
        #[serde(rename = "secretManager")]
        secret_manager: Option<SecretManagerDto>,
        /// Options
        options: Option<BuildBlockOptions>,
    },
    /// Get a node candidate from the healthy node pool.
    GetNode,
    /// Gets the network related information such as network_id and min_pow_score
    GetNetworkInfo,
    /// Gets the network id of the node we're connecting to.
    GetNetworkId,
    /// Returns the bech32_hrp
    GetBech32Hrp,
    /// Returns the min pow score
    GetMinPowScore,
    /// Returns the tips interval
    GetTipsInterval,
    /// Returns the protocol parameters
    GetProtocolParameters,
    /// Returns if local pow should be used or not
    GetLocalPow,
    /// Get fallback to local proof of work timeout
    GetFallbackToLocalPow,
    /// Returns the unhealthy nodes.
    #[cfg(not(target_family = "wasm"))]
    UnhealthyNodes,
    /// Get the ledger status
    /// Expected response: [`LedgerNanoStatus`](crate::message_interface::Response::LedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    GetLedgerNanoStatus {
        /// To use a Ledger Speculos simulator, pass `true` to `is_simulator`; `false` otherwise.
        #[serde(rename = "isSimulator")]
        is_simulator: bool,
    },
    /// Prepare a transaction for signing
    PrepareTransaction {
        /// Secret manager
        #[serde(rename = "secretManager")]
        secret_manager: Option<SecretManagerDto>,
        /// Options
        options: Option<BuildBlockOptions>,
    },
    /// Sign a transaction
    SignTransaction {
        /// Secret manager
        #[serde(rename = "secretManager")]
        secret_manager: SecretManagerDto,
        /// Prepared transaction data
        #[serde(rename = "preparedTransactionData")]
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Create a single Signature Unlock.
    SignatureUnlock {
        /// Secret manager
        #[serde(rename = "secretManager")]
        secret_manager: SecretManagerDto,
        /// Input Signing Data
        // This field is boxed to not inflate the enum's size.
        #[serde(rename = "inputSigningData")]
        input_signing_data: Box<InputSigningDataDto>,
        /// Transaction Essence Hash
        #[serde(rename = "transactionEssenceHash")]
        transaction_essence_hash: Vec<u8>,
        /// Metadata for Ledger Nano signing
        #[serde(rename = "remainderData")]
        remainder_data: Option<RemainderDataDto>,
    },
    /// Store a mnemonic in the Stronghold vault
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    StoreMnemonic {
        /// Stronghold secret manager
        #[serde(rename = "secretManager")]
        secret_manager: SecretManagerDto,
        /// Mnemonic
        mnemonic: String,
    },
    /// Build a block containing the specified payload and post it to the network.
    PostBlockPayload {
        /// The payload to send
        #[serde(rename = "payload")]
        payload_dto: PayloadDto,
    },
    //////////////////////////////////////////////////////////////////////
    // Node core API
    //////////////////////////////////////////////////////////////////////
    /// Get health
    GetHealth {
        /// Url
        url: String,
    },
    /// Get node info
    GetNodeInfo {
        /// Url
        url: String,
        /// Node authentication
        auth: Option<NodeAuth>,
    },
    /// Returns the node information together with the url of the used node
    GetInfo,
    /// Get peers
    GetPeers,
    /// Get tips
    GetTips,
    /// Post block (JSON)
    PostBlock {
        /// Block
        block: BlockDto,
    },
    /// Post block (raw)
    PostBlockRaw {
        /// Block
        #[serde(rename = "blockBytes")]
        block_bytes: Vec<u8>,
    },
    /// Get block
    GetBlock {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
    },
    /// Get block metadata with block_id
    GetBlockMetadata {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
    },
    /// Get block raw
    GetBlockRaw {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
    },
    /// Get output
    GetOutput {
        /// Output ID
        #[serde(rename = "outputId")]
        output_id: OutputId,
    },
    /// Get output metadata
    GetOutputMetadata {
        /// Output ID
        #[serde(rename = "outputId")]
        output_id: OutputId,
    },
    /// Get the milestone by the given milestone id.
    GetMilestoneById {
        /// Milestone ID
        #[serde(rename = "milestoneId")]
        milestone_id: MilestoneId,
    },
    /// Get the raw milestone by the given milestone id.
    GetMilestoneByIdRaw {
        /// Milestone ID
        #[serde(rename = "milestoneId")]
        milestone_id: MilestoneId,
    },
    /// Get the milestone by the given index.
    GetMilestoneByIndex {
        /// Milestone Index
        index: u32,
    },
    /// Get the raw milestone by the given index.
    GetMilestoneByIndexRaw {
        /// Milestone Index
        index: u32,
    },
    /// Get the UTXO changes by the given milestone id.
    GetUtxoChangesById {
        /// Milestone ID
        #[serde(rename = "milestoneId")]
        milestone_id: MilestoneId,
    },
    /// Get the UTXO changes by the given milestone index.
    GetUtxoChangesByIndex {
        /// Milestone Index
        index: u32,
    },
    /// Get all receipts.
    GetReceipts,
    /// Get the receipts by the given milestone index.
    GetReceiptsMigratedAt {
        /// Milestone index
        #[serde(rename = "milestoneIndex")]
        milestone_index: u32,
    },
    /// Get the treasury output.
    GetTreasury,
    /// Returns the included block of the transaction.
    GetIncludedBlock {
        /// Transaction ID
        #[serde(rename = "transactionId")]
        transaction_id: TransactionId,
    },
    /// Returns the included block metadata of the transaction.
    GetIncludedBlockMetadata {
        /// Transaction ID
        #[serde(rename = "transactionId")]
        transaction_id: TransactionId,
    },

    //////////////////////////////////////////////////////////////////////
    // Node indexer API
    //////////////////////////////////////////////////////////////////////
    /// Fetch basic output IDs
    BasicOutputIds {
        /// Query parameters for output requests
        #[serde(rename = "queryParameters")]
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch alias output IDs
    AliasOutputIds {
        /// Query parameters for output requests
        #[serde(rename = "queryParameters")]
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch alias output ID
    AliasOutputId {
        /// Alias id
        #[serde(rename = "aliasId")]
        alias_id: AliasId,
    },
    /// Fetch NFT output IDs
    NftOutputIds {
        /// Query parameters for output requests
        #[serde(rename = "queryParameters")]
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch NFT output ID
    NftOutputId {
        /// NFT ID
        #[serde(rename = "nftId")]
        nft_id: NftId,
    },
    /// Fetch foundry Output IDs
    FoundryOutputIds {
        /// Query parameters for output requests
        #[serde(rename = "queryParameters")]
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch foundry Output ID
    FoundryOutputId {
        /// Foundry ID
        #[serde(rename = "foundryId")]
        foundry_id: FoundryId,
    },

    //////////////////////////////////////////////////////////////////////
    // High level API
    //////////////////////////////////////////////////////////////////////
    /// Fetch OutputWithMetadataResponse from provided OutputIds (requests are sent in parallel)
    GetOutputs {
        /// Output IDs
        #[serde(rename = "outputIds")]
        output_ids: Vec<OutputId>,
    },
    /// Try to get OutputWithMetadataResponse from provided OutputIds (requests are sent in parallel and errors are
    /// ignored, can be useful for spent outputs)
    TryGetOutputs {
        /// Output IDs
        #[serde(rename = "outputIds")]
        output_ids: Vec<OutputId>,
    },
    /// Find all blocks by provided block IDs.
    FindBlocks {
        /// BlockIDs
        #[serde(rename = "blockIds")]
        block_ids: Vec<BlockId>,
    },
    /// Retries (promotes or reattaches) a block for provided block id. Block should only be
    /// retried only if they are valid and haven't been confirmed for a while.
    Retry {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
    },
    /// Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
    /// milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
    /// position and additional reattached blocks
    RetryUntilIncluded {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
        /// Interval
        interval: Option<u64>,
        /// Maximum attempts
        #[serde(rename = "maxAttempts")]
        max_attempts: Option<u64>,
    },
    /// Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
    /// Returns the address to which the funds got consolidated, if any were available
    ConsolidateFunds {
        /// Secret manager
        #[serde(rename = "secretManager")]
        secret_manager: SecretManagerDto,
        /// Addresses generation options
        #[serde(rename = "generateAddressesOptions")]
        generate_addresses_options: GenerateAddressesOptions,
    },
    /// Function to find inputs from addresses for a provided amount (useful for offline signing)
    FindInputs {
        /// Addresses
        addresses: Vec<String>,
        /// Amount
        amount: u64,
    },
    /// Find all outputs based on the requests criteria. This method will try to query multiple nodes if
    /// the request amount exceeds individual node limit.
    FindOutputs {
        /// Output IDs
        #[serde(rename = "outputIds")]
        output_ids: Vec<OutputId>,
        /// Addresses
        addresses: Vec<String>,
    },
    /// Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    Reattach {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
    },
    /// Reattach a block without checking if it should be reattached
    ReattachUnchecked {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
    },
    /// Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
    /// method should error out and should not allow unnecessary promotions.
    Promote {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
    },
    /// Promote a block without checking if it should be promoted
    PromoteUnchecked {
        /// Block ID
        #[serde(rename = "blockId")]
        block_id: BlockId,
    },

    //////////////////////////////////////////////////////////////////////
    // Utils
    //////////////////////////////////////////////////////////////////////
    /// Transforms bech32 to hex
    Bech32ToHex {
        /// Bech32 encoded address
        bech32: String,
    },
    /// Transforms a hex encoded address to a bech32 encoded address
    HexToBech32 {
        /// Hex encoded bech32 address
        hex: String,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: Option<String>,
    },
    /// Transforms an alias id to a bech32 encoded address
    AliasIdToBech32 {
        /// Alias ID
        #[serde(rename = "aliasId")]
        alias_id: AliasId,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: Option<String>,
    },
    /// Transforms an nft id to a bech32 encoded address
    NftIdToBech32 {
        /// Nft ID
        #[serde(rename = "nftId")]
        nft_id: NftId,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: Option<String>,
    },
    /// Transforms a hex encoded public key to a bech32 encoded address
    HexPublicKeyToBech32Address {
        /// Hex encoded public key
        hex: String,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: Option<String>,
    },
    /// Returns a valid Address parsed from a String.
    ParseBech32Address {
        /// Address
        address: String,
    },
    /// Checks if a String is a valid bech32 encoded address.
    IsAddressValid {
        /// Address
        address: String,
    },
    /// Generates a new mnemonic.
    GenerateMnemonic,
    /// Returns a hex encoded seed for a mnemonic.
    MnemonicToHexSeed {
        /// Mnemonic
        mnemonic: String,
    },
    /// Returns a block ID (Blake2b256 hash of block bytes) from a block
    BlockId {
        /// Block
        block: BlockDto,
    },
    /// Returns the transaction ID (Blake2b256 hash of the provided transaction payload)
    TransactionId {
        /// Transaction Payload
        payload: TransactionPayloadDto,
    },
    /// Computes the alias ID
    ComputeAliasId {
        /// Output ID
        #[serde(rename = "outputId")]
        output_id: OutputId,
    },
    /// Computes the NFT ID
    ComputeNftId {
        /// Output ID
        #[serde(rename = "outputId")]
        output_id: OutputId,
    },
    /// Computes the Foundry ID
    ComputeFoundryId {
        /// Alias address
        #[serde(rename = "aliasAddress")]
        alias_address: AliasAddress,
        /// Serial number
        #[serde(rename = "serialNumber")]
        serial_number: u32,
        /// Token scheme kind
        #[serde(rename = "tokenSchemeKind")]
        token_scheme_kind: u8,
    },
    /// Requests funds for a given address from the faucet.
    Faucet {
        /// Faucet URL
        url: String,
        /// The address for request funds
        address: String,
    },
    /// Compute the hash of a transaction essence.
    HashTransactionEssence {
        /// The transaction essence
        essence: TransactionEssenceDto,
    },
}
