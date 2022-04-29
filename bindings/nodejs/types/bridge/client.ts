import type { IMessage, IUTXOInput, PayloadTypes } from '@iota/types';
import type { SecretManager } from '../secretManager';
import type { IGenerateAddressesOptions } from '../generateAddressesOptions';
import type { IGenerateMessageOptions } from '../generateMessageOptions';
import type { MessageId } from '../messageId';
import type { IPreparedTransactionData } from '../preparedTransactionData';
import type { QueryParameter } from '../queryParameters';

export interface __GetInfoPayloadMethod__ {
    name: 'GetInfo';
}

export interface __GetOutputPayloadMethod__ {
    name: 'GetOutput';
    data: {
        outputId: string;
    };
}

export interface __GetOutputIdsPayloadMethod__ {
    name: 'OutputIds';
    data: {
        queryParameters: QueryParameter[];
    };
}

export interface __GetOutputsPayloadMethod__ {
    name: 'GetOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __GenerateMnemonicPayloadMethod__ {
    name: 'GenerateMnemonic';
}

export interface __MnemonicToHexSeedPayloadMethod__ {
    name: 'MnemonicToHexSeed';
    data: {
        mnemonic: string;
    };
}

export interface __GenerateAddressesPayloadMethod__ {
    name: 'GenerateAddresses';
    data: {
        secretManager: SecretManager;
        options: IGenerateAddressesOptions;
    };
}

export interface __PostMessagePayloadMethod__ {
    name: 'PostMessage';
    data: {
        message: IMessage;
    };
}

export interface __GenerateMessagePayloadMethod__ {
    name: 'GenerateMessage';
    data: {
        secretManager?: SecretManager;
        options?: IGenerateMessageOptions;
    };
}

export interface __GetTipsPayloadMethod__ {
    name: 'GetTips';
}

export interface __GetNetworkInfoPayloadMethod__ {
    name: 'GetNetworkInfo';
}

export interface __GetMessageDataPayloadMethod__ {
    name: 'GetMessageData';
    data: {
        messageId: MessageId;
    };
}

export interface __GetMessageMetadataPayloadMethod__ {
    name: 'GetMessageMetadata';
    data: {
        messageId: MessageId;
    };
}

export interface __FindInputsPayloadMethod__ {
    name: 'FindInputs';
    data: {
        addresses: string[];
        amount: number;
    };
}

export interface __FindOutputsPayloadMethod__ {
    name: 'FindOutputs';
    data: {
        outputs: IUTXOInput[];
        addresses: string[];
    };
}

export interface __PrepareTransactionPayloadMethod__ {
    name: 'PrepareTransaction';
    data: {
        secretManager?: SecretManager;
        options?: IGenerateMessageOptions;
    };
}

export interface __SignTransactionPayloadMethod__ {
    name: 'SignTransaction';
    data: {
        secretManager: SecretManager;
        preparedTransactionData: IPreparedTransactionData;
    };
}

export interface __SubmitPayloadPayloadMethod__ {
    name: 'SubmitPayload';
    data: {
        payload: PayloadTypes;
    };
}

export interface __ParseBech32AddressPayloadMethod__ {
    name: 'ParseBech32Address';
    data: {
        address: string;
    };
}

export interface __MessageIdPayloadMethod__ {
    name: 'MessageId';
    data: {
        message: IMessage;
    };
}
