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
    | __MessageIdPayloadMethod__;

export interface __SendMessagePayload__ {
    cmd: 'CallClientMethod';
    payload: __ClientPayloadMethods__;
}
