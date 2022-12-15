// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{InputSelection, Requirement};
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

impl InputSelection {
    /// Fulfills a foundry requirement by selecting the appropriate foundry from the available inputs.
    pub(crate) fn fulfill_foundry_requirement(
        &mut self,
        foundry_id: FoundryId,
    ) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
        // Check if the requirement is already fulfilled.

        if self
            .selected_inputs
            .iter()
            .any(|input| is_foundry_with_id(&input.output, &foundry_id))
        {
            return Ok((Vec::new(), None));
        }

        // Check if the requirement can be fulfilled.

        let index = self
            .available_inputs
            .iter()
            .position(|input| is_foundry_with_id(&input.output, &foundry_id));

        match index {
            // Remove the output from the available inputs and return it, swap to make it O(1).
            Some(index) => Ok((vec![self.available_inputs.swap_remove(index)], None)),
            None => Err(Error::UnfulfillableRequirement(Requirement::Foundry(foundry_id))),
        }
    }
}
