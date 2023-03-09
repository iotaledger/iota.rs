import type { HexEncodedString } from '@iota/iota.js';
import type { FeatureTypes } from '@iota/types';
import type { IBasicOutputBuilderOptions } from './basicOutputOptions';

/**
 * Options for building an Alias Output
 */
export interface IAliasOutputBuilderOptions extends IBasicOutputBuilderOptions {
    aliasId: HexEncodedString;
    stateIndex?: number;
    stateMetadata?: HexEncodedString;
    foundryCounter?: number;
    immutableFeatures?: FeatureTypes[];
}
