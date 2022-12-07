// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{InputSelection, Requirement};
use crate::{
    block::output::{AliasId, Output, OutputId},
    error::{Error, Result},
    secret::types::InputSigningData,
};

/// Checks if an output is an alias with output ID that matches the given alias ID.
pub(crate) fn is_alias_with_id(output: &Output, output_id: &OutputId, alias_id: &AliasId) -> bool {
    if let Output::Alias(alias) = output {
        &alias.alias_id_non_null(output_id) == alias_id
    } else {
        false
    }
}

impl InputSelection {
    /// Fulfills an alias requirement by selecting the appropriate alias from the available inputs.
    pub(crate) fn fulfill_alias_requirement(
        &mut self,
        alias_id: AliasId,
        selected_inputs: &[InputSigningData],
    ) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
        // Checks if the requirement is already fulfilled.
        if selected_inputs
            .iter()
            .any(|input| is_alias_with_id(&input.output, input.output_id(), &alias_id))
        {
            return Ok((Vec::new(), None));
        }

        // Checks if the requirement can be fulfilled.
        {
            let index = self
                .available_inputs
                .iter()
                .position(|input| is_alias_with_id(&input.output, input.output_id(), &alias_id));

            match index {
                // Removes the output from the available inputs and returns it, swaps to make it O(1).
                Some(index) => Ok((vec![self.available_inputs.swap_remove(index)], None)),
                None => Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id))),
            }
        }
    }
}
