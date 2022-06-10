import type {
    __GetInfoPayloadMethod__,
    __GetBasicOutputIdsPayloadMethod__,
    __GetOutputPayloadMethod__,
    __GetOutputsPayloadMethod__,
    __GenerateMnemonicPayloadMethod__,
    __MnemonicToHexSeedPayloadMethod__,
    __GenerateAddressesPayloadMethod__,
    __PostBlockPayloadMethod__,
    __GenerateBlockPayloadMethod__,
    __GetTipsPayloadMethod__,
    __GetNetworkInfoPayloadMethod__,
    __GetBlockPayloadMethod__,
    __GetBlockMetadataPayloadMethod__,
    __FindInputsPayloadMethod__,
    __FindOutputsPayloadMethod__,
    __PrepareTransactionPayloadMethod__,
    __SignTransactionPayloadMethod__,
    __SubmitPayloadPayloadMethod__,
    __ParseBech32AddressPayloadMethod__,
    __BlockIdPayloadMethod__,
    __GetNodePayloadMethod__,
    __GetNetworkIdPayloadMethod__,
    __GetBech32HrpPayloadMethod__,
    __GetMinPowScorePayloadMethod__,
    __GetTipsIntervalPayloadMethod__,
    __GetLocalPowPayloadMethod__,
    __GetFallbackToLocalPowPayloadMethod__,
    __GetHealthPayloadMethod__,
    __GetNodeInfoPayloadMethod__,
    __GetPeersPayloadMethod__,
    __PostBlockRawPayloadMethod__,
    __GetBlockRawPayloadMethod__,
    __GetReceiptsPayloadMethod__,
    __GetReceiptsMigratedAtPayloadMethod__,
    __GetTreasuryPayloadMethod__,
    __GetIncludedBlockPayloadMethod__,
    __Bech32ToHexPayloadMethod__,
    __HexToBech32PayloadMethod__,
    __HexPublicKeyToBech32AddressPayloadMethod__,
    __IsAddressValidPayloadMethod__,
    __AliasOutputIdsPayloadMethod__,
    __AliasOutputIdPayloadMethod__,
    __NftOutputIdsPayloadMethod__,
    __NftOutputIdPayloadMethod__,
    __FoundryOutputIdsPayloadMethod__,
    __FoundryOutputIdPayloadMethod__,
    __TryGetOutputsPayloadMethod__,
    __FindBlocksPayloadMethod__,
    __RetryPayloadMethod__,
    __RetryUntilIncludedPayloadMethod__,
    __ConsolidateFundsPayloadMethod__,
    __ReattachPayloadMethod__,
    __ReattachUncheckedPayloadMethod__,
    __PromotePayloadMethod__,
    __PromoteUncheckedPayloadMethod__,
    __UnsyncedNodesPayloadMethod__,
    __GetMilestoneByIdPayloadMethod__,
    __GetUtxoChangesByIdPayloadMethod__,
    __GetMilestoneByIndexPayloadMethod__,
    __GetUtxoChangesByIndexPayloadMethod__,
    __StoreMnemonicPayloadMethod__,
    __BuildBasicOutputPayloadMethod__,
    __BuildAliasOutputPayloadMethod__,
    __BuildFoundryOutputPayloadMethod__,
    __BuildNftOutputPayloadMethod__,
} from './client';

export type __ClientPayloadMethods__ =
    | __GetInfoPayloadMethod__
    | __GetOutputPayloadMethod__
    | __GetBasicOutputIdsPayloadMethod__
    | __GetOutputsPayloadMethod__
    | __GenerateMnemonicPayloadMethod__
    | __MnemonicToHexSeedPayloadMethod__
    | __GenerateAddressesPayloadMethod__
    | __PostBlockPayloadMethod__
    | __GenerateBlockPayloadMethod__
    | __GetTipsPayloadMethod__
    | __GetNetworkInfoPayloadMethod__
    | __GetBlockPayloadMethod__
    | __GetBlockMetadataPayloadMethod__
    | __FindInputsPayloadMethod__
    | __FindOutputsPayloadMethod__
    | __PrepareTransactionPayloadMethod__
    | __SignTransactionPayloadMethod__
    | __StoreMnemonicPayloadMethod__
    | __SubmitPayloadPayloadMethod__
    | __ParseBech32AddressPayloadMethod__
    | __BlockIdPayloadMethod__
    | __GetNodePayloadMethod__
    | __GetNetworkIdPayloadMethod__
    | __GetBech32HrpPayloadMethod__
    | __GetMinPowScorePayloadMethod__
    | __GetTipsIntervalPayloadMethod__
    | __GetLocalPowPayloadMethod__
    | __GetFallbackToLocalPowPayloadMethod__
    | __GetHealthPayloadMethod__
    | __GetNodeInfoPayloadMethod__
    | __GetPeersPayloadMethod__
    | __PostBlockRawPayloadMethod__
    | __GetBlockRawPayloadMethod__
    | __GetMilestoneByIdPayloadMethod__
    | __GetUtxoChangesByIdPayloadMethod__
    | __GetMilestoneByIndexPayloadMethod__
    | __GetUtxoChangesByIndexPayloadMethod__
    | __GetReceiptsPayloadMethod__
    | __GetReceiptsMigratedAtPayloadMethod__
    | __GetTreasuryPayloadMethod__
    | __GetIncludedBlockPayloadMethod__
    | __Bech32ToHexPayloadMethod__
    | __HexToBech32PayloadMethod__
    | __HexPublicKeyToBech32AddressPayloadMethod__
    | __IsAddressValidPayloadMethod__
    | __AliasOutputIdsPayloadMethod__
    | __AliasOutputIdPayloadMethod__
    | __NftOutputIdsPayloadMethod__
    | __NftOutputIdPayloadMethod__
    | __FoundryOutputIdsPayloadMethod__
    | __FoundryOutputIdPayloadMethod__
    | __TryGetOutputsPayloadMethod__
    | __FindBlocksPayloadMethod__
    | __RetryPayloadMethod__
    | __RetryUntilIncludedPayloadMethod__
    | __ConsolidateFundsPayloadMethod__
    | __ReattachPayloadMethod__
    | __ReattachUncheckedPayloadMethod__
    | __PromotePayloadMethod__
    | __PromoteUncheckedPayloadMethod__
    | __UnsyncedNodesPayloadMethod__
    | __BuildBasicOutputPayloadMethod__
    | __BuildAliasOutputPayloadMethod__
    | __BuildFoundryOutputPayloadMethod__
    | __BuildNftOutputPayloadMethod__;

export interface __SendMessagePayload__ {
    cmd: 'CallClientMethod';
    payload: __ClientPayloadMethods__;
}
