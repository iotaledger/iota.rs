// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::output::Output;

pub(crate) fn fulfill_remainder_requirement(
    available_inputs: &[Output],
    selected_inputs: &[Output],
    outputs: &[Output],
) -> Vec<Output> {
    vec![]
}
