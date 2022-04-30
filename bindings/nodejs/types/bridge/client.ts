import type { IMessage, PayloadTypes } from '@iota/types';
import type { SecretManager } from '../secretManager';
import type { IGenerateAddressesOptions } from '../generateAddressesOptions';
import type { IGenerateMessageOptions } from '../generateMessageOptions';
import type { MessageId } from '../messageId';
import type { IPreparedTransactionData } from '../preparedTransactionData';
import type { QueryParameter } from '../queryParameters';
import type { IAuth } from '../network';

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
        outputIds: string[];
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

export interface __GetNodePayloadMethod__ {
    name: 'GetNode';
}

export interface __GetNetworkIdPayloadMethod__ {
    name: 'GetNetworkId';
}

export interface __GetBech32HrpPayloadMethod__ {
    name: 'GetBech32Hrp';
}

export interface __GetMinPowScorePayloadMethod__ {
    name: 'GetMinPoWScore';
}

export interface __GetTipsIntervalPayloadMethod__ {
    name: 'GetTipsInterval';
}

export interface __GetLocalPowPayloadMethod__ {
    name: 'GetLocalPoW';
}

export interface __GetFallbackToLocalPowPayloadMethod__ {
    name: 'GetFallbackToLocalPoW';
}

export interface __GetNodeHealthPayloadMethod__ {
    name: 'GetNodeHealth';
    data: {
        url: string;
    };
}

export interface __GetHealthPayloadMethod__ {
    name: 'GetHealth';
}

export interface __GetNodeInfoPayloadMethod__ {
    name: 'GetNodeInfo';
    data: {
        url: string;
        auth?: IAuth;
    };
}

export interface __GetPeersPayloadMethod__ {
    name: 'GetPeers';
}

export interface __PostMessageJsonPayloadMethod__ {
    name: 'PostMessageJson';
    data: {
        message: IMessage;
    };
}

export interface __GetMessageRawPayloadMethod__ {
    name: 'GetMessageRaw';
    data: {
        messageId: MessageId;
    };
}

export interface __GetMessageChildrenPayloadMethod__ {
    name: 'GetMessageChildren';
    data: {
        messageId: MessageId;
    };
}

export interface __GetMilestonePayloadMethod__ {
    name: 'GetMilestone';
    data: {
        index: number;
    };
}

export interface __GetMilestoneUtxoChangesPayloadMethod__ {
    name: 'GetMilestoneUtxoChanges';
    data: {
        index: number;
    };
}

export interface __GetReceiptsPayloadMethod__ {
    name: 'GetReceipts';
}

export interface __GetReceiptsMigratedAtPayloadMethod__ {
    name: 'GetReceiptsMigratedAt';
    data: {
        milestoneIndex: number;
    };
}

export interface __GetTreasuryPayloadMethod__ {
    name: 'GetTreasury';
}

export interface __GetIncludedMessagePayloadMethod__ {
    name: 'GetIncludedMessage';
    data: {
        transactionId: string;
    };
}

export interface __Bech32ToHexPayloadMethod__ {
    name: 'Bech32ToHex';
    data: {
        bech32: string;
    };
}

export interface __HexToBech32PayloadMethod__ {
    name: 'HexToBech32';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __HexPublicKeyToBech32AddressPayloadMethod__ {
    name: 'HexPublicKeyToBech32Address';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __IsAddressValidPayloadMethod__ {
    name: 'IsAddressValid';
    data: {
        address: string;
    };
}
