// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use crate::{
    api::PreparedTransactionData, builder::NetworkInfo, node_api::high_level::AddressBalance, node_manager::node::Node,
    Error, NodeInfoWrapper,
};

use bee_message::{address::Address, input::UtxoInput, output::OutputId, Message, MessageId};
use bee_rest_api::types::{
    dtos::{PeerDto, ReceiptDto},
    responses::{
        InfoResponse as NodeInfo, MessageMetadataResponse, MilestoneResponse, OutputResponse, RentStructureResponse,
        TreasuryResponse, UtxoChangesResponse as MilestoneUTXOChanges,
    },
};

use serde::Serialize;

/// The response message.
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ResponseType<'a> {
    /// GenerateAddress response.
    GeneratedAddresses(Vec<String>),
    /// Generated message
    GeneratedMessage(PreparedTransactionData),
    /// Node
    Node(Node),
    // /// Proof of work provider
    // PoWProvider(ClientMiner),
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
    /// Rent structure
    RentStructure(RentStructureResponse),
    /// Is fallback to local proof of work enabled
    FallbackToLocalPoW(bool),
    /// returns the unsynced nodes.
    #[cfg(not(feature = "wasm"))]
    UnsynchedNodes(HashSet<&'a Node>),
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
    MessageData(Message),
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
    IncludedMessage(Message),
    /// Fetched output IDs
    OutputIds(Vec<OutputId>),
    /// Messages
    Messages(Vec<Message>),
    /// Balance
    Balance(u64),
    /// Addresses balances
    AddressesBalances(Vec<AddressBalance>),
    /// Retry
    RetrySuccessful((MessageId, Message)),
    /// Retry until included
    RetryUntilIncludedSuccessful(Vec<(MessageId, Message)>),
    /// Consolidated funds
    ConsolidatedFunds(String),
    /// Found inputs
    Inputs(Vec<UtxoInput>),
    /// Reattach
    Reattached((MessageId, Message)),
    /// Promoted
    Promoted((MessageId, Message)),
    /// Bech32 to hex
    Bech32ToHex(String),
    /// Hex to bech32
    HexToBech32(String),
    /// Parsed bech32 address
    ParsedBech32Address(Address),
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
