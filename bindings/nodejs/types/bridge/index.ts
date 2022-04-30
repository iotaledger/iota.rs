import type {
    __GetInfoPayloadMethod__,
    __GetOutputIdsPayloadMethod__,
    __GetOutputPayloadMethod__,
    __GetOutputsPayloadMethod__,
    __GenerateMnemonicPayloadMethod__,
    __MnemonicToHexSeedPayloadMethod__,
    __GenerateAddressesPayloadMethod__,
    __PostMessagePayloadMethod__,
    __GenerateMessagePayloadMethod__,
    __GetTipsPayloadMethod__,
    __GetNetworkInfoPayloadMethod__,
    __GetMessageDataPayloadMethod__,
    __GetMessageMetadataPayloadMethod__,
    __FindInputsPayloadMethod__,
    __FindOutputsPayloadMethod__,
    __PrepareTransactionPayloadMethod__,
    __SignTransactionPayloadMethod__,
    __SubmitPayloadPayloadMethod__,
    __ParseBech32AddressPayloadMethod__,
    __MessageIdPayloadMethod__,
    __GetNodePayloadMethod__,
    __GetNetworkIdPayloadMethod__,
    __GetBech32HrpPayloadMethod__,
    __GetMinPowScorePayloadMethod__,
    __GetTipsIntervalPayloadMethod__,
    __GetLocalPowPayloadMethod__,
    __GetFallbackToLocalPowPayloadMethod__,
    __GetNodeHealthPayloadMethod__,
    __GetHealthPayloadMethod__,
    __GetNodeInfoPayloadMethod__,
    __GetPeersPayloadMethod__,
    __PostMessageJsonPayloadMethod__,
    __GetMessageRawPayloadMethod__,
    __GetMessageChildrenPayloadMethod__,
    __GetMilestonePayloadMethod__,
    __GetMilestoneUtxoChangesPayloadMethod__,
    __GetReceiptsPayloadMethod__,
    __GetReceiptsMigratedAtPayloadMethod__,
    __GetTreasuryPayloadMethod__,
    __GetIncludedMessagePayloadMethod__,
    __Bech32ToHexPayloadMethod__,
    __HexToBech32PayloadMethod__,
    __HexPublicKeyToBech32AddressPayloadMethod__,
    __IsAddressValidPayloadMethod__,
} from './client';

export type __ClientPayloadMethods__ =
    | __GetInfoPayloadMethod__
    | __GetOutputPayloadMethod__
    | __GetOutputIdsPayloadMethod__
    | __GetOutputsPayloadMethod__
    | __GenerateMnemonicPayloadMethod__
    | __MnemonicToHexSeedPayloadMethod__
    | __GenerateAddressesPayloadMethod__
    | __PostMessagePayloadMethod__
    | __GenerateMessagePayloadMethod__
    | __GetTipsPayloadMethod__
    | __GetNetworkInfoPayloadMethod__
    | __GetMessageDataPayloadMethod__
    | __GetMessageMetadataPayloadMethod__
    | __FindInputsPayloadMethod__
    | __FindOutputsPayloadMethod__
    | __PrepareTransactionPayloadMethod__
    | __SignTransactionPayloadMethod__
    | __SubmitPayloadPayloadMethod__
    | __ParseBech32AddressPayloadMethod__
    | __MessageIdPayloadMethod__
    | __GetNodePayloadMethod__
    | __GetNetworkIdPayloadMethod__
    | __GetBech32HrpPayloadMethod__
    | __GetMinPowScorePayloadMethod__
    | __GetTipsIntervalPayloadMethod__
    | __GetLocalPowPayloadMethod__
    | __GetFallbackToLocalPowPayloadMethod__
    | __GetNodeHealthPayloadMethod__
    | __GetHealthPayloadMethod__
    | __GetNodeInfoPayloadMethod__
    | __GetPeersPayloadMethod__
    | __PostMessageJsonPayloadMethod__
    | __GetMessageRawPayloadMethod__
    | __GetMessageChildrenPayloadMethod__
    | __GetMilestonePayloadMethod__
    | __GetMilestoneUtxoChangesPayloadMethod__
    | __GetReceiptsPayloadMethod__
    | __GetReceiptsMigratedAtPayloadMethod__
    | __GetTreasuryPayloadMethod__
    | __GetIncludedMessagePayloadMethod__
    | __Bech32ToHexPayloadMethod__
    | __HexToBech32PayloadMethod__
    | __HexPublicKeyToBech32AddressPayloadMethod__
    | __IsAddressValidPayloadMethod__;

export interface __SendMessagePayload__ {
    cmd: 'CallClientMethod';
    payload: __ClientPayloadMethods__;
}
