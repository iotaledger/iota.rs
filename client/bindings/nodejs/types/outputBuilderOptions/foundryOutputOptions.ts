import type { FeatureTypes, ISimpleTokenScheme } from '@iota/types';
import type { IBasicOutputBuilderOptions } from './basicOutputOptions';

/**
 * Options for building a Foundry Output
 */
export interface IFoundryOutputBuilderOptions
    extends IBasicOutputBuilderOptions {
    /**
     * The serial number of the foundry with respect to the controlling alias.
     */
    serialNumber: number;
    tokenScheme: ISimpleTokenScheme;
    immutableFeatures?: FeatureTypes[];
}
