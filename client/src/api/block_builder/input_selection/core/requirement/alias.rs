// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection, Requirement};
use crate::{
    block::output::{AliasId, AliasTransition, Output, OutputId},
    secret::types::InputSigningData,
};

// Returns
// - the alias transition type of a given input and outputs
// - whether the output was provided or not, to differentiate a burn from a proper governance transition
pub(crate) fn is_alias_transition(input: &InputSigningData, outputs: &[Output]) -> Option<(AliasTransition, bool)> {
    is_alias_transition_internal(&input.output, *input.output_id(), outputs)
}

pub(crate) fn is_alias_transition_internal(
    input: &Output,
    input_id: OutputId,
    outputs: &[Output],
) -> Option<(AliasTransition, bool)> {
    if let Output::Alias(alias_input) = &input {
        let alias_id = alias_input.alias_id_non_null(&input_id);
        // Checks if the alias exists in the outputs and gets the transition type.
        outputs
            .iter()
            .find_map(|output| {
                if let Output::Alias(alias_output) = output {
                    if *alias_output.alias_id() == alias_id {
                        if alias_output.state_index() == alias_input.state_index() {
                            // Governance transition.
                            Some(Some((AliasTransition::Governance, true)))
                        } else {
                            // State transition.
                            Some(Some((AliasTransition::State, true)))
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            // If the alias was not found in the outputs, it gets burned which is a governance transition.
            .unwrap_or(Some((AliasTransition::Governance, false)))
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
        alias_transition: AliasTransition,
    ) -> Result<Vec<(InputSigningData, Option<AliasTransition>)>, Error> {
        // Check that the alias is not burned when a state transition is required.
        if alias_transition.is_state()
            && self
                .burn
                .as_ref()
                .map_or(false, |burn| burn.aliases.contains(&alias_id))
        {
            return Err(Error::UnfulfillableRequirement(Requirement::Alias(
                alias_id,
                alias_transition,
            )));
        }

        let selected_input = self
            .selected_inputs
            .iter()
            .find(|input| is_alias_with_id(&input.output, input.output_id(), &alias_id));

        // If a state transition is not required and the alias has already been selected, no additional check has to be
        // performed.
        if !alias_transition.is_state() && selected_input.is_some() {
            log::debug!(
                "{alias_id:?}/{alias_transition:?} requirement already fulfilled by {:?}",
                selected_input.unwrap().output_id()
            );
            return Ok(Vec::new());
        }

        let available_index = self
            .available_inputs
            .iter()
            .position(|input| is_alias_with_id(&input.output, input.output_id(), &alias_id));

        // If the alias was not already selected and it not available, the requirement can't be fulfilled.
        if selected_input.is_none() && available_index.is_none() {
            return Err(Error::UnfulfillableRequirement(Requirement::Alias(
                alias_id,
                alias_transition,
            )));
        }

        // If a state transition is not required, we can simply select the alias.
        if !alias_transition.is_state() {
            // Remove the output from the available inputs, swap to make it O(1).
            let input = self.available_inputs.swap_remove(available_index.unwrap());

            log::debug!(
                "{alias_id:?}/{alias_transition:?} requirement fulfilled by {:?}",
                input.output_id()
            );

            // PANIC: safe to unwrap as it's been checked that it can't be None when a state transition is not required.
            return Ok(vec![(input, None)]);
        }

        // At this point, a state transition is required so we need to verify that an alias output describing a
        // governance transition was not provided.

        // PANIC: safe to unwrap as it's been checked that both can't be None at the same time.
        let input = selected_input.unwrap_or_else(|| &self.available_inputs[available_index.unwrap()]);

        if is_alias_transition(input, &self.outputs) == Some((AliasTransition::Governance, true)) {
            return Err(Error::UnfulfillableRequirement(Requirement::Alias(
                alias_id,
                alias_transition,
            )));
        }

        if let Some(available_index) = available_index {
            // Remove the output from the available inputs, swap to make it O(1).
            let input = self.available_inputs.swap_remove(available_index);

            log::debug!(
                "{alias_id:?}/{alias_transition:?} requirement fulfilled by {:?}",
                input.output_id()
            );

            return Ok(vec![(input, None)]);
        }

        log::debug!(
            "{alias_id:?}/{alias_transition:?} requirement already fulfilled by {:?}",
            selected_input.unwrap().output_id()
        );

        Ok(Vec::new())
    }
}
