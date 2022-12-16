// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use super::{burn::Burn, InputSelection, OutputInfo};
use crate::{
    block::{
        address::Address,
        output::{Output, OutputId},
        protocol::ProtocolParameters,
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

/// A builder for an [`InputSelection`].
pub struct InputSelectionBuilder {
    available_inputs: Vec<InputSigningData>,
    required_inputs: HashSet<OutputId>,
    forbidden_inputs: HashSet<OutputId>,
    outputs: Vec<Output>,
    burn: Option<Burn>,
    remainder_address: Option<Address>,
    protocol_parameters: ProtocolParameters,
    timestamp: Option<u32>,
}

impl InputSelectionBuilder {
    /// Creates a new [`InputSelectionBuilder`].
    pub fn new(
        available_inputs: Vec<InputSigningData>,
        outputs: Vec<Output>,
        protocol_parameters: ProtocolParameters,
    ) -> Self {
        Self {
            available_inputs,
            required_inputs: HashSet::new(),
            forbidden_inputs: HashSet::new(),
            outputs,
            burn: None,
            remainder_address: None,
            protocol_parameters,
            timestamp: None,
        }
    }

    /// Sets the required inputs of an [`InputSelectionBuilder`].
    pub fn required_inputs(mut self, inputs: HashSet<OutputId>) -> Self {
        self.required_inputs = inputs;
        self
    }

    /// Sets the forbidden inputs of an [`InputSelectionBuilder`].
    pub fn forbidden_inputs(mut self, inputs: HashSet<OutputId>) -> Self {
        self.forbidden_inputs = inputs;
        self
    }

    /// Sets the burn of an [`InputSelectionBuilder`].
    pub fn burn(mut self, burn: Burn) -> Self {
        self.burn.replace(burn);
        self
    }

    /// Sets the remainder address of an [`InputSelectionBuilder`].
    pub fn remainder_address(mut self, address: Address) -> Self {
        self.remainder_address.replace(address);
        self
    }

    /// Sets the timestamp of an [`InputSelectionBuilder`].
    pub fn timestamp(mut self, time: u32) -> Self {
        self.timestamp.replace(time);
        self
    }

    /// Finishes an [`InputSelectionBuilder`] into an [`InputSelection`].
    pub fn finish(self) -> Result<InputSelection> {
        if self.available_inputs.is_empty() {
            return Err(Error::NoInputsProvided);
        }
        if self.outputs.is_empty() {
            return Err(Error::NoOutputsProvided);
        }

        Ok(InputSelection {
            available_inputs: self.available_inputs,
            required_inputs: Some(self.required_inputs),
            forbidden_inputs: self.forbidden_inputs,
            selected_inputs: Vec::new(),
            outputs: self
                .outputs
                .into_iter()
                .map(|output| OutputInfo { output, provided: true })
                .collect(),
            burn: self.burn,
            remainder_address: self.remainder_address,
            protocol_parameters: self.protocol_parameters,
            timestamp: self.timestamp.unwrap_or_else(|| {
                instant::SystemTime::now()
                    .duration_since(instant::SystemTime::UNIX_EPOCH)
                    .expect("time went backwards")
                    .as_secs() as u32
            }),
        })
    }
}
