// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod builder;
pub(crate) mod burn;
pub(crate) mod requirement;

use std::collections::HashSet;

use burn::Burn;
use requirement::Requirements;

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
    pub fn filter(mut self, addresses: &[Address]) -> Self {
        let addresses = addresses
            .iter()
            // TODO meh
            .copied()
            .chain(self.available_inputs.iter().filter_map(|input| match &input.output {
                Output::Alias(output) => Some(Address::Alias(AliasAddress::from(
                    output.alias_id().or_from_output_id(*input.output_id()),
                ))),
                Output::Nft(output) => Some(Address::Nft(NftAddress::from(
                    output.nft_id().or_from_output_id(*input.output_id()),
                ))),
                _ => None,
            }));

        // self.available_inputs.retain(|input| input.can_be_unlocked_now(input, addresses, ???, self.time))

        self
    }

    pub fn select(mut self) -> Result<(Vec<InputSigningData>, Vec<Output>)> {
        // Check that no required input is also forbidden.
        for required_input in self.required_inputs.iter() {
            if self.forbidden_inputs.contains(&required_input) {
                return Err(Error::RequiredAndForbiddenInput(*required_input));
            }
        }

        // TODO could be part of filter?
        // Remove forbidden inputs from available inputs.
        self.available_inputs
            .retain(|input| !self.forbidden_inputs.contains(input.output_id()));

        // // TODO dumb check that all required are actually available

        let mut selected_inputs = Vec::new();

        // let (mut selected_inputs, self.available_inputs) = self.available_inputs.into_iter().partition(|input| !self.required_inputs.contains(input.output_id()));

        // for available_input in self.available_inputs {
        //     let output_id = available_input.output_id();
        //     if self.required_inputs.contains(&output_id) {
        //         selected_inputs.push(available_input);
        //         self.required_inputs.remove(&output_id);
        //     }
        // }

        let mut requirements = Requirements::new();

        // for selected_input in selected_inputs.iter() {
        //     let (output, requirement) = process_input(selected_input, &outputs, &self.burn);

        //     if let Some(output) = output {
        //         self.outputs.push(output);
        //     }

        //     if let Some(requirement) = requirement {
        //         requirements.push(requirement);
        //     }
        // }

        // TODO do we actually need extend?
        requirements.extend(Requirements::from_inputs_outputs(
            selected_inputs.as_slice(),
            self.outputs.as_slice(),
        ));

        // if let Some(burn) = self.burn {
        //     requirements.extend(requirements_from_burn(self.burn));
        // }
        // requirements.push_back(Requirement::BaseCoinAmount);

        // Process all the requirements until there are no more.
        while let Some(requirement) = requirements.pop() {
            //     let inputs = fulfill_requirement(requirement, &available_inputs, &selected_inputs, &mut self.outputs)?;
            //     if !inputs.is_empty() && requirements.is_empty(){
            //         requirements.push(Requirement::BaseCoinAmount);
            //     }
            //     selected_inputs.extend(inputs);
            // TODO remove from available ?
            //     for input in inputs{
            //         let (output, requirement) = process_input(input, &outputs, &self.burn);

            //         if let Some(output) = output {
            //             self.outputs.push(output);
            //             let new_requirements = requirements_from_outputs(vec![output]);
            //             requirements.push_front(new_requirements);
            //         }

            //         if let Some(requirement) = requirement {
            //             requirements.push(requirement);
            //         }
            //     }
        }

        // self.output.extend(create_storage_deposit_return_outputs(selected_input, self.outputs));

        // // Potentially do native tokens + base coin + storage deposit here
        // if let Some(remainder) = create_remainder_output(selected_inputs, self.outputs, self.remainder_address) {
        //     self.outputs.push(remainder)
        // }

        Ok((selected_inputs, self.outputs))
    }
}
