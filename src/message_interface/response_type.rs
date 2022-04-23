// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use bee_message::{address::dto::AddressDto, input::dto::UtxoInputDto, output::OutputId, MessageDto, MessageId};
use bee_rest_api::types::{
    dtos::{PeerDto, ReceiptDto},
    responses::{
        InfoResponse as NodeInfo, MessageMetadataResponse, MilestoneResponse, OutputResponse, TreasuryResponse,
        UtxoChangesResponse as MilestoneUTXOChanges,
    },
};
use serde::Serialize;

use crate::{
    api::PreparedTransactionData, builder::NetworkInfo, node_api::high_level::AddressBalance, node_manager::node::Node,
    Error, NodeInfoWrapper,
};

/// The response message.
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ResponseType {
    /// GenerateAddress response.
    GeneratedAddresses(Vec<String>),
    /// Generated message
    GeneratedMessage(MessageDto),
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
    /// returns the unsynced nodes.
    #[cfg(not(feature = "wasm"))]
    UnsyncedNodes(HashSet<Node>),
    /// Node health
    NodeHealth(bool),
    /// Node info
    NodeInfo(NodeInfo),
    /// Info
    Info(NodeInfoWrapper),
    /// Peers
    Peers(Vec<PeerDto>),
    /// Tips
    Tips(Vec<MessageId>),
    /// Posted message
    PostMessageSuccessful(MessageId),
    /// Message data
    MessageData(MessageDto),
    /// Message metadata
    MessageMetadata(MessageMetadataResponse),
    /// Message raw
    MessageRaw(String),
    /// Message children
    MessageChildren(Box<[MessageId]>),
    /// Get output successful
    Output(OutputResponse),
    /// Get outputs successful
    Outputs(Vec<OutputResponse>),
    /// Get milestone successful
    Milestone(MilestoneResponse),
    /// Get milestone utxo changes
    MilestoneUtxoChanges(MilestoneUTXOChanges),
    /// Get receipts successful
    Receipts(Vec<ReceiptDto>),
    /// Get receipts migrated at milestone
    ReceiptsMigratedAtMilestone(Vec<ReceiptDto>),
    /// Get treasury successful
    Treasury(TreasuryResponse),
    /// Get included message successful
    IncludedMessage(MessageDto),
    /// Fetched output ID
    OutputId(OutputId),
    /// Fetched output IDs
    OutputIds(Vec<OutputId>),
    /// Messages
    Messages(Vec<MessageDto>),
    /// Balance
    Balance(u64),
    /// Addresses balances
    AddressesBalances(Vec<AddressBalance>),
    /// Retry
    RetrySuccessful((MessageId, MessageDto)),
    /// Retry until included
    RetryUntilIncludedSuccessful(Vec<(MessageId, MessageDto)>),
    /// Consolidated funds
    ConsolidatedFunds(String),
    /// Found inputs
    Inputs(Vec<UtxoInputDto>),
    /// Reattach
    Reattached((MessageId, MessageDto)),
    /// Promoted
    Promoted((MessageId, MessageDto)),
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
    /// An error occurred.
    Error(Error),
    /// A panic occurred.
    Panic(String),
    /// All went fine.
    Ok(()),
}
