import type {
    FeatureTypes,
    INativeToken,
    UnlockConditionTypes,
} from '@iota/types';

/**
 * Options for building a Basic Output
 */
export interface IBasicOutputBuilderOptions {
    /**
     * If not provided, minimum storage deposit will be used
     */
    amount?: string;
    /**
     * The native tokens to be held by the output.
     */
    nativeTokens?: INativeToken[];
    /**
     * The unlock conditions for the output.
     */
    unlockConditions: UnlockConditionTypes[];
    /**
     * Features to be contained by the output.
     */
    features?: FeatureTypes[];
}
