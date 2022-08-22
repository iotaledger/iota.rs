import type { IBlock, PayloadTypes } from '@iota/types';
import type { SecretManager } from '../secretManager';
import type { IGenerateAddressesOptions } from '../generateAddressesOptions';
import type { IBuildBlockOptions } from '../buildBlockOptions';
import type { BlockId } from '../blockId';
import type { IPreparedTransactionData } from '../preparedTransactionData';
import type {
    AliasQueryParameter,
    FoundryQueryParameter,
    NftQueryParameter,
    QueryParameter,
} from '../queryParameters';
import type { IAuth } from '../network';
import type { IBasicOutputBuilderOptions } from '../outputBuilderOptions/basicOutputOptions';
import type { IAliasOutputBuilderOptions } from '../outputBuilderOptions/aliasOutputOptions';
import type { IFoundryOutputBuilderOptions } from '../outputBuilderOptions/foundryOutputOptions';
import type { INftOutputBuilderOptions } from '../outputBuilderOptions/nftOutputOptions';

export interface __GetInfoMessage__ {
    name: 'GetInfo';
}

export interface __GetOutputMessage__ {
    name: 'GetOutput';
    data: {
        outputId: string;
    };
}

export interface __GetBasicOutputIdsMessage__ {
    name: 'BasicOutputIds';
    data: {
        queryParameters: QueryParameter[];
    };
}

export interface __GetOutputsMessage__ {
    name: 'GetOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __GenerateMnemonicMessage__ {
    name: 'GenerateMnemonic';
}

export interface __MnemonicToHexSeedMessage__ {
    name: 'MnemonicToHexSeed';
    data: {
        mnemonic: string;
    };
}

export interface __GenerateAddressesMessage__ {
    name: 'GenerateAddresses';
    data: {
        secretManager: SecretManager;
        options: IGenerateAddressesOptions;
    };
}

export interface __PostBlockMessage__ {
    name: 'PostBlock';
    data: {
        block: IBlock;
    };
}

export interface __BuildAndPostBlockMessage__ {
    name: 'BuildAndPostBlock';
    data: {
        secretManager?: SecretManager;
        options?: IBuildBlockOptions;
    };
}

export interface __GetTipsMessage__ {
    name: 'GetTips';
}

export interface __GetNetworkInfoMessage__ {
    name: 'GetNetworkInfo';
}

export interface __GetBlockMessage__ {
    name: 'GetBlock';
    data: {
        blockId: BlockId;
    };
}

export interface __GetBlockMetadataMessage__ {
    name: 'GetBlockMetadata';
    data: {
        blockId: BlockId;
    };
}

export interface __FindInputsMessage__ {
    name: 'FindInputs';
    data: {
        addresses: string[];
        amount: number;
    };
}

export interface __FindOutputsMessage__ {
    name: 'FindOutputs';
    data: {
        outputIds: string[];
        addresses: string[];
    };
}

export interface __GetLedgerNanoStatusMessage__ {
    name: 'GetLedgerNanoStatus';
    data: {
        isSimulator: boolean;
    };
}

export interface __PrepareTransactionMessage__ {
    name: 'PrepareTransaction';
    data: {
        secretManager?: SecretManager;
        options?: IBuildBlockOptions;
    };
}

export interface __SignTransactionMessage__ {
    name: 'SignTransaction';
    data: {
        secretManager: SecretManager;
        preparedTransactionData: IPreparedTransactionData;
    };
}

export interface __StoreMnemonicMessage__ {
    name: 'StoreMnemonic';
    data: {
        secretManager: SecretManager;
        mnemonic: string;
    };
}

export interface __SubmitPayloadMessage__ {
    name: 'SubmitPayload';
    data: {
        payload: PayloadTypes;
    };
}

export interface __ParseBech32AddressMessage__ {
    name: 'ParseBech32Address';
    data: {
        address: string;
    };
}

export interface __BlockIdMessage__ {
    name: 'BlockId';
    data: {
        block: IBlock;
    };
}

export interface __GetNodeMessage__ {
    name: 'GetNode';
}

export interface __GetNetworkIdMessage__ {
    name: 'GetNetworkId';
}

export interface __GetBech32HrpMessage__ {
    name: 'GetBech32Hrp';
}

export interface __GetMinPowScoreMessage__ {
    name: 'GetMinPowScore';
}

export interface __GetTipsIntervalMessage__ {
    name: 'GetTipsInterval';
}

export interface __GetLocalPowMessage__ {
    name: 'GetLocalPow';
}

export interface __GetFallbackToLocalPowMessage__ {
    name: 'GetFallbackToLocalPow';
}

export interface __GetHealthMessage__ {
    name: 'GetHealth';
    data: {
        url: string;
    };
}

