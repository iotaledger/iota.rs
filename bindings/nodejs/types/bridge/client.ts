import type { GenerateAddressesOptions } from '../generateAddressesOptions';
import type { GenerateMessageOptions } from '../generateMessageOptions';
import type { UTXOInput } from '../inputs/UTXOInput';
import type { Message } from '../message';
import type { Payload } from '../payloads';
import type { PreparedTransactionData } from '../preparedTransactionData';
import type { QueryParameter } from '../queryParameters';

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
        queryParameters: QueryParameter[];
    };
}

export interface __GetOutputIdsPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetOutputPayloadMethod__;
}

export interface __GetOutputsPayloadMethod__ {
    name: 'GetOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __GetOutputsPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetOutputsPayloadMethod__;
}

export interface __GenerateMnemonicPayloadMethod__ {
    name: 'GenerateMnemonic';
}

export interface __GenerateMnemonicPayload__ {
    cmd: 'CallClientMethod';
    payload: __GenerateMnemonicPayloadMethod__;
}

export interface __MnemonicToHexSeedPayloadMethod__ {
    name: 'MnemonicToHexSeed';
    data: {
        mnemonic: string;
    };
}

export interface __MnemonicToHexSeedPayload__ {
    cmd: 'CallClientMethod';
    payload: __MnemonicToHexSeedPayloadMethod__;
}

export interface __GenerateAddressesPayloadMethod__ {
    name: 'GenerateAddresses';
    data: {
        signer: string;
        options: GenerateAddressesOptions;
    };
}

export interface __GenerateAddressesPayload__ {
    cmd: 'CallClientMethod';
    payload: __GenerateAddressesPayloadMethod__;
}

export interface __PostMessagePayloadMethod__ {
    name: 'PostMessage';
    data: {
        message: Message;
    };
}

export interface __PostMessagePayload__ {
    cmd: 'CallClientMethod';
    payload: __PostMessagePayloadMethod__;
}

export interface __GenerateMessagePayloadMethod__ {
    name: 'GenerateMessage';
    data: {
        signer?: string;
        options?: GenerateMessageOptions;
    };
}

export interface __GenerateMessagePayload__ {
    cmd: 'CallClientMethod';
    payload: __GenerateMessagePayloadMethod__;
}
export interface __GetTipsPayloadMethod__ {
    name: 'GetTips';
}

export interface __GetTipsPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetTipsPayloadMethod__;
}

export interface __GetNetworkInfoPayloadMethod__ {
    name: 'GetNetworkInfo';
}

export interface __GetNetworkInfoPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetNetworkInfoPayloadMethod__;
}

export interface __GetMessageDataPayloadMethod__ {
    name: 'GetMessageData';
    data: {
        messageId: string;
    };
}

export interface __GetMessageDataPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetMessageDataPayloadMethod__;
}

export interface __GetMessageMetadataPayloadMethod__ {
    name: 'GetMessageMetadata';
    data: {
        messageId: string;
    };
}

export interface __GetMessageMetadataPayload__ {
    cmd: 'CallClientMethod';
    payload: __GetMessageMetadataPayloadMethod__;
}

export interface __FindInputsPayloadMethod__ {
    name: 'FindInputs';
    data: {
        addresses: string[];
        amount: number;
    };
}

export interface __FindInputsPayload__ {
    cmd: 'CallClientMethod';
    payload: __FindInputsPayloadMethod__;
}
export interface __FindOutputsPayloadMethod__ {
    name: 'FindOutputs';
    data: {
        outputs: UTXOInput[];
        addresses: string[];
    };
}

export interface __FindOutputsPayload__ {
    cmd: 'CallClientMethod';
    payload: __FindOutputsPayloadMethod__;
}

export interface __PrepareTransactionPayloadMethod__ {
    name: 'PrepareTransaction';
    data: {
        signer?: string;
        options?: GenerateMessageOptions;
    };
}

export interface __PrepareTransactionPayload__ {
    cmd: 'CallClientMethod';
    payload: __PrepareTransactionPayloadMethod__;
}

export interface __SignTransactionPayloadMethod__ {
    name: 'SignTransaction';
    data: {
        signer: string;
        preparedTransactionData: PreparedTransactionData;
    };
}

export interface __SignTransactionPayload__ {
    cmd: 'CallClientMethod';
    payload: __SignTransactionPayloadMethod__;
}

export interface __SubmitPayloadPayloadMethod__ {
    name: 'SubmitPayload';
    data: {
        payload: Payload;
    };
}

export interface __SubmitPayloadPayload__ {
    cmd: 'CallClientMethod';
    payload: __SubmitPayloadPayloadMethod__;
}

export interface __ParseBech32AddressPayloadMethod__ {
    name: 'ParseBech32Address';
    data: {
        address: string;
    };
}

export interface __ParseBech32AddressPayload__ {
    cmd: 'CallClientMethod';
    payload: __ParseBech32AddressPayloadMethod__;
}
