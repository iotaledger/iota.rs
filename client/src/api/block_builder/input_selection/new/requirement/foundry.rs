// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::Requirement;
use crate::{
    block::output::{FoundryId, Output},
    error::{Error, Result},
    secret::types::InputSigningData,
};

/// Checks if an output is a foundry with foundry ID that matches the given one.
pub(crate) fn is_foundry_with_id(output: &Output, foundry_id: &FoundryId) -> bool {
    if let Output::Foundry(foundry) = output {
        &foundry.id() == foundry_id
    } else {
        false
    }
}

/// Fulfills a foundry requirement by selecting the appropriate foundry from the available inputs.
pub(crate) fn fulfill_foundry_requirement(
    foundry_id: FoundryId,
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
) -> Result<Vec<InputSigningData>> {
    // Checks if the requirement is already fulfilled.
    if selected_inputs
        .iter()
        .any(|input| is_foundry_with_id(&input.output, &foundry_id))
    {
        return Ok(Vec::new());
    }

    // Checks if the requirement can be fulfilled.
    {
        let index = available_inputs
            .iter()
            .position(|input| is_foundry_with_id(&input.output, &foundry_id));

        match index {
            // Removes the output from the available inputs and returns it, swaps to make it O(1).
            Some(index) => Ok(vec![available_inputs.swap_remove(index)]),
            None => Err(Error::UnfulfillableRequirement(Requirement::Foundry(foundry_id))),
        }
    }
}
