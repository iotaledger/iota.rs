// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod builder;
pub(crate) mod burn;
pub(crate) mod helper;
pub(crate) mod remainder;
pub(crate) mod requirement;
pub(crate) mod transition;

use std::collections::HashSet;

pub use builder::InputSelectionBuilder;
pub use burn::Burn;
use helper::is_alias_state_transition;
use remainder::remainder_output;
pub use requirement::Requirement;
use requirement::Requirements;
use transition::transition_input;

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
    outputs: Vec<OutputInfo>,
    // TODO impl Iter instead?
    available_inputs: Vec<InputSigningData>,
    protocol_parameters: ProtocolParameters,
    timestamp: u32,
    required_inputs: HashSet<OutputId>,
    forbidden_inputs: HashSet<OutputId>,
    remainder_address: Option<Address>,
    burn: Option<Burn>,
    // TODO: decide if we want to add the addresses here to check if we can unlock an output or not:
    // alias output can have two different addresses and expiration unlock condition can change the unlock address
    // sender_addresses: Vec<Address>,
}

impl InputSelection {
    fn required_address(
        input: &InputSigningData,
        outputs: &[OutputInfo],
        timestamp: u32,
    ) -> Result<Option<Requirement>> {
        // TODO burn?
        // TODO unwrap or false?
        // TODO this is only temporary to accommodate the current ISA.
        let outputs = outputs.iter().map(|output| output.output.clone()).collect::<Vec<_>>();
        let is_alias_state_transition = is_alias_state_transition(input, &outputs)?.unwrap_or(false);
        let (required_address, _) =
            input
                .output
                .required_and_unlocked_address(timestamp, input.output_id(), is_alias_state_transition)?;

        match required_address {
            Address::Alias(alias_address) => Ok(Some(Requirement::Alias(*alias_address.alias_id()))),
            Address::Nft(nft_address) => Ok(Some(Requirement::Nft(*nft_address.nft_id()))),
            _ => Ok(None),
        }
    }

    fn select_input(
        selected_inputs: &mut Vec<InputSigningData>,
        input: InputSigningData,
        outputs: &mut Vec<OutputInfo>,
        requirements: &mut Requirements,
        burn: Option<&Burn>,
        timestamp: u32,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<()> {
        if let Some(output) = transition_input(&input, outputs, burn, protocol_parameters)? {
            let output_info = OutputInfo {
                output,
                provided: false,
            };
            // TODO is this really necessary?
            // TODO should input be pushed before ? probably
            requirements.extend(Requirements::from_outputs(
                selected_inputs.iter(),
                std::iter::once(&output_info),
            ));
            outputs.push(output_info);
        }

        if let Some(requirement) = Self::required_address(&input, outputs, timestamp)? {
            requirements.push(requirement);
        }

        selected_inputs.push(input);

        Ok(())
    }

    // TODO rename
    fn init(&mut self) -> Result<(Vec<InputSigningData>, Requirements)> {
        let mut selected_inputs = Vec::new();
        let mut requirements = Requirements::new();

        // Removes forbidden inputs from available inputs.
        self.available_inputs
            .retain(|input| !self.forbidden_inputs.contains(input.output_id()));

        for required_input in self.required_inputs.iter() {
            // Checks that required input is not forbidden.
            if self.forbidden_inputs.contains(required_input) {
                return Err(Error::RequiredInputIsForbidden(*required_input));
            }

            // Checks that required input is available.
            match self
                .available_inputs
                .iter()
                .position(|input| input.output_id() == required_input)
            {
                Some(index) => {
                    // Removes required input from available inputs.
                    let input = self.available_inputs.swap_remove(index);

                    // Selects required input.
                    Self::select_input(
                        &mut selected_inputs,
                        input,
                        &mut self.outputs,
                        &mut requirements,
                        self.burn.as_ref(),
                        self.timestamp,
                        &self.protocol_parameters,
                    )?
                }
                None => return Err(Error::RequiredInputIsNotAvailable(*required_input)),
            }
        }

        // Gets requirements from outputs.
        // TODO this may re-evaluate outputs added by inputs
        let new_requirements = Requirements::from_outputs(selected_inputs.iter(), self.outputs.iter());
        println!("new requirements from outputs: {:?}", new_requirements);
        requirements.extend(new_requirements);

        // Gets requirements from burn.
        if let Some(burn) = &self.burn {
            let new_requirements = Requirements::from_burn(burn);
            println!("new requirements from burn: {:?}", new_requirements);
            requirements.extend(new_requirements);
        }

        // Adds an initial native tokens requirement.
        requirements.push(Requirement::NativeTokens);
        // Adds an initial base token requirement.
        requirements.push(Requirement::BaseToken);

        Ok((selected_inputs, requirements))
    }

    /// Creates an [`InputSelectionBuilder`].
    pub fn build(
        outputs: Vec<Output>,
        available_inputs: Vec<InputSigningData>,
        protocol_parameters: ProtocolParameters,
    ) -> InputSelectionBuilder {
        InputSelectionBuilder::new(outputs, available_inputs, protocol_parameters)
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
    /// transaction. Also creates outputs if transitions are required.
    pub fn select(mut self) -> Result<(Vec<InputSigningData>, Vec<Output>)> {
        // Creates the initial state, selected inputs and requirements, based on the provided outputs.
        let (mut selected_inputs, mut requirements) = self.init()?;

        // Process all the requirements until there are no more.
        while let Some(requirement) = requirements.pop() {
            // Fulfill the requirement.
            let (inputs, new_requirement) = self.fulfill_requirement(requirement, &selected_inputs)?;

            if let Some(new_requirement) = new_requirement {
                println!("NEW REQUIREMENT");
                requirements.push(new_requirement);
            }

            //     if !inputs.is_empty() && requirements.is_empty(){
            //         requirements.push(Requirement::BaseCoinAmount);
            //     }

            // Select suggested inputs.
            for input in inputs {
                Self::select_input(
                    &mut selected_inputs,
                    input,
                    &mut self.outputs,
                    &mut requirements,
                    self.burn.as_ref(),
                    self.timestamp,
                    &self.protocol_parameters,
                )?;
            }
        }

        // self.output.extend(create_storage_deposit_return_outputs(selected_input, self.outputs));

        // // Potentially do native tokens + base coin + storage deposit here
        if let Some(output) = remainder_output(
            &selected_inputs,
            &self.outputs,
            self.remainder_address,
            &self.protocol_parameters,
        )? {
            self.outputs.push(OutputInfo {
                output,
                provided: false,
            })
        }

        Ok((
            selected_inputs,
            self.outputs.into_iter().map(|output| output.output).collect(),
        ))
    }
}
