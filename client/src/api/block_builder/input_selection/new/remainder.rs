// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    requirement::{
        base_token::base_token_sums,
        native_tokens::{get_minted_and_melted_native_tokens, get_native_tokens, get_native_tokens_diff},
    },
    InputSelection, OutputInfo,
};
use crate::{
    block::{
        address::Address,
        output::{
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            BasicOutputBuilder, NativeTokensBuilder,
        },
    },
    error::{Error, Result},
};

impl InputSelection {
    // Gets the remainder address from configuration of finds one from the inputs.
    // TODO should this also look for non-ed25519 addresses?
    // TODO should this also check available inputs ? not only selected.
    fn get_remainder_address(&self) -> Option<Address> {
        if self.remainder_address.is_some() {
            return self.remainder_address;
        }

        // TODO need to check timelock/expiration

        for input in &self.selected_inputs {
            if let Some(unlock_conditions) = input.output.unlock_conditions() {
                if let Some(address_unlock_condition) = unlock_conditions.address() {
                    if address_unlock_condition.address().is_ed25519() {
                        return Some(*address_unlock_condition.address());
                    }
                }

                // TODO check transition type

                if let Some(governor_address_unlock_condition) = unlock_conditions.governor_address() {
                    if governor_address_unlock_condition.address().is_ed25519() {
                        return Some(*governor_address_unlock_condition.address());
                    }
                }

                if let Some(state_controller_address_unlock_condition) = unlock_conditions.state_controller_address() {
                    if state_controller_address_unlock_condition.address().is_ed25519() {
                        return Some(*state_controller_address_unlock_condition.address());
                    }
                }
            }
        }

        None
    }

    pub(crate) fn remainder_and_storage_deposit_return_outputs(&self) -> Result<Vec<OutputInfo>> {
        let (inputs_sum, mut outputs_sum, inputs_sdr, outputs_sdr) =
            base_token_sums(&self.selected_inputs, &self.outputs);
        let mut new_outputs = Vec::new();

        for (address, amount) in inputs_sdr {
            let output_sdr_amount = *outputs_sdr.get(&address).unwrap_or(&0);

            if amount > output_sdr_amount {
                let diff = amount - output_sdr_amount;
                let srd_output = BasicOutputBuilder::new_with_amount(diff)?
                    .with_unlock_conditions([UnlockCondition::Address(AddressUnlockCondition::new(address))])
                    .finish_output(self.protocol_parameters.token_supply())?;

                new_outputs.push(OutputInfo {
                    output: srd_output,
                    provided: false,
                });
                outputs_sum += diff;
            }
        }

        let mut input_native_tokens = get_native_tokens(self.selected_inputs.iter().map(|input| &input.output))?;
        let mut output_native_tokens = get_native_tokens(self.outputs.iter().map(|output| &output.output))?;
        let (minted_native_tokens, melted_native_tokens) =
            get_minted_and_melted_native_tokens(&self.selected_inputs, &self.outputs)?;

        input_native_tokens.merge(minted_native_tokens)?;
        output_native_tokens.merge(melted_native_tokens)?;

        if let Some(burn) = self.burn.as_ref() {
            output_native_tokens.merge(NativeTokensBuilder::from(burn.native_tokens.clone()))?;
        }

        let native_tokens_diff = get_native_tokens_diff(&input_native_tokens, &output_native_tokens)?;

        if inputs_sum > outputs_sum || native_tokens_diff.is_some() {
            let Some(remainder_address) = self.get_remainder_address() else {
            return Err(Error::MissingInputWithEd25519Address);
        };

            let mut remainder_builder = if inputs_sum > outputs_sum {
                // TODO could this also fail if not enough to cover native tokens ?
                // TODO checked ops
                BasicOutputBuilder::new_with_amount(inputs_sum - outputs_sum)?
            } else {
                BasicOutputBuilder::new_with_minimum_storage_deposit(self.protocol_parameters.rent_structure().clone())?
            };

            remainder_builder = remainder_builder
                .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_address)));

            if let Some(native_tokens) = native_tokens_diff {
                remainder_builder = remainder_builder.with_native_tokens(native_tokens);
            }

            let remainder = remainder_builder.finish_output(self.protocol_parameters.token_supply())?;

            // TODO should we always try to select enough inputs so the diff covers the deposit?
            remainder.verify_storage_deposit(
                self.protocol_parameters.rent_structure().clone(),
                self.protocol_parameters.token_supply(),
            )?;

            new_outputs.push(OutputInfo {
                output: remainder,
                provided: false,
            });
        }

        Ok(new_outputs)
    }
}
