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
    name: 'getInfo';
}

export interface __GetOutputMessage__ {
    name: 'getOutput';
    data: {
        outputId: string;
    };
}

export interface __GetBasicOutputIdsMessage__ {
    name: 'basicOutputIds';
    data: {
        queryParameters: QueryParameter[];
    };
}

export interface __GetOutputsMessage__ {
    name: 'getOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __GenerateMnemonicMessage__ {
    name: 'generateMnemonic';
}

export interface __MnemonicToHexSeedMessage__ {
    name: 'mnemonicToHexSeed';
    data: {
        mnemonic: string;
    };
}

export interface __ComputeAliasIdMessage__ {
    name: 'computeAliasId';
    data: {
        outputId: string;
    };
}

export interface __ComputeNftIdMessage__ {
    name: 'computeNftId';
    data: {
        outputId: string;
    };
}

export interface __ComputeFoundryIdMessage__ {
    name: 'computeFoundryId';
    data: {
        aliasAddress: string;
        serialNumber: number;
        tokenSchemeKind: number;
    };
}

export interface __GenerateAddressesMessage__ {
    name: 'generateAddresses';
    data: {
        secretManager: SecretManager;
        options: IGenerateAddressesOptions;
    };
}

export interface __PostBlockMessage__ {
    name: 'postBlock';
    data: {
        block: IBlock;
    };
}

export interface __BuildAndPostBlockMessage__ {
    name: 'buildAndPostBlock';
    data: {
        secretManager?: SecretManager;
        options?: IBuildBlockOptions;
    };
}

export interface __GetTipsMessage__ {
    name: 'getTips';
}

export interface __GetNetworkInfoMessage__ {
    name: 'getNetworkInfo';
}

export interface __GetBlockMessage__ {
    name: 'getBlock';
    data: {
        blockId: BlockId;
    };
}

export interface __GetBlockMetadataMessage__ {
    name: 'getBlockMetadata';
    data: {
        blockId: BlockId;
    };
}

export interface __FindInputsMessage__ {
    name: 'findInputs';
    data: {
        addresses: string[];
        amount: number;
    };
}

export interface __FindOutputsMessage__ {
    name: 'findOutputs';
    data: {
        outputIds: string[];
        addresses: string[];
    };
}

export interface __GetLedgerNanoStatusMessage__ {
    name: 'getLedgerNanoStatus';
    data: {
        isSimulator: boolean;
    };
}

export interface __PrepareTransactionMessage__ {
    name: 'prepareTransaction';
    data: {
        secretManager?: SecretManager;
        options?: IBuildBlockOptions;
    };
}

export interface __SignTransactionMessage__ {
    name: 'signTransaction';
    data: {
        secretManager: SecretManager;
        preparedTransactionData: IPreparedTransactionData;
    };
}

export interface __StoreMnemonicMessage__ {
    name: 'storeMnemonic';
    data: {
        secretManager: SecretManager;
        mnemonic: string;
    };
}

export interface __PostBlockPayloadMessage__ {
    name: 'postBlockPayload';
    data: {
        payload: PayloadTypes;
    };
}

export interface __ParseBech32AddressMessage__ {
    name: 'parseBech32Address';
    data: {
        address: string;
    };
}

export interface __BlockIdMessage__ {
    name: 'blockId';
    data: {
        block: IBlock;
    };
}

export interface __GetNodeMessage__ {
    name: 'getNode';
}

export interface __GetNetworkIdMessage__ {
    name: 'getNetworkId';
}

export interface __GetBech32HrpMessage__ {
    name: 'getBech32Hrp';
}

export interface __GetMinPowScoreMessage__ {
    name: 'getMinPowScore';
}

export interface __GetTipsIntervalMessage__ {
    name: 'getTipsInterval';
}

export interface __GetProtocolParametersMessage__ {
    name: 'getProtocolParameters';
}

export interface __GetLocalPowMessage__ {
    name: 'getLocalPow';
}

export interface __GetFallbackToLocalPowMessage__ {
    name: 'getFallbackToLocalPow';
}

export interface __GetHealthMessage__ {
    name: 'getHealth';
    data: {
        url: string;
    };
}

export interface __GetNodeInfoMessage__ {
    name: 'getNodeInfo';
    data: {
        url: string;
        auth?: IAuth;
    };
}

export interface __GetPeersMessage__ {
    name: 'getPeers';
}

export interface __PostBlockRawMessage__ {
    name: 'postBlockRaw';
    data: {
        block: IBlock;
    };
}

export interface __GetBlockRawMessage__ {
    name: 'getBlockRaw';
    data: {
        blockId: BlockId;
    };
}

