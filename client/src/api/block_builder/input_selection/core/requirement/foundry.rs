// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{InputSelection, Requirement};
use crate::{
    block::output::{AliasTransition, FoundryId, Output},
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
    ) -> Result<Vec<(InputSigningData, Option<AliasTransition>)>> {
        // Check if the requirement is already fulfilled.
        if let Some(output) = self
            .selected_inputs
            .iter()
            .find(|input| is_foundry_with_id(&input.output, &foundry_id))
        {
            log::debug!(
                "{foundry_id:?} requirement already fulfilled by {:?}",
                output.output_id()
            );
            return Ok(Vec::new());
        }

        // Check if the requirement can be fulfilled.
        let index = self
            .available_inputs
            .iter()
            .position(|input| is_foundry_with_id(&input.output, &foundry_id))
            .ok_or(Error::UnfulfillableRequirement(Requirement::Foundry(foundry_id)))?;
        // Remove the output from the available inputs, swap to make it O(1).
        let input = self.available_inputs.swap_remove(index);

        log::debug!("{foundry_id:?} requirement fulfilled by {:?}", input.output_id());

        Ok(vec![(input, None)])
    }
}
