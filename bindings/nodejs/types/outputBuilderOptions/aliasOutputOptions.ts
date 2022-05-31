import type { FeatureTypes } from '@iota/types';
import type { IBasicOutputBuilderOptions } from './basicOutputOptions';

/**
 * Options for building an Alias Output
 */
export interface IAliasOutputBuilderOptions extends IBasicOutputBuilderOptions {
    aliasId: string;
    stateIndex?: number;
    stateMetadata?: number[];
    foundryCounter?: number;
    immutableFeatures?: FeatureTypes[];
}