export interface __GetMilestoneByIdMessage__ {
    name: 'getMilestoneById';
    data: {
        milestoneId: string;
    };
}

export interface __GetUtxoChangesByIdMessage__ {
    name: 'getUtxoChangesById';
    data: {
        milestoneId: string;
    };
}
export interface __GetMilestoneByIndexMessage__ {
    name: 'getMilestoneByIndex';
    data: {
        index: number;
    };
}

export interface __GetUtxoChangesByIndexMessage__ {
    name: 'getUtxoChangesByIndex';
    data: {
        index: number;
    };
}

export interface __GetReceiptsMessage__ {
    name: 'getReceipts';
}

export interface __GetReceiptsMigratedAtMessage__ {
    name: 'getReceiptsMigratedAt';
    data: {
        milestoneIndex: number;
    };
}

export interface __GetTreasuryMessage__ {
    name: 'getTreasury';
}

export interface __GetIncludedBlockMessage__ {
    name: 'getIncludedBlock';
    data: {
        transactionId: string;
    };
}

export interface __Bech32ToHexMessage__ {
    name: 'bech32ToHex';
    data: {
        bech32: string;
    };
}

export interface __HexToBech32Message__ {
    name: 'hexToBech32';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __AliasIdToBech32Message__ {
    name: 'aliasIdToBech32';
    data: {
        aliasId: string;
        bech32Hrp?: string;
    };
}

export interface __NftIdToBech32Message__ {
    name: 'nftIdToBech32';
    data: {
        nftId: string;
        bech32Hrp?: string;
    };
}

export interface __HexPublicKeyToBech32AddressMessage__ {
    name: 'hexPublicKeyToBech32Address';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __IsAddressValidMessage__ {
    name: 'isAddressValid';
    data: {
        address: string;
    };
}

export interface __AliasOutputIdsMessage__ {
    name: 'aliasOutputIds';
    data: {
        queryParameters: AliasQueryParameter[];
    };
}

export interface __AliasOutputIdMessage__ {
    name: 'aliasOutputId';
    data: {
        aliasId: string;
    };
}

export interface __NftOutputIdsMessage__ {
    name: 'nftOutputIds';
    data: {
        queryParameters: NftQueryParameter[];
    };
}

export interface __NftOutputIdMessage__ {
    name: 'nftOutputId';
    data: {
        nftId: string;
    };
}

export interface __FoundryOutputIdsMessage__ {
    name: 'foundryOutputIds';
    data: {
        queryParameters: FoundryQueryParameter[];
    };
}

export interface __FoundryOutputIdMessage__ {
    name: 'foundryOutputId';
    data: {
        foundryId: string;
    };
}

export interface __TryGetOutputsMessage__ {
    name: 'tryGetOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __FindBlocksMessage__ {
    name: 'findBlocks';
    data: {
        blockIds: string[];
    };
}

export interface __RetryMessage__ {
    name: 'retry';
    data: {
        blockId: string;
    };
}

export interface __RetryUntilIncludedMessage__ {
    name: 'retryUntilIncluded';
    data: {
        blockId: string;
        interval?: number;
        maxAttempts?: number;
    };
}

export interface __ConsolidateFundsMessage__ {
    name: 'consolidateFunds';
    data: {
        secretManager: SecretManager;
        generateAddressesOptions: IGenerateAddressesOptions;
    };
}

export interface __ReattachMessage__ {
    name: 'reattach';
    data: {
        blockId: BlockId;
    };
}

export interface __ReattachUncheckedMessage__ {
    name: 'reattachUnchecked';
    data: {
        blockId: BlockId;
    };
}

export interface __PromoteMessage__ {
    name: 'promote';
    data: {
        blockId: BlockId;
    };
}

export interface __PromoteUncheckedMessage__ {
    name: 'promoteUnchecked';
    data: {
        blockId: BlockId;
    };
}

export interface __UnhealthyNodesMessage__ {
    name: 'unhealthyNodes';
}

export interface __BuildBasicOutputMessage__ {
    name: 'buildBasicOutput';
    data: IBasicOutputBuilderOptions;
}

export interface __BuildAliasOutputMessage__ {
    name: 'buildAliasOutput';
    data: IAliasOutputBuilderOptions;
}

export interface __BuildFoundryOutputMessage__ {
    name: 'buildFoundryOutput';
    data: IFoundryOutputBuilderOptions;
}

export interface __BuildNftOutputMessage__ {
    name: 'buildNftOutput';
    data: INftOutputBuilderOptions;
}
