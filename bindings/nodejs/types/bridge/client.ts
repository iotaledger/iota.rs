import type { GenerateAddressesOptions } from '../client';

export interface __GetInfoPayloadMethod__ {
    name: 'GetInfo';
}

export interface __GetInfoPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetInfoPayloadMethod__;
}

export interface __GetOutputPayloadMethod__ {
    name: 'GetOutput';
    data: {
        outputId: string;
    };
}

export interface __GetOutputPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetOutputPayloadMethod__;
}

export interface __GetOutputIdsPayloadMethod__ {
    name: 'OutputIds';
    data: {
        queryParameters: string[];
    };
}

export interface __GetOutputIdsPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetOutputPayloadMethod__;
}

export interface __GetOutputsMethod__ {
    name: 'GetOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __GetOutputsPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetOutputsMethod__;
}

export interface __GenerateMnemonicMethod__ {
    name: 'GenerateMnemonic';
}

export interface __GenerateMnemonicPayload__ {
    cmd: 'CallClientMethod';
    payload: __GenerateMnemonicMethod__;
}

export interface __MnemonicToHexSeedMethod__ {
    name: 'MnemonicToHexSeed';
    data: {
        mnemonic: string;
    };
}

export interface __MnemonicToHexSeedPayload__ {
    cmd: 'CallClientMethod';
    payload: __MnemonicToHexSeedMethod__;
}

export interface __GenerateAddressesMethod__ {
    name: 'GenerateAddresses';
    data: {
        signer: string;
        options: GenerateAddressesOptions;
    };
}

export interface __GenerateAddressesPayload__ {
    cmd: 'CallClientMethod';
    payload: __GenerateAddressesMethod__;
}
