import type {
    __GetOutputsMethod__,
    __GetOutputsPayload__,
    __GetInfoPayloadMethod__,
    __GetInfoPayload__,
    __GetOutputIdsPayloadMethod__,
    __GetOutputIdsPayload__,
    __GetOutputPayloadMethod__,
    __GetOutputPayload__,
    __GenerateMnemonicMethod__,
    __GenerateMnemonicPayload__,
    __MnemonicToHexSeedMethod__,
    __MnemonicToHexSeedPayload__,
    __GenerateAddressesMethod__,
    __GenerateAddressesPayload__,
} from './client';

export type __ClientPayloadMethods__ =
    | __GetInfoPayloadMethod__
    | __GetOutputPayloadMethod__
    | __GetOutputIdsPayloadMethod__
    | __GetOutputsMethod__
    | __GenerateMnemonicMethod__
    | __MnemonicToHexSeedMethod__
    | __GenerateAddressesMethod__;

export type __SendMessagePayload__ =
    | __GetInfoPayload__
    | __GetOutputPayload__
    | __GetOutputIdsPayload__
    | __GetOutputsPayload__
    | __GenerateMnemonicPayload__
    | __MnemonicToHexSeedPayload__
    | __GenerateAddressesPayload__;
