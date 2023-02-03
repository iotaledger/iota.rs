// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{InputSelection, Requirement};
use crate::{
    block::output::{AliasId, AliasTransition, Output, OutputId},
    error::{Error, Result},
    secret::types::InputSigningData,
};

// Returns
// - if alias transition is a state transition with the provided outputs for a given input
// - if the output was provided, to differentiate a burn from a proper governance transition
pub(crate) fn is_alias_state_transition(input: &InputSigningData, outputs: &[Output]) -> Option<(bool, bool)> {
    if let Output::Alias(alias_input) = &input.output {
        let alias_id = alias_input.alias_id_non_null(input.output_id());
        // Checks if the alias exists in the outputs and gets the transition type.
        outputs
            .iter()
            .find_map(|output| {
                if let Output::Alias(alias_output) = output {
                    if *alias_output.alias_id() == alias_id {
                        if alias_output.state_index() == alias_input.state_index() {
                            // Governance transition.
                            Some(Some((false, true)))
                        } else {
                            // State transition.
                            Some(Some((true, true)))
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            // If the alias was not found in the outputs, it gets burned which is a governance transition.
            .unwrap_or(Some((false, false)))
    } else {
        // Not an alias transition.
        None
    }
}

/// Checks if an output is an alias with a given non null alias ID.
/// Calling it with a null alias ID may lead to undefined behavior.
pub(crate) fn is_alias_with_id_non_null(output: &Output, alias_id: &AliasId) -> bool {
    if let Output::Alias(alias) = output {
        alias.alias_id() == alias_id
    } else {
        false
    }
}

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
        state_transition: bool,
    ) -> Result<(Vec<(InputSigningData, Option<AliasTransition>)>, Option<Requirement>)> {
        // Check that the alias is not burned when a state transition is required.
        if state_transition
            && self
                .burn
                .as_ref()
                .map_or(false, |burn| burn.aliases.contains(&alias_id))
        {
            return Err(Error::UnfulfillableRequirement(Requirement::Alias(
                alias_id,
                state_transition,
            )));
        }

        let selected_input = self
            .selected_inputs
            .iter()
            .find(|input| is_alias_with_id(&input.output, input.output_id(), &alias_id));

        // If a state transition is not required and the alias has already been selected, no additional check has to be
        // performed.
        if !state_transition && selected_input.is_some() {
            return Ok((Vec::new(), None));
        }

        let available_index = self
            .available_inputs
            .iter()
            .position(|input| is_alias_with_id(&input.output, input.output_id(), &alias_id));

        // If the alias was not already selected and it not available, the requirement can't be fulfilled.
        if selected_input.is_none() && available_index.is_none() {
            return Err(Error::UnfulfillableRequirement(Requirement::Alias(
                alias_id,
                state_transition,
            )));
        }

        // If a state transition is not required, we can simply select the alias.
        if !state_transition {
            // Remove the output from the available inputs and return it, swap to make it O(1).
            // PANIC: safe to unwrap as it's been checked that it can't be None when a state transition is not required.
            return Ok((
                vec![(self.available_inputs.swap_remove(available_index.unwrap()), None)],
                None,
            ));
        }

        // At this point, a state transition is required so we need to verify that an alias output describing a
        // governance transition was not provided.

        // PANIC: safe to unwrap as it's been checked that both can't be None at the same time.
        let input = selected_input.unwrap_or_else(|| &self.available_inputs[available_index.unwrap()]);

        if is_alias_state_transition(input, &self.outputs) == Some((false, true)) {
            return Err(Error::UnfulfillableRequirement(Requirement::Alias(
                alias_id,
                state_transition,
            )));
        }

        if let Some(available_index) = available_index {
            // Remove the output from the available inputs and return it, swap to make it O(1).
            return Ok((vec![(self.available_inputs.swap_remove(available_index), None)], None));
        }

        Ok((Vec::new(), None))
    }
}
