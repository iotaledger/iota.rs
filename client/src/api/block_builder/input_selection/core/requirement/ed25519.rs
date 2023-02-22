// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{alias::is_alias_transition, Error, InputSelection, Requirement};
use crate::{
    block::{address::Address, output::AliasTransition},
    secret::types::InputSigningData,
};

impl InputSelection {
    // Checks if a selected input unlocks a given ED25519 address.
    fn selected_unlocks_ed25519_address(&self, input: &InputSigningData, address: &Address) -> bool {
        let alias_transition =
            is_alias_transition(input, self.outputs.as_slice()).map(|(alias_transition, _)| alias_transition);

        // PANIC: safe to unwrap as outputs with no address have been filtered out already.
        let required_address = input
            .output
            .required_and_unlocked_address(self.timestamp, input.output_id(), alias_transition)
            .unwrap()
            .0;

        if alias_transition.is_some() {
            // Only check if we own the required address if the input is an alias because other types of output have
            // been filtered by address already.
            &required_address == address && self.addresses.contains(address)
        } else {
            &required_address == address
        }
    }

    // Checks if an available input can unlock a given ED25519 address.
    // In case an alias input is selected, also tells if it needs to be state or governance transitioned.
    fn available_has_ed25519_address(
        &self,
        input: &InputSigningData,
        address: &Address,
    ) -> (bool, Option<AliasTransition>) {
        if input.output.is_alias() {
            // PANIC: safe to unwrap as outputs without unlock conditions have been filtered out already.
            let unlock_conditions = input.output.unlock_conditions().unwrap();

            // PANIC: safe to unwrap as aliases have a state controller address.
            if unlock_conditions.state_controller_address().unwrap().address() == address {
                return (self.addresses.contains(address), Some(AliasTransition::State));
            }

            // PANIC: safe to unwrap as aliases have a governor address.
            if unlock_conditions.governor_address().unwrap().address() == address {
                return (self.addresses.contains(address), Some(AliasTransition::Governance));
            }

            (false, None)
        } else {
            let (required_address, _) = input
                .output
                .required_and_unlocked_address(self.timestamp, input.output_id(), None)
                .unwrap();

            (&required_address == address, None)
        }
    }

    /// Fulfills an ed25519 sender requirement by selecting an available input that unlocks its address.
    pub(crate) fn fulfill_ed25519_requirement(
        &mut self,
        address: Address,
    ) -> Result<Vec<(InputSigningData, Option<AliasTransition>)>, Error> {
        // Checks if the requirement is already fulfilled.
        if let Some(input) = self
            .selected_inputs
            .iter()
            .find(|input| self.selected_unlocks_ed25519_address(input, &address))
        {
            log::debug!(
                "{address:?} sender requirement already fulfilled by {:?}",
                input.output_id()
            );
            return Ok(vec![]);
        }

        // Checks if the requirement can be fulfilled by a basic output.
        let found = if let Some((index, _)) = self
            .available_inputs
            .iter()
            .enumerate()
            .find(|(_, input)| input.output.is_basic() && self.available_has_ed25519_address(input, &address).0)
        {
            Some((index, None))
        } else {
            // Otherwise, checks if the requirement can be fulfilled by a non-basic output.
            self.available_inputs.iter().enumerate().find_map(|(index, input)| {
                if !input.output.is_basic() {
                    if let (true, alias_transition) = self.available_has_ed25519_address(input, &address) {
                        Some((index, alias_transition))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        };

        match found {
            Some((index, alias_transition)) => {
                // Remove the input from the available inputs, swap to make it O(1).
                let input = self.available_inputs.swap_remove(index);

                log::debug!(
                    "{address:?} sender requirement fulfilled by {:?} (alias transition {:?})",
                    input.output_id(),
                    alias_transition
                );

                Ok(vec![(input, alias_transition)])
            }
            None => Err(Error::UnfulfillableRequirement(Requirement::Ed25519(address))),
        }
    }
}
