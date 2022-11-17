// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_types::block::output::Output;

use crate::{secret::types::InputSigningData, Result};

// Returns if alias transition is a state transition with the provided outputs for a given input.
pub(crate) fn is_alias_state_transition(input: &InputSigningData, outputs: &[Output]) -> Result<Option<bool>> {
    Ok(if let Output::Alias(alias_input) = &input.output {
        let alias_id = alias_input.alias_id_non_null(input.output_id());
        // Checks if the alias exists in the outputs and gets the transition type.
        outputs
            .iter()
            .find_map(|output| {
                if let Output::Alias(alias_output) = output {
                    if *alias_output.alias_id() == alias_id {
                        if alias_output.state_index() == alias_input.state_index() {
                            // Governance transition.
                            Some(Some(false))
                        } else {
                            // State transition.
                            Some(Some(true))
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            // If the alias was not found in the outputs, it gets burned which is a governance transition.
            .unwrap_or(Some(false))
    } else {
        // Not an alias transition.
        None
    })
}
