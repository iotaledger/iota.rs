// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { TypeBase } from '../typeBase';
import type { CommonOutput } from './commonOutput';

/**
 * The global type for the basic output.
 */
export const BASIC_OUTPUT_TYPE = 3;

/*
 *Describes a basic output.
 */
export interface BasicOutput extends TypeBase<3>, CommonOutput {
    /**
     * Amount of IOTA tokens held by the output.
     */
    amount: string;
}
