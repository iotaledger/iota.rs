// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{alias::is_alias_transition, InputSelection, Requirement};
use crate::{
    block::{
        address::Address,
        output::{AliasTransition, Output},
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

fn selected_has_ed25519_address(
    input: &InputSigningData,
    outputs: &[Output],
    address: &Address,
    timestamp: u32,
) -> bool {
    let alias_transition = is_alias_transition(input, outputs).map(|(alias_transition, _)| alias_transition);
    // PANIC: safe to unwrap as outputs with no address have been filtered out already.
    let required_address = input
        .output
        .required_and_unlocked_address(timestamp, input.output_id(), alias_transition)
        .unwrap()
        .0;

    &required_address == address
}

fn available_has_ed25519_address(
    input: &InputSigningData,
    address: &Address,
    timestamp: u32,
) -> (bool, Option<AliasTransition>) {
    if input.output.is_alias() {
        // PANIC: safe to unwrap as outputs without unlock conditions have been filtered out already.
        let unlock_conditions = input.output.unlock_conditions().unwrap();

        // PANIC: safe to unwrap as aliases have a state controller address.
        if unlock_conditions.state_controller_address().unwrap().address() == address {
            return (true, Some(AliasTransition::State));
        }

        // PANIC: safe to unwrap as aliases have a governor address.
        if unlock_conditions.governor_address().unwrap().address() == address {
            return (true, Some(AliasTransition::Governance));
        }

        (false, None)
    } else {
        let (required_address, _) = input
            .output
            .required_and_unlocked_address(timestamp, input.output_id(), None)
            .unwrap();
        (&required_address == address, None)
    }
}

impl InputSelection {
    fn fulfill_ed25519_address_requirement(
        &mut self,
        address: Address,
    ) -> Result<(Vec<(InputSigningData, Option<AliasTransition>)>, Option<Requirement>)> {
        // Checks if the requirement is already fulfilled.
        if let Some(output) = self
            .selected_inputs
            .iter()
            .find(|input| selected_has_ed25519_address(input, self.outputs.as_slice(), &address, self.timestamp))
        {
            log::debug!(
                "{address:?} sender requirement already fulfilled by {:?}",
                output.output_id()
            );
            return Ok((Vec::new(), None));
        }

        // Checks if the requirement can be fulfilled.

        // TODO bit dumb atm, need to add more possible strategies.

        // TODO check that the enumeration index is kept original and not filtered.
        // Tries to find a basic output first.
        let found = if let Some((index, _)) = self.available_inputs.iter().enumerate().find(|(_, input)| {
            input.output.is_basic() && available_has_ed25519_address(input, &address, self.timestamp).0
        }) {
            Some((index, None))
        } else {
            // TODO any preference between alias and NFT?
            // If no basic output has been found, tries the other kinds of output.
            self.available_inputs.iter().enumerate().find_map(|(index, input)| {
                if !input.output.is_basic() {
                    let (found, alias_transition) = available_has_ed25519_address(input, &address, self.timestamp);
                    if found { Some((index, alias_transition)) } else { None }
                } else {
                    None
                }
            })
        };

        match found {
            Some((index, alias_transition)) => {
                // Remove the output from the available inputs, swap to make it O(1).
                let input = self.available_inputs.swap_remove(index);

                log::debug!(
                    "{address:?} sender requirement fulfilled by {:?} (alias transition {:?})",
                    input.output_id(),
                    alias_transition
                );

                Ok((vec![(input, alias_transition)], None))
            }
            None => Err(Error::UnfulfillableRequirement(Requirement::Sender(address))),
        }
    }

    /// Fulfills a sender requirement.
    pub(crate) fn fulfill_sender_requirement(
        &mut self,
        address: Address,
    ) -> Result<(Vec<(InputSigningData, Option<AliasTransition>)>, Option<Requirement>)> {
        match address {
            Address::Ed25519(_) => self.fulfill_ed25519_address_requirement(address),
            Address::Alias(alias_address) => {
                log::debug!("Treating {address:?} sender requirement as an alias requirement");

                match self.fulfill_alias_requirement(alias_address.into_alias_id(), AliasTransition::State) {
                    Ok(res) => Ok(res),
                    Err(Error::UnfulfillableRequirement(Requirement::Alias(_, _))) => {
                        Err(Error::UnfulfillableRequirement(Requirement::Sender(address)))
                    }
                    Err(e) => Err(e),
                }
            }
            Address::Nft(nft_address) => {
                log::debug!("Treating {address:?} sender requirement as an nft requirement");

                match self.fulfill_nft_requirement(nft_address.into_nft_id()) {
                    Ok(res) => Ok(res),
                    Err(Error::UnfulfillableRequirement(Requirement::Nft(_))) => {
                        Err(Error::UnfulfillableRequirement(Requirement::Sender(address)))
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}
