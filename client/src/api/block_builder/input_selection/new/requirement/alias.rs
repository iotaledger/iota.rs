// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::output::{AliasId, Output};

pub(crate) fn fulfill_alias_requirement(
    alias_id: &AliasId,
    available_inputs: &[Output],
    selected_inputs: &[Output],
    outputs: &[Output],
) -> Vec<Output> {
    todo!()
}
