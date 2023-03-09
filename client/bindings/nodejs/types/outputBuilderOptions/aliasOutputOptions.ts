import type { FeatureTypes } from '@iota/types';
import type { HexEncodedString } from '../../lib';
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
