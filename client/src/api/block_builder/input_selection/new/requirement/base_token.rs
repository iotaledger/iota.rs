// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    block::output::Output,
    error::{Error, Result},
    secret::types::InputSigningData,
};

pub(crate) fn fulfill_base_coin_requirement(
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    todo!()
}
