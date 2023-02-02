// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod burn;
pub(crate) mod remainder;
pub(crate) mod requirement;
pub(crate) mod transition;

use std::collections::HashSet;

use self::requirement::alias::is_alias_state_transition;
pub use self::{
    burn::{Burn, BurnDto},
    requirement::Requirement,
};
use crate::{
    api::{block_builder::input_selection::helpers::sort_input_signing_data, types::RemainderData},
    block::{
        address::{Address, AliasAddress, NftAddress},
        output::{ChainId, Output, OutputId},
        protocol::ProtocolParameters,
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

// TODO should ISA have its own error type? At least review errors.
// TODO make methods actually take self? There was a mut issue.

/// Working state for the input selection algorithm.
pub struct InputSelection {
    available_inputs: Vec<InputSigningData>,
    // TODO option needed ?
    required_inputs: Option<HashSet<OutputId>>,
    forbidden_inputs: HashSet<OutputId>,
    selected_inputs: Vec<InputSigningData>,
    outputs: Vec<Output>,
    addresses: HashSet<Address>,
    burn: Option<Burn>,
    remainder_address: Option<Address>,
    protocol_parameters: ProtocolParameters,
    timestamp: u32,
    requirements: Vec<Requirement>,
    automatically_transitioned: HashSet<ChainId>,
}

/// Result of the input selection algorithm.
#[derive(Clone, Debug)]
pub struct Selected {
    /// Selected inputs.
    pub inputs: Vec<InputSigningData>,
    /// Provided and created outputs.
    pub outputs: Vec<Output>,
    /// Remainder, if there was one.
    pub remainder: Option<RemainderData>,
}

impl InputSelection {
    fn required_alias_nft_addresses(&self, input: &InputSigningData) -> Result<Option<Requirement>> {
        // TODO burn?
        // TODO unwrap or false?
        let is_alias_state_transition = is_alias_state_transition(input, &self.outputs)?
            .unwrap_or((false, false))
            .0;
        let (required_address, _) =
            input
                .output
                .required_and_unlocked_address(self.timestamp, input.output_id(), is_alias_state_transition)?;

        match required_address {
            Address::Alias(alias_address) => Ok(Some(Requirement::Alias(*alias_address.alias_id(), true))),
            Address::Nft(nft_address) => Ok(Some(Requirement::Nft(*nft_address.nft_id()))),
            _ => Ok(None),
        }
    }

    fn select_input(&mut self, input: InputSigningData, governance_transition: bool) -> Result<()> {
        if let Some(output) = self.transition_input(&input, governance_transition)? {
            // TODO is this really necessary?
            // TODO should input be pushed before ? probably
            self.outputs_requirements(Some(&output));
            self.outputs.push(output);
        }

        if let Some(requirement) = self.required_alias_nft_addresses(&input)? {
            self.requirements.push(requirement);
        }

        self.selected_inputs.push(input);

        Ok(())
    }

    // TODO rename
    fn init(&mut self) -> Result<()> {
        // Adds an initial amount requirement.
        self.requirements.push(Requirement::Amount);
        // Adds an initial native tokens requirement.
        self.requirements.push(Requirement::NativeTokens);

        // Removes forbidden inputs from available inputs.
        self.available_inputs
            .retain(|input| !self.forbidden_inputs.contains(input.output_id()));

        // The `take` avoids a mutable borrow compilation issue without having to clone the required inputs.
        // TODO could be reworked by having select_input not taking mut.
        if let Some(required_inputs) = self.required_inputs.take() {
            for required_input in required_inputs.into_iter() {
                // Checks that required input is not forbidden.
                if self.forbidden_inputs.contains(&required_input) {
                    return Err(Error::RequiredInputIsForbidden(required_input));
                }

                // Checks that required input is available.
                match self
                    .available_inputs
                    .iter()
                    .position(|input| input.output_id() == &required_input)
                {
                    Some(index) => {
                        // Removes required input from available inputs.
                        let input = self.available_inputs.swap_remove(index);

                        // Selects required input.
                        self.select_input(input, false)?
                    }
                    None => return Err(Error::RequiredInputIsNotAvailable(required_input)),
                }
            }
        }

        // Gets requirements from outputs.
        // TODO this may re-evaluate outputs added by inputs
        self.outputs_requirements(None);

        // Gets requirements from burn.
        self.burn_requirements()?;

        Ok(())
    }

    /// Creates a new [`InputSelection`].
    pub fn new(
        available_inputs: Vec<InputSigningData>,
        outputs: Vec<Output>,
        addresses: Vec<Address>,
        protocol_parameters: ProtocolParameters,
    ) -> Self {
        let mut addresses = HashSet::from_iter(addresses);

        addresses.extend(available_inputs.iter().filter_map(|input| match &input.output {
            Output::Alias(output) => Some(Address::Alias(AliasAddress::from(
                output.alias_id_non_null(input.output_id()),
            ))),
            Output::Nft(output) => Some(Address::Nft(NftAddress::from(
                output.nft_id_non_null(input.output_id()),
            ))),
            _ => None,
        }));

        Self {
            available_inputs,
            required_inputs: None,
            forbidden_inputs: HashSet::new(),
            selected_inputs: Vec::new(),
            outputs,
            addresses,
            burn: None,
            remainder_address: None,
            protocol_parameters,
            timestamp: instant::SystemTime::now()
                .duration_since(instant::SystemTime::UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs() as u32,
            requirements: Vec::new(),
            automatically_transitioned: HashSet::new(),
        }
    }

    /// Sets the required inputs of an [`InputSelection`].
    pub fn required_inputs(mut self, inputs: HashSet<OutputId>) -> Self {
        self.required_inputs.replace(inputs);
        self
    }

    /// Sets the forbidden inputs of an [`InputSelection`].
    pub fn forbidden_inputs(mut self, inputs: HashSet<OutputId>) -> Self {
        self.forbidden_inputs = inputs;
        self
    }

    /// Sets the burn of an [`InputSelection`].
    pub fn burn(mut self, burn: Burn) -> Self {
        self.burn.replace(burn);
        self
    }

    /// Sets the remainder address of an [`InputSelection`].
    pub fn remainder_address(mut self, address: Address) -> Self {
        self.remainder_address.replace(address);
        self
    }

    /// Sets the timestamp of an [`InputSelection`].
    pub fn timestamp(mut self, timestamp: u32) -> Self {
        self.timestamp = timestamp;
        self
    }

    fn filter_inputs(&mut self) {
        self.available_inputs.retain(|input| {
            // Keep alias outputs because at this point we do not know if a state or governor address will be required.
            if input.output.is_alias() {
                return true;
            }
            // Filter out non basic/foundry/nft outputs.
            else if !input.output.is_basic() && !input.output.is_foundry() && !input.output.is_nft() {
                return false;
            }

            // PANIC: safe to unwrap as non basic/alias/foundry/nft outputs are already filtered out.
            let unlock_conditions = input.output.unlock_conditions().unwrap();

            if unlock_conditions.is_time_locked(self.timestamp) {
                return false;
            }

            let required_address = input
                .output
                // True is irrelevant here as we keep aliases anyway.
                .required_and_unlocked_address(self.timestamp, input.output_id(), true)
                // PANIC: safe to unwrap as non basic/alias/foundry/nft outputs are already filtered out.
                .unwrap()
                .0;

            self.addresses.contains(&required_address)
        })
    }

    /// Selects inputs that meet the requirements of the outputs to satisfy the semantic validation of the overall
    /// transaction. Also creates a remainder output and chain transition outputs if required.
    pub fn select(mut self) -> Result<Selected> {
        self.filter_inputs();

        if self.available_inputs.is_empty() {
            return Err(Error::NoAvailableInputsProvided);
        }
        if self.outputs.is_empty() && self.burn.is_none() {
            return Err(Error::NoOutputsProvided);
        }

        // Creates the initial state, selected inputs and requirements, based on the provided outputs.
        self.init()?;

        // Process all the requirements until there are no more.
        while let Some(requirement) = self.requirements.pop() {
            // Fulfill the requirement.
            let (inputs, new_requirement) = self.fulfill_requirement(requirement)?;

            if let Some(new_requirement) = new_requirement {
                self.requirements.push(new_requirement);
            }

            // Select suggested inputs.
            for (input, governance_transition) in inputs {
                self.select_input(input, governance_transition)?;
            }
        }

        let (remainder, storage_deposit_returns) = self.remainder_and_storage_deposit_return_outputs()?;

        if let Some(remainder) = &remainder {
            self.outputs.push(remainder.output.clone());
        }

        self.outputs.extend(storage_deposit_returns);

        Ok(Selected {
            inputs: sort_input_signing_data(self.selected_inputs)?,
            outputs: self.outputs,
            remainder,
        })
    }
}
