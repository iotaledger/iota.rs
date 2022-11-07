// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::output::{FoundryId, Output};

pub(crate) fn fulfill_foundry_requirement(
    foundry_id: &FoundryId,
    available_inputs: &[Output],
    selected_inputs: &[Output],
    outputs: &[Output],
) -> Vec<Output> {
    vec![]
}
