// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::Requirement;
use crate::{
    block::output::{FoundryId, Output},
    error::{Error, Result},
    secret::types::InputSigningData,
};

pub(crate) fn is_foundry_id(input: &InputSigningData, foundry_id: &FoundryId) -> bool {
    if let Output::Foundry(foundry_output) = &input.output {
        &foundry_output.id() == foundry_id
    } else {
        false
    }
}

/// Tries to fulfill a foundry requirement by selecting the appropriate foundry output from the available inputs.
pub(crate) fn fulfill_foundry_requirement(
    foundry_id: FoundryId,
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
    _outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    // Checks if the requirement is already fulfilled.
    if selected_inputs
        .iter()
        .find(|input| is_foundry_id(input, &foundry_id))
        .is_some()
    {
        return Ok(Vec::new());
    }

    // Checks if the requirement can be fulfilled.
    {
        let index = available_inputs
            .iter()
            .position(|input| is_foundry_id(input, &foundry_id));

        match index {
            Some(index) => Ok(vec![available_inputs.swap_remove(index)]),
            None => Err(Error::UnfulfillableRequirement(Requirement::Foundry(foundry_id))),
        }
    }
}
