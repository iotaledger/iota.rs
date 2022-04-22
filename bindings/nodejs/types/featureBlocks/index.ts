// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { IssuerFeatureBlock } from './issuerFeatureBlock';
import type { MetadataFeatureBlock } from './metaDataFeatureBlock';
import type { SenderFeatureBlock } from './senderFeatureBlock';
import type { TagFeatureBlock } from './tagFeatureBlock';

export type FeatureBlock =
    | SenderFeatureBlock
    | IssuerFeatureBlock
    | MetadataFeatureBlock
    | TagFeatureBlock;
