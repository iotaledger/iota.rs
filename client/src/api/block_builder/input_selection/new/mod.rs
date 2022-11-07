// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod builder;
mod burn;
mod requirement;

use burn::Burn;

use crate::{
    block::{
        address::{Address, AliasAddress, NftAddress},
        output::{Output, OutputId},
        protocol::ProtocolParameters,
    },
    secret::types::InputSigningData,
};

pub struct InputSelection {
    outputs: Vec<Output>,
    // TODO impl Iter ?
    available_inputs: Vec<InputSigningData>,
    protocol_parameters: ProtocolParameters,
    timestamp: u32,
    required_inputs: Vec<OutputId>,
    forbidden_inputs: Vec<OutputId>,
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
                // TODO unwrap ?
                Output::Alias(output) => Some(Address::Alias(AliasAddress::from(
                    output.alias_id().or_from_output_id(input.output_id().unwrap()),
                ))),
                Output::Nft(output) => Some(Address::Nft(NftAddress::from(
                    output.nft_id().or_from_output_id(input.output_id().unwrap()),
                ))),
                _ => None,
            }));

        // self.available_inputs.retain(|input| input.can_be_unlocked_now(input, addresses, ???, self.time))

        self
    }
}
