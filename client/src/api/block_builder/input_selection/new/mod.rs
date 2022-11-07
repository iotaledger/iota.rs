// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod burn;
mod requirement;

use burn::Burn;

use crate::block::{
    address::Address,
    output::{Output, OutputId},
    protocol::ProtocolParameters,
};

pub struct InputSelection {
    outputs: Vec<Output>,
    // TODO impl Iter ?
    available_inputs: Vec<Output>,
    protocol_parameters: ProtocolParameters,
    timestamp: Option<u32>,
    required_inputs: Vec<OutputId>,
    forbidden_inputs: Vec<OutputId>,
    remainder_address: Option<Address>,
    burn: Option<Burn>,
    // TODO: decide if we want to add the addresses here to check if we can unlock an output or not:
    // alias output can have two different addresses and expiration unlock condition can change the unlock address
    // sender_addresses: Vec<Address>,
}

impl InputSelection {
    pub fn new(outputs: Vec<Output>, available_inputs: Vec<Output>, protocol_parameters: ProtocolParameters) -> Self {
        Self {
            outputs,
            available_inputs,
            protocol_parameters,
            timestamp: None,
            required_inputs: Vec::new(),
            forbidden_inputs: Vec::new(),
            remainder_address: None,
            burn: None,
        }
    }

    pub fn time(mut self, time: u32) -> Self {
        self.timestamp.replace(time);
        self
    }

    pub fn required_inputs(mut self, inputs: Vec<OutputId>) -> Self {
        self.required_inputs = inputs;
        self
    }

    pub fn forbidden_inputs(mut self, inputs: Vec<OutputId>) -> Self {
        self.forbidden_inputs = inputs;
        self
    }

    pub fn remainder_address(mut self, address: Address) -> Self {
        self.remainder_address.replace(address);
        self
    }

    pub fn burn(mut self, burn: Burn) -> Self {
        self.burn.replace(burn);
        self
    }
}
