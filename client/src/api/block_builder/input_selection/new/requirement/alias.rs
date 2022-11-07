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
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    fn predicate(input: &InputSigningData, alias_id: &AliasId) -> bool {
        if let Output::Alias(alias_output) = &input.output {
            // TODO unwrap
            &alias_output.alias_id().or_from_output_id(input.output_id().unwrap()) == alias_id
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

    // Check if the requirement can be fulfilled.
    {
        let index = available_inputs.iter().position(|input| predicate(input, alias_id));

        match index {
            Some(index) => Ok(vec![available_inputs.swap_remove(index)]),
            None => Err(Error::UnfulfillableRequirement(Requirement::Alias(*alias_id))),
        }
    }
}
