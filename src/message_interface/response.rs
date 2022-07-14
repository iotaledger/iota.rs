// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_family = "wasm"))]
use std::collections::HashSet;

use bee_block::{
    address::dto::AddressDto,
    input::dto::UtxoInputDto,
    output::{dto::OutputDto, AliasId, FoundryId, NftId, OutputId},
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

use crate::{api::PreparedTransactionDataDto, builder::NetworkInfo, node_manager::node::Node, Error, NodeInfoWrapper};

/// The response message.
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum Response {
    /// Response for:
    /// - [`BuildAliasOutput`](crate::message_interface::ClientMethod::BuildAliasOutput)
    /// - [`BuildBasicOutput`](crate::message_interface::ClientMethod::BuildBasicOutput)
    /// - [`BuildFoundryOutput`](crate::message_interface::ClientMethod::BuildFoundryOutput)
    /// - [`BuildNftOutput`](crate::message_interface::ClientMethod::BuildNftOutput)
    BuiltOutput(OutputDto),
    /// Response for:
    /// - [`GenerateAddresses`](crate::message_interface::ClientMethod::GenerateAddresses)
    GeneratedAddresses(Vec<String>),
    /// Response for:
    /// - [`GetNode`](crate::message_interface::ClientMethod::GetNode)
    Node(Node),
    /// Response for:
    /// - [`GetNetworkInfo`](crate::message_interface::ClientMethod::GetNetworkInfo)
    NetworkInfo(NetworkInfo),
    /// Response for:
    /// - [`GetNetworkId`](crate::message_interface::ClientMethod::GetNetworkId)
    NetworkId(u64),
    /// Response for:
    /// - [`GetBech32Hrp`](crate::message_interface::ClientMethod::GetBech32Hrp)
    Bech32Hrp(String),
    /// Response for:
    /// - [`GetMinPoWScore`](crate::message_interface::ClientMethod::GetMinPoWScore)
    MinPoWScore(f64),
    /// Response for:
    /// - [`GetTipsInterval`](crate::message_interface::ClientMethod::GetTipsInterval)
    TipsInterval(u64),
    /// Response for:
    /// - [`GetLocalPoW`](crate::message_interface::ClientMethod::GetLocalPoW)
    LocalPoW(bool),
    /// Response for:
    /// - [`GetFallbackToLocalPoW`](crate::message_interface::ClientMethod::GetFallbackToLocalPoW)
    FallbackToLocalPoW(bool),
    /// Response for:
    /// - [`PrepareTransaction`](crate::message_interface::ClientMethod::PrepareTransaction)
    PreparedTransactionData(PreparedTransactionDataDto),
    /// Response for:
    /// - [`SignTransaction`](crate::message_interface::ClientMethod::SignTransaction)
    SignedTransaction(PayloadDto),
    /// Response for:
    /// - [`UnsyncedNodes`](crate::message_interface::ClientMethod::UnsyncedNodes)
    UnsyncedNodes(HashSet<Node>),
    /// Response for:
    /// - [`GetHealth`](crate::message_interface::ClientMethod::GetHealth)
    Health(bool),
    /// Response for:
    /// - [`GetNodeInfo`](crate::message_interface::ClientMethod::GetNodeInfo)
    NodeInfo(NodeInfo),
    /// Response for:
    /// - [`GetInfo`](crate::message_interface::ClientMethod::GetInfo)
    Info(NodeInfoWrapper),
    /// Response for:
    /// - [`GetPeers`](crate::message_interface::ClientMethod::GetPeers)
    Peers(Vec<PeerDto>),
    /// Response for:
    /// - [`GetTips`](crate::message_interface::ClientMethod::GetTips)
    Tips(Vec<BlockId>),
    /// Response for:
    /// - [`GetBlock`](crate::message_interface::ClientMethod::GetBlock)
    /// - [`GenerateBlock`](crate::message_interface::ClientMethod::GenerateBlock)
    /// - [`GetIncludedBlock`](crate::message_interface::ClientMethod::GetIncludedBlock)
    Block(BlockDto),
    /// Response for:
    /// - [`PostBlockPayload`](crate::message_interface::ClientMethod::PostBlockPayload)
    /// - [`Retry`](crate::message_interface::ClientMethod::Retry)
    BlockIdWithBlock(BlockId, BlockDto),
    /// Response for:
    /// - [`GetBlockMetadata`](crate::message_interface::ClientMethod::GetBlockMetadata)
    BlockMetadata(BlockMetadataResponse),
    /// Response for:
    /// - [`GetBlockRaw`](crate::message_interface::ClientMethod::GetBlockRaw)
    BlockRaw(Vec<u8>),
    /// Response for:
    /// - [`GetOutput`](crate::message_interface::ClientMethod::GetOutput)
    Output(OutputResponse),
    /// Response for:
    /// - [`GetOutputMetadata`](crate::message_interface::ClientMethod::GetOutputMetadata)
    OutputMetadata(OutputMetadataResponse),
    /// Response for:
    /// - [`GetOutputs`](crate::message_interface::ClientMethod::GetOutputs)
    /// - [`TryGetOutputs`](crate::message_interface::ClientMethod::TryGetOutputs)
    /// - [`FindOutputs`](crate::message_interface::ClientMethod::FindOutputs)
    Outputs(Vec<OutputResponse>),
    /// Response for:
    /// - [`GetMilestoneById`](crate::message_interface::ClientMethod::GetMilestoneById)
    /// - [`GetMilestoneByIndex`](crate::message_interface::ClientMethod::GetMilestoneByIndex)
    Milestone(MilestonePayloadDto),
    /// Response for:
    /// - [`GetMilestoneByIdRaw`](crate::message_interface::ClientMethod::GetMilestoneByIdRaw)
    /// - [`GetMilestoneByIndexRaw`](crate::message_interface::ClientMethod::GetMilestoneByIndexRaw)
    MilestoneRaw(Vec<u8>),
    /// Response for:
    /// - [`GetUtxoChangesById`](crate::message_interface::ClientMethod::GetUtxoChangesById)
    /// - [`GetUtxoChangesByIndex`](crate::message_interface::ClientMethod::GetUtxoChangesByIndex)
    MilestoneUtxoChanges(MilestoneUTXOChanges),
    /// Response for:
    /// - [`GetReceipts`](crate::message_interface::ClientMethod::GetReceipts)
    /// - [`GetReceiptsMigratedAt`](crate::message_interface::ClientMethod::GetReceiptsMigratedAt)
    Receipts(Vec<ReceiptDto>),
    /// Response for:
    /// - [`GetTreasury`](crate::message_interface::ClientMethod::GetTreasury)
    Treasury(TreasuryResponse),
    /// Response for:
    /// - [`AliasOutputId`](crate::message_interface::ClientMethod::AliasOutputId)
    /// - [`NftOutputId`](crate::message_interface::ClientMethod::NftOutputId)
    /// - [`FoundryOutputId`](crate::message_interface::ClientMethod::FoundryOutputId)
    OutputId(OutputId),
    /// Response for:
    /// - [`BasicOutputIds`](crate::message_interface::ClientMethod::BasicOutputIds)
    /// - [`AliasOutputIds`](crate::message_interface::ClientMethod::AliasOutputIds)
    /// - [`NftOutputIds`](crate::message_interface::ClientMethod::NftOutputIds)
    /// - [`FoundryOutputIds`](crate::message_interface::ClientMethod::FoundryOutputIds)
    OutputIds(Vec<OutputId>),
    /// Response for:
    /// - [`FindBlocks`](crate::message_interface::ClientMethod::FindBlocks)
    Blocks(Vec<BlockDto>),
    /// Response for:
    /// - [`RetryUntilIncluded`](crate::message_interface::ClientMethod::RetryUntilIncluded)
    RetryUntilIncludedSuccessful(Vec<(BlockId, BlockDto)>),
    /// Response for:
    /// - [`ConsolidateFunds`](crate::message_interface::ClientMethod::ConsolidateFunds)
    ConsolidatedFunds(String),
    /// Response for:
    /// - [`FindInputs`](crate::message_interface::ClientMethod::FindInputs)
    Inputs(Vec<UtxoInputDto>),
    /// Response for:
    /// - [`Reattach`](crate::message_interface::ClientMethod::Reattach)
    /// - [`ReattachUnchecked`](crate::message_interface::ClientMethod::ReattachUnchecked)
    Reattached((BlockId, BlockDto)),
    /// Response for:
    /// - [`Promote`](crate::message_interface::ClientMethod::Promote)
    /// - [`PromoteUnchecked`](crate::message_interface::ClientMethod::PromoteUnchecked)
    Promoted((BlockId, BlockDto)),
    /// Response for:
    /// - [`Bech32ToHex`](crate::message_interface::ClientMethod::Bech32ToHex)
    Bech32ToHex(String),
    /// Response for:
    /// - [`HexToBech32`](crate::message_interface::ClientMethod::HexToBech32)
    /// - [`HexPublicKeyToBech32Address`](crate::message_interface::ClientMethod::HexPublicKeyToBech32Address)
    HexToBech32(String),
    /// Response for:
    /// - [`ParseBech32Address`](crate::message_interface::ClientMethod::ParseBech32Address)
    ParsedBech32Address(AddressDto),
    /// Response for:
    /// - [`IsAddressValid`](crate::message_interface::ClientMethod::IsAddressValid)
    IsAddressValid(bool),
    /// Response for:
    /// - [`GenerateMnemonic`](crate::message_interface::ClientMethod::GenerateMnemonic)
    GeneratedMnemonic(String),
    /// Response for:
    /// - [`MnemonicToHexSeed`](crate::message_interface::ClientMethod::MnemonicToHexSeed)
    MnemonicHexSeed(String),
    /// Response for:
    /// - [`BlockId`](crate::message_interface::ClientMethod::BlockId)
    /// - [`PostBlock`](crate::message_interface::ClientMethod::PostBlock)
    /// - [`PostBlockRaw`](crate::message_interface::ClientMethod::PostBlockRaw)
    BlockId(BlockId),
    /// Response for:
    /// - [`TransactionId`](crate::message_interface::ClientMethod::TransactionId)
    TransactionId(TransactionId),
    /// Response for:
    /// - [`ComputeAliasId`](crate::message_interface::ClientMethod::ComputeAliasId)
    AliasId(AliasId),
    /// Response for:
    /// - [`ComputeNftId`](crate::message_interface::ClientMethod::ComputeNftId)
    NftId(NftId),
    /// Response for:
    /// - [`ComputeFoundryId`](crate::message_interface::ClientMethod::ComputeFoundryId)
    FoundryId(FoundryId),
    /// Response for:
    /// - [`Faucet`](crate::message_interface::ClientMethod::Faucet)
    Faucet(String),
    /// Response for:
    /// - [`StoreMnemonic`](crate::message_interface::ClientMethod::StoreMnemonic)
    Ok,
    /// Response for any method that returns an error.
    Error(Error),
    /// Response for any method that panics.
    Panic(String),
}
