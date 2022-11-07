// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::{address::Address, output::Output};

pub(crate) fn fulfill_sender_requirement(
    address: &Address,
    available_inputs: &[Output],
    selected_inputs: &[Output],
    outputs: &[Output],
) -> Vec<Output> {
    todo!()
}
