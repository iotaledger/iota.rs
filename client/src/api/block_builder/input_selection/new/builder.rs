// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use super::{burn::Burn, InputSelection};
use crate::{
    block::{
        address::Address,
        output::{Output, OutputId},
        protocol::ProtocolParameters,
    },
    secret::types::InputSigningData,
};

pub struct InputSelectionBuilder {
    outputs: Vec<Output>,
    // TODO impl Iter ?
    available_inputs: Vec<InputSigningData>,
    protocol_parameters: ProtocolParameters,
    timestamp: Option<u32>,
    required_inputs: HashSet<OutputId>,
    forbidden_inputs: HashSet<OutputId>,
    remainder_address: Option<Address>,
    burn: Option<Burn>,
    // TODO: decide if we want to add the addresses here to check if we can unlock an output or not:
    // alias output can have two different addresses and expiration unlock condition can change the unlock address
    // sender_addresses: Vec<Address>,
}

impl InputSelectionBuilder {
    pub fn new(
        outputs: Vec<Output>,
        available_inputs: Vec<InputSigningData>,
        protocol_parameters: ProtocolParameters,
    ) -> Self {
        Self {
            outputs,
            available_inputs,
            protocol_parameters,
            timestamp: None,
            required_inputs: HashSet::new(),
            forbidden_inputs: HashSet::new(),
            remainder_address: None,
            burn: None,
        }
    }

    pub fn time(mut self, time: u32) -> Self {
        self.timestamp.replace(time);
        self
    }

    pub fn required_inputs(mut self, inputs: HashSet<OutputId>) -> Self {
        self.required_inputs = inputs;
        self
    }

    pub fn forbidden_inputs(mut self, inputs: HashSet<OutputId>) -> Self {
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

    pub fn build(self) -> InputSelection {
        InputSelection {
            outputs: self.outputs,
            available_inputs: self.available_inputs,
            protocol_parameters: self.protocol_parameters,
            timestamp: self.timestamp.unwrap_or_else(|| {
                instant::SystemTime::now()
                    .duration_since(instant::SystemTime::UNIX_EPOCH)
                    .expect("time went backwards")
                    .as_secs() as u32
            }),
            required_inputs: self.required_inputs,
            forbidden_inputs: self.forbidden_inputs,
            remainder_address: self.remainder_address,
            burn: self.burn,
        }
    }
}
