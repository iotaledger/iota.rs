// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_family = "wasm"))]
use std::collections::HashSet;

use iota_types::{
    api::{
        core::{
            dto::{PeerDto, ReceiptDto},
            response::{
                BlockMetadataResponse, InfoResponse as NodeInfo, OutputWithMetadataResponse, TreasuryResponse,
                UtxoChangesResponse as MilestoneUTXOChanges,
            },
        },
        plugins::indexer::OutputIdsResponse,
    },
    block::{
        address::dto::AddressDto,
        input::dto::UtxoInputDto,
        output::{
            dto::{OutputDto, OutputMetadataDto},
            AliasId, FoundryId, NftId, OutputId,
        },
        payload::{
            dto::{MilestonePayloadDto, PayloadDto},
            transaction::TransactionId,
        },
        protocol::dto::ProtocolParametersDto,
        unlock::dto::UnlockDto,
        BlockDto, BlockId,
    },
};
use serde::Serialize;

#[cfg(feature = "ledger_nano")]
use crate::secret::LedgerNanoStatus;
use crate::{api::PreparedTransactionDataDto, node_manager::node::Node, Error, NetworkInfoDto, NodeInfoWrapper};

/// The response message.
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Response {
    /// Response for:
    /// - [`BuildAliasOutput`](crate::message_interface::Message::BuildAliasOutput)
    /// - [`BuildBasicOutput`](crate::message_interface::Message::BuildBasicOutput)
    /// - [`BuildFoundryOutput`](crate::message_interface::Message::BuildFoundryOutput)
    /// - [`BuildNftOutput`](crate::message_interface::Message::BuildNftOutput)
    BuiltOutput(OutputDto),
    /// Response for:
    /// - [`GenerateAddresses`](crate::message_interface::Message::GenerateAddresses)
    GeneratedAddresses(Vec<String>),
    /// Response for:
    /// - [`GetNode`](crate::message_interface::Message::GetNode)
    Node(Node),
    /// Response for:
    /// - [`GetNetworkInfo`](crate::message_interface::Message::GetNetworkInfo)
    NetworkInfo(NetworkInfoDto),
    /// Response for:
    /// - [`GetNetworkId`](crate::message_interface::Message::GetNetworkId)
    NetworkId(u64),
    /// Response for:
    /// - [`GetBech32Hrp`](crate::message_interface::Message::GetBech32Hrp)
    Bech32Hrp(String),
    /// Response for:
    /// - [`GetMinPowScore`](crate::message_interface::Message::GetMinPowScore)
    MinPowScore(u32),
    /// Response for:
    /// - [`GetTipsInterval`](crate::message_interface::Message::GetTipsInterval)
    TipsInterval(u64),
    /// Response for:
    /// - [`GetProtocolParameters`](crate::message_interface::Message::GetProtocolParameters)
    ProtocolParameters(ProtocolParametersDto),
    /// Response for:
    /// - [`GetLocalPow`](crate::message_interface::Message::GetLocalPow)
    LocalPow(bool),
    /// Response for:
    /// - [`GetFallbackToLocalPow`](crate::message_interface::Message::GetFallbackToLocalPow)
    FallbackToLocalPow(bool),
    /// Response for:
    /// - [`GetLedgerNanoStatus`](crate::message_interface::Message::GetLedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNanoStatus(LedgerNanoStatus),
    /// Response for:
    /// - [`PrepareTransaction`](crate::message_interface::Message::PrepareTransaction)
    PreparedTransactionData(PreparedTransactionDataDto),
    /// Response for:
    /// - [`SignTransaction`](crate::message_interface::Message::SignTransaction)
    SignedTransaction(PayloadDto),
    /// Response for:
    /// - [`SignatureUnlock`](crate::message_interface::Message::SignatureUnlock)
    SignatureUnlock(UnlockDto),
    /// Response for:
    /// - [`UnhealthyNodes`](crate::message_interface::Message::UnhealthyNodes)
    #[cfg(not(target_family = "wasm"))]
    UnhealthyNodes(HashSet<Node>),
    /// Response for:
    /// - [`GetHealth`](crate::message_interface::Message::GetHealth)
    Health(bool),
    /// Response for:
    /// - [`GetNodeInfo`](crate::message_interface::Message::GetNodeInfo)
    NodeInfo(NodeInfo),
    /// Response for:
    /// - [`GetInfo`](crate::message_interface::Message::GetInfo)
    Info(NodeInfoWrapper),
    /// Response for:
    /// - [`GetPeers`](crate::message_interface::Message::GetPeers)
    Peers(Vec<PeerDto>),
    /// Response for:
    /// - [`GetTips`](crate::message_interface::Message::GetTips)
    Tips(Vec<BlockId>),
    /// Response for:
    /// - [`GetBlock`](crate::message_interface::Message::GetBlock)
    /// - [`GetIncludedBlock`](crate::message_interface::Message::GetIncludedBlock)
    Block(BlockDto),
    /// Response for:
    /// - [`BuildAndPostBlock`](crate::message_interface::Message::BuildAndPostBlock)
    /// - [`PostBlockPayload`](crate::message_interface::Message::PostBlockPayload)
    /// - [`Retry`](crate::message_interface::Message::Retry)
    BlockIdWithBlock(BlockId, BlockDto),
    /// Response for:
    /// - [`GetBlockMetadata`](crate::message_interface::Message::GetBlockMetadata)
    BlockMetadata(BlockMetadataResponse),
    /// Response for:
    /// - [`GetBlockRaw`](crate::message_interface::Message::GetBlockRaw)
    BlockRaw(Vec<u8>),
    /// Response for:
    /// - [`GetOutput`](crate::message_interface::Message::GetOutput)
    Output(OutputWithMetadataResponse),
    /// Response for:
    /// - [`GetOutputMetadata`](crate::message_interface::Message::GetOutputMetadata)
    OutputMetadata(OutputMetadataDto),
    /// Response for:
    /// - [`GetOutputs`](crate::message_interface::Message::GetOutputs)
    /// - [`TryGetOutputs`](crate::message_interface::Message::TryGetOutputs)
    /// - [`FindOutputs`](crate::message_interface::Message::FindOutputs)
    Outputs(Vec<OutputWithMetadataResponse>),
    /// Response for:
    /// - [`GetMilestoneById`](crate::message_interface::Message::GetMilestoneById)
    /// - [`GetMilestoneByIndex`](crate::message_interface::Message::GetMilestoneByIndex)
    Milestone(MilestonePayloadDto),
    /// Response for:
    /// - [`GetMilestoneByIdRaw`](crate::message_interface::Message::GetMilestoneByIdRaw)
    /// - [`GetMilestoneByIndexRaw`](crate::message_interface::Message::GetMilestoneByIndexRaw)
    MilestoneRaw(Vec<u8>),
    /// Response for:
    /// - [`GetUtxoChangesById`](crate::message_interface::Message::GetUtxoChangesById)
    /// - [`GetUtxoChangesByIndex`](crate::message_interface::Message::GetUtxoChangesByIndex)
    MilestoneUtxoChanges(MilestoneUTXOChanges),
    /// Response for:
    /// - [`GetReceipts`](crate::message_interface::Message::GetReceipts)
    /// - [`GetReceiptsMigratedAt`](crate::message_interface::Message::GetReceiptsMigratedAt)
    Receipts(Vec<ReceiptDto>),
    /// Response for:
    /// - [`GetTreasury`](crate::message_interface::Message::GetTreasury)
    Treasury(TreasuryResponse),
    /// Response for:
    /// - [`AliasOutputId`](crate::message_interface::Message::AliasOutputId)
    /// - [`NftOutputId`](crate::message_interface::Message::NftOutputId)
    /// - [`FoundryOutputId`](crate::message_interface::Message::FoundryOutputId)
    OutputId(OutputId),
    /// Response for:
    /// - [`BasicOutputIds`](crate::message_interface::Message::BasicOutputIds)
    /// - [`AliasOutputIds`](crate::message_interface::Message::AliasOutputIds)
    /// - [`NftOutputIds`](crate::message_interface::Message::NftOutputIds)
    /// - [`FoundryOutputIds`](crate::message_interface::Message::FoundryOutputIds)
    OutputIdsResponse(OutputIdsResponse),
    /// Response for:
    /// - [`FindBlocks`](crate::message_interface::Message::FindBlocks)
    Blocks(Vec<BlockDto>),
    /// Response for:
    /// - [`RetryUntilIncluded`](crate::message_interface::Message::RetryUntilIncluded)
    RetryUntilIncludedSuccessful(Vec<(BlockId, BlockDto)>),
    /// Response for:
    /// - [`ConsolidateFunds`](crate::message_interface::Message::ConsolidateFunds)
    ConsolidatedFunds(String),
    /// Response for:
    /// - [`FindInputs`](crate::message_interface::Message::FindInputs)
    Inputs(Vec<UtxoInputDto>),
    /// Response for:
    /// - [`Reattach`](crate::message_interface::Message::Reattach)
    /// - [`ReattachUnchecked`](crate::message_interface::Message::ReattachUnchecked)
    Reattached((BlockId, BlockDto)),
    /// Response for:
    /// - [`Promote`](crate::message_interface::Message::Promote)
    /// - [`PromoteUnchecked`](crate::message_interface::Message::PromoteUnchecked)
    Promoted((BlockId, BlockDto)),
    /// Response for:
    /// - [`Bech32ToHex`](crate::message_interface::Message::Bech32ToHex)
    Bech32ToHex(String),
    /// Response for:
    /// - [`AliasIdToBech32`](crate::message_interface::Message::AliasIdToBech32)
    /// - [`HexPublicKeyToBech32Address`](crate::message_interface::Message::HexPublicKeyToBech32Address)
    /// - [`HexToBech32`](crate::message_interface::Message::HexToBech32)
    /// - [`NftIdToBech32`](crate::message_interface::Message::NftIdToBech32)
    Bech32Address(String),
    /// Response for:
    /// - [`ParseBech32Address`](crate::message_interface::Message::ParseBech32Address)
    ParsedBech32Address(AddressDto),
    /// Response for:
    /// - [`IsAddressValid`](crate::message_interface::Message::IsAddressValid)
    IsAddressValid(bool),
    /// Response for:
    /// - [`GenerateMnemonic`](crate::message_interface::Message::GenerateMnemonic)
    GeneratedMnemonic(String),
    /// Response for:
    /// - [`MnemonicToHexSeed`](crate::message_interface::Message::MnemonicToHexSeed)
    MnemonicHexSeed(String),
    /// Response for:
    /// - [`BlockId`](crate::message_interface::Message::BlockId)
    /// - [`PostBlock`](crate::message_interface::Message::PostBlock)
    /// - [`PostBlockRaw`](crate::message_interface::Message::PostBlockRaw)
    BlockId(BlockId),
    /// Response for:
    /// - [`TransactionId`](crate::message_interface::Message::TransactionId)
    TransactionId(TransactionId),
    /// Response for:
    /// - [`ComputeAliasId`](crate::message_interface::Message::ComputeAliasId)
    AliasId(AliasId),
    /// Response for:
    /// - [`ComputeNftId`](crate::message_interface::Message::ComputeNftId)
    NftId(NftId),
    /// Response for:
    /// - [`ComputeFoundryId`](crate::message_interface::Message::ComputeFoundryId)
    FoundryId(FoundryId),
    /// Response for:
    /// - [`Faucet`](crate::message_interface::Message::Faucet)
    Faucet(String),
    /// Response for:
    /// - [`HashTransactionEssence`](crate::message_interface::Message::HashTransactionEssence)
    TransactionEssenceHash(String),
    /// Response for:
    /// - [`ClearListeners`](crate::message_interface::Message::ClearListeners)
    /// - [`StoreMnemonic`](crate::message_interface::Message::StoreMnemonic)
    Ok,
    /// Response for any method that returns an error.
    Error(Error),
    /// Response for any method that panics.
    Panic(String),
}
