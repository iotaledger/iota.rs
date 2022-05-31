import type { IBlock, PayloadTypes } from '@iota/types';
import type { SecretManager } from '../secretManager';
import type { IGenerateAddressesOptions } from '../generateAddressesOptions';
import type { IGenerateBlockOptions } from '../generateBlockOptions';
import type { BlockId } from '../blockId';
import type { IPreparedTransactionData } from '../preparedTransactionData';
import type {
    AliasQueryParameter,
    FoundryQueryParameter,
    NftQueryParameter,
    QueryParameter,
} from '../queryParameters';
import type { IAuth } from '../network';
import type { IRange } from '../range';
import type { IBasicOutputBuilderOptions } from '../outputBuilderOptions/basicOutputOptions';
import type { IAliasOutputBuilderOptions } from '../outputBuilderOptions/aliasOutputOptions';
import type { IFoundryOutputBuilderOptions } from '../outputBuilderOptions/foundryOutputOptions';
import type { INftOutputBuilderOptions } from '../outputBuilderOptions/nftOutputOptions';

export interface __GetInfoPayloadMethod__ {
    name: 'GetInfo';
}

export interface __GetOutputPayloadMethod__ {
    name: 'GetOutput';
    data: {
        outputId: string;
    };
}

export interface __GetBasicOutputIdsPayloadMethod__ {
    name: 'BasicOutputIds';
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

export interface __PostBlockPayloadMethod__ {
    name: 'PostBlock';
    data: {
        block: IBlock;
    };
}

export interface __GenerateBlockPayloadMethod__ {
    name: 'GenerateBlock';
    data: {
        secretManager?: SecretManager;
        options?: IGenerateBlockOptions;
    };
}

export interface __GetTipsPayloadMethod__ {
    name: 'GetTips';
}

export interface __GetNetworkInfoPayloadMethod__ {
    name: 'GetNetworkInfo';
}

export interface __GetBlockPayloadMethod__ {
    name: 'GetBlock';
    data: {
        blockId: BlockId;
    };
}

export interface __GetBlockMetadataPayloadMethod__ {
    name: 'GetBlockMetadata';
    data: {
        blockId: BlockId;
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
        options?: IGenerateBlockOptions;
    };
}

export interface __SignTransactionPayloadMethod__ {
    name: 'SignTransaction';
    data: {
        secretManager: SecretManager;
        preparedTransactionData: IPreparedTransactionData;
    };
}

export interface __StoreMnemonicPayloadMethod__ {
    name: 'StoreMnemonic';
    data: {
        secretManager: SecretManager;
        mnemonic: string;
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

export interface __BlockIdPayloadMethod__ {
    name: 'BlockId';
    data: {
        block: IBlock;
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

export interface __GetHealthPayloadMethod__ {
    name: 'GetHealth';
    data: {
        url: string;
    };
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

export interface __PostBlockRawPayloadMethod__ {
    name: 'PostBlockRaw';
    data: {
        block: IBlock;
    };
}

export interface __GetBlockRawPayloadMethod__ {
    name: 'GetBlockRaw';
    data: {
        blockId: BlockId;
    };
}

export interface __GetBlockChildrenPayloadMethod__ {
    name: 'GetBlockChildren';
    data: {
        blockId: BlockId;
    };
}

export interface __GetMilestoneByIdPayloadMethod__ {
    name: 'GetMilestoneById';
    data: {
        milestoneId: string;
    };
}

export interface __GetUtxoChangesByIdPayloadMethod__ {
    name: 'GetUtxoChangesById';
    data: {
        milestoneId: string;
    };
}
export interface __GetMilestoneByIndexPayloadMethod__ {
    name: 'GetMilestoneByIndex';
    data: {
        index: number;
    };
}

export interface __GetUtxoChangesByIndexPayloadMethod__ {
    name: 'GetUtxoChangesByIndex';
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

export interface __GetIncludedBlockPayloadMethod__ {
    name: 'GetIncludedBlock';
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

export interface __AliasOutputIdsPayloadMethod__ {
    name: 'AliasOutputIds';
    data: {
        queryParameters: AliasQueryParameter[];
    };
}

export interface __AliasOutputIdPayloadMethod__ {
    name: 'AliasOutputId';
    data: {
        aliasId: string;
    };
}

export interface __NftOutputIdsPayloadMethod__ {
    name: 'NftOutputIds';
    data: {
        queryParameters: NftQueryParameter[];
    };
}

export interface __NftOutputIdPayloadMethod__ {
    name: 'NftOutputId';
    data: {
        nftId: string;
    };
}

export interface __FoundryOutputIdsPayloadMethod__ {
    name: 'FoundryOutputIds';
    data: {
        queryParameters: FoundryQueryParameter[];
    };
}

export interface __FoundryOutputIdPayloadMethod__ {
    name: 'FoundryOutputId';
    data: {
        foundryId: string;
    };
}

export interface __TryGetOutputsPayloadMethod__ {
    name: 'TryGetOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __FindBlocksPayloadMethod__ {
    name: 'FindBlocks';
    data: {
        blockIds: string[];
    };
}

export interface __RetryPayloadMethod__ {
    name: 'Retry';
    data: {
        blockId: string;
    };
}

export interface __RetryUntilIncludedPayloadMethod__ {
    name: 'RetryUntilIncluded';
    data: {
        blockId: string;
        interval?: number;
        maxAttempts?: number;
    };
}

export interface __ConsolidateFundsPayloadMethod__ {
    name: 'ConsolidateFunds';
    data: {
        secretManager: SecretManager;
        accountIndex: number;
        addressRange?: IRange;
    };
}

export interface __ReattachPayloadMethod__ {
    name: 'Reattach';
    data: {
        blockId: BlockId;
    };
}

export interface __ReattachUncheckedPayloadMethod__ {
    name: 'ReattachUnchecked';
    data: {
        blockId: BlockId;
    };
}

export interface __PromotePayloadMethod__ {
    name: 'Promote';
    data: {
        blockId: BlockId;
    };
}

export interface __PromoteUncheckedPayloadMethod__ {
    name: 'PromoteUnchecked';
    data: {
        blockId: BlockId;
    };
}

export interface __UnsyncedNodesPayloadMethod__ {
    name: 'UnsyncedNodes';
}

export interface __BuildBasicOutputPayloadMethod__ {
    name: 'BuildBasicOutput';
    data: IBasicOutputBuilderOptions;
}
export interface __BuildAliasOutputPayloadMethod__ {
    name: 'BuildAliasOutput';
    data: IAliasOutputBuilderOptions;
}
export interface __BuildFoundryOutputPayloadMethod__ {
    name: 'BuildFoundryOutput';
    data: IFoundryOutputBuilderOptions;
}
export interface __BuildNftOutputPayloadMethod__ {
    name: 'BuildNftOutput';
    data: INftOutputBuilderOptions;
}
