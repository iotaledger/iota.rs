// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Range;

use bee_message::{
    input::UtxoInput,
    output::{AliasId, FoundryId, NftId, OutputId},
    payload::{dto::PayloadDto, transaction::TransactionId},
    MessageDto, MessageId,
};
use serde::Deserialize;

use crate::{
    api::{
        ClientMessageBuilderOptions as GenerateMessageOptions, GetAddressesBuilderOptions as GenerateAddressesOptions,
        PreparedTransactionDataDto,
    },
    node_api::indexer::query_parameters::QueryParameter,
    node_manager::node::NodeAuth,
};

/// Each public client method.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "name", content = "data")]
pub enum ClientMethod {
    /// Generate a addresses.
    GenerateAddresses {
        /// Create secret manager from json; alias "signer" for compatibility
        #[serde(alias = "signer")]
        secmngr: String,
        /// Addresses generation options
        options: GenerateAddressesOptions,
    },
    /// Generate client message
    GenerateMessage {
        /// Secret manager; alias "signer" for compatibility
        #[serde(alias = "signer")]
        secmngr: Option<String>,
        /// Options
        options: Option<GenerateMessageOptions>,
    },
    /// Get a node candidate from the synced node pool.
    GetNode,
    // /// Gets the miner to use based on the PoW setting
    // GetPoWProvider,
    /// Gets the network related information such as network_id and min_pow_score
    GetNetworkInfo,
    /// Gets the network id of the node we're connecting to.
    GetNetworkId,
    /// Returns the bech32_hrp
    GetBech32Hrp,
    /// Returns the min pow score
    GetMinPoWScore,
    /// Returns the tips interval
    GetTipsInterval,
    /// Returns if local pow should be used or not
    GetLocalPoW,
    /// Get fallback to local proof of work timeout
    GetFallbackToLocalPoW,
    /// returns the unsynced nodes.
    #[cfg(not(feature = "wasm"))]
    UnsyncedNodes,
    /// Prepare a transaction for signing
    PrepareTransaction {
        /// Signer
        signer: Option<String>,
        /// Options
        options: Option<GenerateMessageOptions>,
    },
    /// Sign a transaction
    SignTransaction {
        /// Signer
        signer: String,
        /// Prepared transaction data
        #[serde(rename = "preparedTransactionData")]
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Submit a payload in a message
    SubmitPayload {
        /// The payload to send
        #[serde(rename = "payload")]
        payload_dto: PayloadDto,
    },
    //////////////////////////////////////////////////////////////////////
    // Node core API
    //////////////////////////////////////////////////////////////////////
    /// Get node health
    GetNodeHealth {
        /// Url
        url: String,
    },
    /// Get health
    GetHealth,
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
    /// Post message
    PostMessage {
        /// Message
        message: MessageDto,
    },
    /// Post message json
    PostMessageJson {
        /// Message
        message: MessageDto,
    },
    /// Get message data
    GetMessageData {
        /// Message ID
        message_id: MessageId,
    },
    /// Get message metadata with message_id
    GetMessageMetadata {
        /// Message ID
        message_id: MessageId,
    },
    /// Get message raw
    GetMessageRaw {
        /// Message ID
        message_id: MessageId,
    },
    /// Get message children
    GetMessageChildren {
        /// Message ID
        message_id: MessageId,
    },
    /// Get output
    GetOutput {
        /// Output ID
        output_id: OutputId,
    },
    /// Get the milestone by the given index.
    GetMilestone {
        /// Index
        index: u32,
    },
    /// Get the milestone by the given index.
    GetMilestoneUtxoChanges {
        /// Index
        index: u32,
    },
    /// Get all receipts.
    GetReceipts,
    /// Get the receipts by the given milestone index.
    GetReceiptsMigratedAt {
        /// Milestone index
        milestone_index: u32,
    },
    /// Get the treasury output.
    GetTreasury,
    /// Returns the included message of the transaction.
    GetIncludedMessage {
        /// Transaction ID
        transaction_id: TransactionId,
    },

    //////////////////////////////////////////////////////////////////////
    // Node indexer API
    //////////////////////////////////////////////////////////////////////
    /// Fetch output IDs
    OutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch aliases output IDs
    AliasesOutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch alias output ID
    AliasOutputId {
        /// Alias id
        alias_id: AliasId,
    },
    /// Fetch NFTs output IDs
    NftsOutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch NFT output ID
    NftOutputId {
        /// NFT ID
        nft_id: NftId,
    },
    /// Fetch Foundries Output IDs
    FoundriesOutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch Foundry Output ID
    FoundryOutputId {
        /// Foundry ID
        foundry_id: FoundryId,
    },

    //////////////////////////////////////////////////////////////////////
    // High level API
    //////////////////////////////////////////////////////////////////////
    /// Fetch OutputResponse from provided OutputIds (requests are sent in parallel)
    GetOutputs {
        /// Output IDs
        output_ids: Vec<OutputId>,
    },
    /// Try to get OutputResponse from provided OutputIds (requests are sent in parallel and errors are ignored, can be
    /// useful for spent outputs)
    TryGetOutputs {
        /// Output IDs
        output_ids: Vec<OutputId>,
    },
    /// Find all messages by provided message IDs.
    FindMessages {
        /// MessageIDs
        message_ids: Vec<MessageId>,
    },
    /// Retries (promotes or reattaches) a message for provided message id. Message should only be
    /// retried only if they are valid and haven't been confirmed for a while.
    Retry {
        /// Message ID
        message_id: MessageId,
    },
    /// Retries (promotes or reattaches) a message for provided message id until it's included (referenced by a
    /// milestone). Default interval is 5 seconds and max attempts is 40. Returns the included message at first
    /// position and additional reattached messages
    RetryUntilIncluded {
        /// Message ID
        message_id: MessageId,
        /// Interval
        interval: Option<u64>,
        /// Maximum attempts
        max_attempts: Option<u64>,
    },
    /// Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
    /// Returns the address to which the funds got consolidated, if any were available
    ConsolidateFunds {
        /// Secret manager; alias "signer" for compatibility
        #[serde(alias = "signer")]
        secmngr: String,
        /// Account index
        account_index: u32,
        /// Address_range
        address_range: Range<u32>,
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
        /// UtxoInputs
        outputs: Vec<UtxoInput>,
        /// Addresses
        addresses: Vec<String>,
    },
    /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    Reattach {
        /// Message ID
        message_id: MessageId,
    },
    /// Reattach a message without checking if it should be reattached
    ReattachUnchecked {
        /// Message ID
        message_id: MessageId,
    },
    /// Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
    /// method should error out and should not allow unnecessary promotions.
    Promote {
        /// Message ID
        message_id: MessageId,
    },
    /// Promote a message without checking if it should be promoted
    PromoteUnchecked {
        /// Message ID
        message_id: MessageId,
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
        bech32_hrp: Option<String>,
    },
    /// Transforms a hex encoded public key to a bech32 encoded address
    HexPublicKeyToBech32Address {
        /// Hex encoded public key
        hex: String,
        /// Human readable part
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
}
