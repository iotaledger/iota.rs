// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { FeatureBlock } from '../featureBlocks';
import type { NativeToken } from '../nativeToken';
import type { UnlockCondition } from '../unlockConditions';

/**
 * Common output properties.
 */
export interface CommonOutput {
    /**
     * The native tokens held by the output.
     */
    nativeTokens: NativeToken[];

    /**
     * The unlock conditions for the output.
     */
    unlockConditions: UnlockCondition[];

    /**
     * Feature blocks contained by the output.
     */
    featureBlocks: FeatureBlock[];
}
