// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    api::input_selection::new::requirement::Requirement,
    block::output::{AliasId, Output},
    error::{Error, Result},
    secret::types::InputSigningData,
};

pub(crate) fn fulfill_alias_requirement(
    alias_id: &AliasId,
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[Output],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    fn predicate(output: &Output, alias_id: &AliasId) -> bool {
        if let Output::Alias(alias_output) = output {
            alias_output.alias_id() == alias_id
        } else {
            false
        }
    }

    // Check if the requirement is already fulfilled.
    if selected_inputs
        .iter()
        .find(|input| predicate(input, alias_id))
        .is_some()
    {
        return Ok(Vec::new());
    }

    let index = available_inputs
        .iter()
        .position(|input| predicate(&input.output, alias_id));

    match index {
        Some(index) => Ok(vec![available_inputs.swap_remove(index)]),
        None => Err(Error::UnfulfilledRequirement(Requirement::Alias(*alias_id))),
    }
}
