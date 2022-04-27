// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { PoWMilestoneOption } from './powMilestoneOption';
import type { ReceiptMilestoneOption } from './receiptMilestoneOption';

/**
 * All of the milestone option types.
 */
export type MilestoneOption = ReceiptMilestoneOption | PoWMilestoneOption;