export interface __GetNodeInfoMessage__ {
    name: 'GetNodeInfo';
    data: {
        url: string;
        auth?: IAuth;
    };
}

export interface __GetPeersMessage__ {
    name: 'GetPeers';
}

export interface __PostBlockRawMessage__ {
    name: 'PostBlockRaw';
    data: {
        block: IBlock;
    };
}

export interface __GetBlockRawMessage__ {
    name: 'GetBlockRaw';
    data: {
        blockId: BlockId;
    };
}

export interface __GetMilestoneByIdMessage__ {
    name: 'GetMilestoneById';
    data: {
        milestoneId: string;
    };
}

export interface __GetUtxoChangesByIdMessage__ {
    name: 'GetUtxoChangesById';
    data: {
        milestoneId: string;
    };
}
export interface __GetMilestoneByIndexMessage__ {
    name: 'GetMilestoneByIndex';
    data: {
        index: number;
    };
}

export interface __GetUtxoChangesByIndexMessage__ {
    name: 'GetUtxoChangesByIndex';
    data: {
        index: number;
    };
}

export interface __GetReceiptsMessage__ {
    name: 'GetReceipts';
}

export interface __GetReceiptsMigratedAtMessage__ {
    name: 'GetReceiptsMigratedAt';
    data: {
        milestoneIndex: number;
    };
}

export interface __GetTreasuryMessage__ {
    name: 'GetTreasury';
}

export interface __GetIncludedBlockMessage__ {
    name: 'GetIncludedBlock';
    data: {
        transactionId: string;
    };
}

export interface __Bech32ToHexMessage__ {
    name: 'Bech32ToHex';
    data: {
        bech32: string;
    };
}

export interface __HexToBech32Message__ {
    name: 'HexToBech32';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __HexPublicKeyToBech32AddressMessage__ {
    name: 'HexPublicKeyToBech32Address';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __IsAddressValidMessage__ {
    name: 'IsAddressValid';
    data: {
        address: string;
    };
}

export interface __AliasOutputIdsMessage__ {
    name: 'AliasOutputIds';
    data: {
        queryParameters: AliasQueryParameter[];
    };
}

export interface __AliasOutputIdMessage__ {
    name: 'AliasOutputId';
    data: {
        aliasId: string;
    };
}

export interface __NftOutputIdsMessage__ {
    name: 'NftOutputIds';
    data: {
        queryParameters: NftQueryParameter[];
    };
}

export interface __NftOutputIdMessage__ {
    name: 'NftOutputId';
    data: {
        nftId: string;
    };
}

export interface __FoundryOutputIdsMessage__ {
    name: 'FoundryOutputIds';
    data: {
        queryParameters: FoundryQueryParameter[];
    };
}

export interface __FoundryOutputIdMessage__ {
    name: 'FoundryOutputId';
    data: {
        foundryId: string;
    };
}

export interface __TryGetOutputsMessage__ {
    name: 'TryGetOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __FindBlocksMessage__ {
    name: 'FindBlocks';
    data: {
        blockIds: string[];
    };
}

export interface __RetryMessage__ {
    name: 'Retry';
    data: {
        blockId: string;
    };
}

export interface __RetryUntilIncludedMessage__ {
    name: 'RetryUntilIncluded';
    data: {
        blockId: string;
        interval?: number;
        maxAttempts?: number;
    };
}

export interface __ConsolidateFundsMessage__ {
    name: 'ConsolidateFunds';
    data: {
        secretManager: SecretManager;
        generateAddressesOptions: IGenerateAddressesOptions;
    };
}

export interface __ReattachMessage__ {
    name: 'Reattach';
    data: {
        blockId: BlockId;
    };
}

export interface __ReattachUncheckedMessage__ {
    name: 'ReattachUnchecked';
    data: {
        blockId: BlockId;
    };
}

export interface __PromoteMessage__ {
    name: 'Promote';
    data: {
        blockId: BlockId;
    };
}

export interface __PromoteUncheckedMessage__ {
    name: 'PromoteUnchecked';
    data: {
        blockId: BlockId;
    };
}

export interface __UnsyncedNodesMessage__ {
    name: 'UnsyncedNodes';
}

export interface __BuildBasicOutputMessage__ {
    name: 'BuildBasicOutput';
    data: IBasicOutputBuilderOptions;
}

export interface __BuildAliasOutputMessage__ {
    name: 'BuildAliasOutput';
    data: IAliasOutputBuilderOptions;
}

export interface __BuildFoundryOutputMessage__ {
    name: 'BuildFoundryOutput';
    data: IFoundryOutputBuilderOptions;
}

export interface __BuildNftOutputMessage__ {
    name: 'BuildNftOutput';
    data: INftOutputBuilderOptions;
}
