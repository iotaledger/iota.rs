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

struct InputSelection {
    outputs: Vec<Output>,
    available_inputs: Vec<Output>,
    protocol_parameters: ProtocolParameters,
    time: u32,
    required_inputs: Vec<OutputId>,
    forbidden_inputs: Vec<OutputId>,
    remainder_address: Option<Address>,
    burn: Option<Burn>,
    // TODO: decide if we want to add the addresses here to check if we can unlock an output or not:
    // alias output can have two different addresses and expiration unlock condition can change the unlock address
    // sender_addresses: Vec<Address>,
}
