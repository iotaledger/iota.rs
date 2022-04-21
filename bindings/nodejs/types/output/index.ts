import type { FeatureBlockDto } from './featureBlocks';
import type { UnlockConditionDto } from './unlockConditions';

export interface OutputResponse {
    messageId: string;
    transactionId: string;
    outputIndex: number;
    isSpent: boolean;
    milestoneIndexSpent?: number;
    milestoneTimestampSpent?: number;
    transactionIdSpent?: string;
    milestoneIndexBooked: number;
    milestoneTimestampBooked: number;
    ledgerIndex: number;
    // TODO: Verify this type is correct.
    output: OutputDto;
}

// Describes all the different output types.
export type OutputDto =
    | TreasuryOutputDto
    | BasicOutputDto
    | AliasOutputDto
    | FoundryOutputDto
    | NftOutputDto;

// Base type of all outputDto's
export interface BaseOutput {
    type: number;
    // Amount of IOTA tokens held by the output.
    amount: string;
}

// Describes a treasury output.
export type TreasuryOutputDto = BaseOutput;

// Describes a basic output.
export interface BasicOutputDto extends BaseOutput {
    // Native tokens held by the output.
    nativeTokens: NativeToken[];
    unlockConditions: UnlockConditionDto[];
    featureBlocks: FeatureBlockDto[];
}

// Describes an alias account in the ledger that can be controlled by the state and governance controllers.
export interface AliasOutputDto extends BaseOutput {
    // Native tokens held by the output.
    nativeTokens: NativeToken[];
    // Unique identifier of the alias.
    aliasId: string;
    // A counter that must increase by 1 every time the alias is state transitioned.
    stateIndex: number;
    // Metadata that can only be changed by the state controller.
    stateMetadata: string;
    // A counter that denotes the number of foundries created by this alias account.
    foundryCounter: number;
    unlockConditions: UnlockConditionDto[];
    featureBlocks: FeatureBlockDto[];
    immutableFeatureBlocks: FeatureBlockDto[];
}

// Describes a foundry output that is controlled by an alias.
export interface FoundryOutputDto extends BaseOutput {
    // Native tokens held by the output.
    nativeTokens: NativeToken[];
    // The serial number of the foundry with respect to the controlling alias.
    serialNumber: number;
    // Data that is always the last 12 bytes of ID of the tokens produced by this foundry.
    tokenTag: string;
    tokenScheme: TokenSchemeDto;
    unlockConditions: UnlockConditionDto[];
    featureBlocks: FeatureBlockDto[];
    immutableFeatureBlocks: FeatureBlockDto[];
}

// Describes an NFT output, a globally unique token with metadata attached.
export interface NftOutputDto extends BaseOutput {
    // Native tokens held by the output.
    nativeTokens: NativeToken[];
    // Unique identifier of the NFT.
    nftId: string;
    unlockConditions: UnlockConditionDto[];
    featureBlocks: FeatureBlockDto[];
    immutableFeatureBlocks: FeatureBlockDto[];
}

export interface NativeToken {
    id: string;
    amount: string;
}

export type TokenSchemeDto = SimpleTokenSchemeDto;

export interface SimpleTokenSchemeDto {
    type: number;
    // Amount of tokens minted by a foundry.
    mintedTokens: string;
    // Amount of tokens melted by a foundry.
    meltedTokens: string;
    // Maximum supply of tokens controlled by a foundry.
    maximumSupply: string;
}
