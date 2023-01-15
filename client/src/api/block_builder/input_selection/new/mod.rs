// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod burn;
pub(crate) mod remainder;
pub(crate) mod requirement;
pub(crate) mod transition;

use std::collections::HashSet;

pub use burn::Burn;
use remainder::remainder_output;
pub use requirement::Requirement;
use requirement::{alias::is_alias_state_transition, Requirements};

use crate::{
    block::{
        address::{Address, AliasAddress, NftAddress},
        output::{Output, OutputId},
        protocol::ProtocolParameters,
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

// TODO should ISA have its own error type? At least review errors.
// TODO make methods actually take self? There was a mut issue.

#[derive(Debug)]
pub(crate) struct OutputInfo {
    pub(crate) output: Output,
    pub(crate) provided: bool,
}

/// Working state for the input selection algorithm.
pub struct InputSelection {
    available_inputs: Vec<InputSigningData>,
    // TODO option needed ?
    required_inputs: Option<HashSet<OutputId>>,
    forbidden_inputs: HashSet<OutputId>,
    selected_inputs: Vec<InputSigningData>,
    outputs: Vec<OutputInfo>,
    burn: Option<Burn>,
    remainder_address: Option<Address>,
    protocol_parameters: ProtocolParameters,
    timestamp: u32,
}

/// Result of the input selection algorithm.
pub struct Selected {
    /// Selected inputs.
    pub inputs: Vec<InputSigningData>,
    /// Provided and created outputs.
    pub outputs: Vec<Output>,
}

impl InputSelection {
    fn required_alias_nft_addresses(&self, input: &InputSigningData) -> Result<Option<Requirement>> {
        // TODO burn?
        // TODO unwrap or false?
        // TODO this is only temporary to accommodate the current ISA.
        let outputs = self
            .outputs
            .iter()
            .map(|output| output.output.clone())
            .collect::<Vec<_>>();
        let is_alias_state_transition = is_alias_state_transition(input, &outputs)?.unwrap_or((false, false)).0;
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

    fn select_input(&mut self, input: InputSigningData, requirements: &mut Requirements) -> Result<()> {
        if let Some(output) = self.transition_input(&input)? {
            let output_info = OutputInfo {
                output,
                provided: false,
            };
            // TODO is this really necessary?
            // TODO should input be pushed before ? probably
            requirements.extend(Requirements::from_outputs(
                // TODO do we really need to chain?
                self.available_inputs.iter().chain(self.selected_inputs.iter()),
                std::iter::once(&output_info),
            ));
            self.outputs.push(output_info);
        }

        if let Some(requirement) = self.required_alias_nft_addresses(&input)? {
            requirements.push(requirement);
        }

        self.selected_inputs.push(input);

        Ok(())
    }

    // TODO rename
    fn init(&mut self) -> Result<Requirements> {
        // let mut selected_inputs = Vec::new();
        let mut requirements = Requirements::new();

        // Adds an initial base token requirement.
        requirements.push(Requirement::BaseToken);
        // Adds an initial native tokens requirement.
        requirements.push(Requirement::NativeTokens);

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
                        self.select_input(input, &mut requirements)?
                    }
                    None => return Err(Error::RequiredInputIsNotAvailable(required_input)),
                }
            }
        }

        // Gets requirements from outputs.
        // TODO this may re-evaluate outputs added by inputs
        let new_requirements = Requirements::from_outputs(
            // TODO do we really need to chain?
            self.available_inputs.iter().chain(self.selected_inputs.iter()),
            self.outputs.iter(),
        );
        requirements.extend(new_requirements);

        // Gets requirements from burn.
        if let Some(burn) = &self.burn {
            requirements.extend(Requirements::from_burn(burn));
        }

        Ok(requirements)
    }

    /// Creates a new [`InputSelection`].
    pub fn new(
        available_inputs: Vec<InputSigningData>,
        outputs: Vec<Output>,
        protocol_parameters: ProtocolParameters,
    ) -> Self {
        Self {
            available_inputs,
            required_inputs: None,
            forbidden_inputs: HashSet::new(),
            selected_inputs: Vec::new(),
            outputs: outputs
                .into_iter()
                .map(|output| OutputInfo { output, provided: true })
                .collect(),
            burn: None,
            remainder_address: None,
            protocol_parameters,
            timestamp: instant::SystemTime::now()
                .duration_since(instant::SystemTime::UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs() as u32,
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

    // TODO should we somehow enforce using filter so we don't have to use can_be_unlocked_now later everywhere ?
    /// Filters out the available inputs that
    /// - can't be unlocked by the given addresses
    /// - can't be unlocked now
    pub fn filter(self, addresses: &[Address]) -> Self {
        let _addresses = addresses
            .iter()
            // TODO meh
            .copied()
            .chain(self.available_inputs.iter().filter_map(|input| match &input.output {
                Output::Alias(output) => Some(Address::Alias(AliasAddress::from(
                    output.alias_id_non_null(input.output_id()),
                ))),
                Output::Nft(output) => Some(Address::Nft(NftAddress::from(
                    output.nft_id_non_null(input.output_id()),
                ))),
                _ => None,
            }));

        // self.available_inputs.retain(|input| input.can_be_unlocked_now(input, addresses, ???, self.time))

        self
    }

    /// Selects inputs that meet the requirements of the outputs to satisfy the semantic validation of the overall
    /// transaction. Also creates a remainder output and chain transition outputs if required.
    pub fn select(mut self) -> Result<Selected> {
        if self.available_inputs.is_empty() {
            return Err(Error::NoInputsProvided);
        }
        if self.outputs.is_empty() && self.burn.is_none() {
            return Err(Error::NoOutputsProvided);
        }

        // Creates the initial state, selected inputs and requirements, based on the provided outputs.
        let mut requirements = self.init()?;

        // Process all the requirements until there are no more.
        while let Some(requirement) = requirements.pop() {
            // Fulfill the requirement.
            let (inputs, new_requirement) = self.fulfill_requirement(requirement)?;

            if let Some(new_requirement) = new_requirement {
                requirements.push(new_requirement);
            }

            //     if !inputs.is_empty() && requirements.is_empty(){
            //         requirements.push(Requirement::BaseCoinAmount);
            //     }

            // Select suggested inputs.
            for input in inputs {
                self.select_input(input, &mut requirements)?;
            }
        }

        // self.output.extend(create_storage_deposit_return_outputs(selected_input, self.outputs));

        // // Potentially do native tokens + base coin + storage deposit here
        if let Some(output) = remainder_output(
            &self.selected_inputs,
            &self.outputs,
            self.remainder_address,
            &self.protocol_parameters,
        )? {
            self.outputs.push(OutputInfo {
                output,
                provided: false,
            })
        }

        Ok(Selected {
            inputs: self.selected_inputs,
            outputs: self.outputs.into_iter().map(|output| output.output).collect(),
        })
    }
}
