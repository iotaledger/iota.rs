import type { FeatureTypes } from '@iota/types';
import type { IBasicOutputBuilderOptions } from './basicOutputOptions';

/**
 * Options for building an Alias Output
 */
export interface IAliasOutputBuilderOptions extends IBasicOutputBuilderOptions {
    aliasId: string;
    stateIndex?: number;
    stateMetadata?: Uint8Array;
    foundryCounter?: number;
    immutableFeatures?: FeatureTypes[];
}
