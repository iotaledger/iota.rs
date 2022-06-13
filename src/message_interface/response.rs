// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_family = "wasm"))]
use std::collections::HashSet;

use bee_block::{
    address::dto::AddressDto,
    input::dto::UtxoInputDto,
    output::{AliasId, FoundryId, NftId, OutputDto, OutputId},
    payload::{
        dto::{MilestonePayloadDto, PayloadDto},
        transaction::TransactionId,
    },
    BlockDto, BlockId,
};
use bee_rest_api::types::{
    dtos::{PeerDto, ReceiptDto},
    responses::{
        BlockMetadataResponse, InfoResponse as NodeInfo, OutputMetadataResponse, OutputResponse, TreasuryResponse,
        UtxoChangesResponse as MilestoneUTXOChanges,
    },
};
use serde::Serialize;

use crate::{
    api::PreparedTransactionDataDto, builder::NetworkInfo, node_api::high_level::AddressBalance,
    node_manager::node::Node, Error, NodeInfoWrapper,
};

/// The response message.
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum Response {
    /// Response for
    /// [`BuildAliasOutput`](crate::message_interface::ClientMethod::BuildAliasOutput)
    /// [`BuildBasicOutput`](crate::message_interface::ClientMethod::BuildBasicOutput)
    /// [`BuildFoundryOutput`](crate::message_interface::ClientMethod::BuildFoundryOutput)
    /// [`BuildNftOutput`](crate::message_interface::ClientMethod::BuildNftOutput)
    BuiltOutput(OutputDto),
    /// GenerateAddress response.
    GeneratedAddresses(Vec<String>),
    /// Generated block
    GeneratedBlock(BlockDto),
    /// Node
    Node(Node),
    /// Network info
    NetworkInfo(NetworkInfo),
    /// Network ID
    NetworkId(u64),
    /// Protocol version
    ProtocolVersion(u8),
    /// Bech32 human readable part
    Bech32Hrp(String),
    /// Min proof of work score
    MinPoWScore(f64),
    /// tips interval
    TipsInterval(u64),
    /// Is local proof of work used
    LocalPoW(bool),
    /// Is fallback to local proof of work enabled
    FallbackToLocalPoW(bool),
    /// Prepared transaction data for signing
    PreparedTransactionData(PreparedTransactionDataDto),
    /// Signed transaction data for signing
    SignedTransaction(PayloadDto),
    /// returns the unsynced nodes.
    #[cfg(not(target_family = "wasm"))]
    UnsyncedNodes(HashSet<Node>),
    /// Health
    Health(bool),
    /// Node info
    NodeInfo(NodeInfo),
    /// Info
    Info(NodeInfoWrapper),
    /// Peers
    Peers(Vec<PeerDto>),
    /// Tips
    Tips(Vec<BlockId>),
    /// Posted block
    PostBlockSuccessful(BlockId),
    /// Block
    Block(BlockDto),
    /// Block metadata
    BlockMetadata(BlockMetadataResponse),
    /// Block raw
    BlockRaw(Vec<u8>),
    /// Get output successful
    Output(OutputResponse),
    /// Get the metadata of an output
    OutputMetadata(OutputMetadataResponse),
    /// Get outputs successful
    Outputs(Vec<OutputResponse>),
    /// Get milestone successful
    Milestone(MilestonePayloadDto),
    /// Get the milestone in raw bytes
    MilestoneRaw(Vec<u8>),
    /// Get milestone utxo changes
    MilestoneUtxoChanges(MilestoneUTXOChanges),
    /// Get receipts successful
    Receipts(Vec<ReceiptDto>),
    /// Get receipts migrated at milestone
    ReceiptsMigratedAtMilestone(Vec<ReceiptDto>),
    /// Get treasury successful
    Treasury(TreasuryResponse),
    /// Get included block successful
    IncludedBlock(BlockDto),
    /// Fetched output ID
    OutputId(OutputId),
    /// Fetched output IDs
    OutputIds(Vec<OutputId>),
    /// Blocks
    Blocks(Vec<BlockDto>),
    /// Balance
    Balance(u64),
    /// Addresses balances
    AddressesBalances(Vec<AddressBalance>),
    /// Retry
    RetrySuccessful((BlockId, BlockDto)),
    /// Retry until included
    RetryUntilIncludedSuccessful(Vec<(BlockId, BlockDto)>),
    /// Consolidated funds
    ConsolidatedFunds(String),
    /// Found inputs
    Inputs(Vec<UtxoInputDto>),
    /// Reattach
    Reattached((BlockId, BlockDto)),
    /// Promoted
    Promoted((BlockId, BlockDto)),
    /// Bech32 to hex
    Bech32ToHex(String),
    /// Hex to bech32
    HexToBech32(String),
    /// Parsed bech32 address
    ParsedBech32Address(AddressDto),
    /// Is address valid
    IsAddressValid(bool),
    /// Generated mnemonic
    GeneratedMnemonic(String),
    /// Mnemonic to hex encoded seed
    MnemonicHexSeed(String),
    /// The BLAKE2b-256 hash of the block bytes
    BlockId(BlockId),
    /// The BLAKE2b-256 hash of the block bytes
    TransactionId(TransactionId),
    /// The BLAKE2b-256 hash of the block bytes
    AliasId(AliasId),
    /// The BLAKE2b-256 hash of the block bytes
    NftId(NftId),
    /// The BLAKE2b-256 hash of the block bytes
    FoundryId(FoundryId),
    /// The response from the faucet
    Faucet(String),
    /// An error occurred.
    Error(Error),
    /// A panic occurred.
    Panic(String),
    /// All went fine.
    Ok(()),
}
