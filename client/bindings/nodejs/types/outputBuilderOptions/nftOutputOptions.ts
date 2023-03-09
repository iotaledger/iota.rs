import type { HexEncodedString } from '@iota/iota.js';
import type { FeatureTypes } from '@iota/types';
import type { IBasicOutputBuilderOptions } from './basicOutputOptions';

/**
 * Options for building an Nft Output
 */
export interface INftOutputBuilderOptions extends IBasicOutputBuilderOptions {
    nftId: HexEncodedString;
    immutableFeatures?: FeatureTypes[];
}
