// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod builder;
pub(crate) mod burn;
pub(crate) mod remainder;
pub(crate) mod requirement;

use std::collections::HashSet;

pub use builder::InputSelectionBuilder;
pub use burn::Burn;
use remainder::remainder_output;
use requirement::{Requirement, Requirements};

use crate::{
    block::{
        address::{Address, AliasAddress, NftAddress},
        output::{Output, OutputId},
        protocol::ProtocolParameters,
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

pub struct InputSelection {
    outputs: Vec<Output>,
    // TODO impl Iter ?
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
    fn transition_input(input: &InputSigningData, outputs: &[Output], burn: Option<&Burn>) -> Option<Output> {
        // match input {
        //     Output::Alias(alias_output) => {
        //         if burn.aliases.contains(alias_id) {
        //             return None;
        //         }

        //         Some(
        //                     // TODO create output from input
        //                 )
        //     }
        //     Output::Nft(nft_input) => {
        //         if burn.nfts.contains(nft_id) {
        //             return None;
        //         }

        //         Some(
        //                     // TODO create output from input
        //                 )
        //     }
        //     Output::Foundry(foundry_output) => {
        //         if burn.foundries.contains(foundry_id) {
        //             return None;
        //         }

        //         Some(
        //                     // TODO create output from input
        //                 )
        //     }
        //     _ => None,
        // }

        None
    }

    fn unlock_conditions_input(
        input: &InputSigningData,
        outputs: &[Output],
        burn: Option<&Burn>,
    ) -> Option<Requirement> {
        // let alias_state_transition = alias_state_transition(input, outputs);
        // let required_address = input.required_and_unlock_address(time, alias_state_transition).0;

        // match required_address {
        //     Address::Alias(alias_address) => Ok(Some(Requirement::Alias(*alias_address.alias_id()))),
        //     Address::Nft(nft_address) => Ok(Some(Requirement::Nft(*nft_address.nft_id()))),
        //     _ => Ok(None),
        // }

        None
    }

    fn select_input(
        selected_inputs: &mut Vec<InputSigningData>,
        input: InputSigningData,
        outputs: &mut Vec<Output>,
        requirements: &mut Requirements,
        burn: Option<&Burn>,
    ) {
        if let Some(output) = Self::transition_input(&input, outputs, burn) {
            requirements.extend(Requirements::from_outputs(
                selected_inputs.iter(),
                std::iter::once(&output),
            ));
            outputs.push(output);
        }

        if let Some(requirement) = Self::unlock_conditions_input(&input, outputs, burn) {
            requirements.push(requirement);
        }

        selected_inputs.push(input)
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
    pub fn filter(mut self, addresses: &[Address]) -> Self {
        let addresses = addresses
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
        let mut selected_inputs = Vec::new();
        let mut requirements = Requirements::new();

        // Remove forbidden inputs from available inputs.
        self.available_inputs
            .retain(|input| !self.forbidden_inputs.contains(input.output_id()));

        println!("{:?}", self.available_inputs);

        for required_input in self.required_inputs.iter() {
            // Check that required inputs are not forbidden.
            if self.forbidden_inputs.contains(&required_input) {
                return Err(Error::RequiredInputIsForbidden(*required_input));
            }

            // Check that required inputs are available.
            match self
                .available_inputs
                .iter()
                .position(|input| input.output_id() == required_input)
            {
                // Select required inputs.
                Some(index) => Self::select_input(
                    &mut selected_inputs,
                    self.available_inputs.swap_remove(index),
                    &mut self.outputs,
                    &mut requirements,
                    self.burn.as_ref(),
                ),
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

        // Adds an initial base token requirement.
        requirements.push(Requirement::BaseToken);

        // Process all the requirements until there are no more.
        while let Some(requirement) = requirements.pop() {
            // Fulfill the requirement.
            let inputs = requirement.fulfill(&mut self.available_inputs, &selected_inputs, &self.outputs)?;

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
                );
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
            self.outputs.push(output)
        }

        Ok((selected_inputs, self.outputs))
    }
}
