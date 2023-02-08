// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    requirement::{
        amount::amount_sums,
        native_tokens::{get_minted_and_melted_native_tokens, get_native_tokens, get_native_tokens_diff},
    },
    InputSelection,
};
use crate::{
    api::RemainderData,
    block::{
        address::{Address, Ed25519Address},
        output::{
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            BasicOutputBuilder, NativeTokensBuilder, Output,
        },
    },
    crypto::keys::slip10::Chain,
    error::{Error, Result},
};

impl InputSelection {
    // Gets the remainder address from configuration of finds one from the inputs.
    // TODO should this also look for non-ed25519 addresses?
    // TODO should this also check available inputs ? not only selected.
    fn get_remainder_address(&self) -> Option<(Address, Option<Chain>)> {
        if self.remainder_address.is_some() {
            return self.remainder_address.map(|address| (address, None));
        }

        // TODO need to check timelock/expiration

        for input in &self.selected_inputs {
            if let Some(unlock_conditions) = input.output.unlock_conditions() {
                if let Some(address_unlock_condition) = unlock_conditions.address() {
                    if address_unlock_condition.address().is_ed25519() {
                        return Some((*address_unlock_condition.address(), input.chain.clone()));
                    }
                }

                // TODO check transition type

                if let Some(governor_address_unlock_condition) = unlock_conditions.governor_address() {
                    if governor_address_unlock_condition.address().is_ed25519() {
                        return Some((*governor_address_unlock_condition.address(), input.chain.clone()));
                    }
                }

                if let Some(state_controller_address_unlock_condition) = unlock_conditions.state_controller_address() {
                    if state_controller_address_unlock_condition.address().is_ed25519() {
                        return Some((
                            *state_controller_address_unlock_condition.address(),
                            input.chain.clone(),
                        ));
                    }
                }
            }
        }

        None
    }

    pub(crate) fn remainder_amount(&self) -> Result<(u64, bool)> {
        let mut input_native_tokens = get_native_tokens(self.selected_inputs.iter().map(|input| &input.output))?;
        let mut output_native_tokens = get_native_tokens(self.outputs.iter())?;
        let (minted_native_tokens, melted_native_tokens) =
            get_minted_and_melted_native_tokens(&self.selected_inputs, self.outputs.as_slice())?;

        input_native_tokens.merge(minted_native_tokens)?;
        output_native_tokens.merge(melted_native_tokens)?;

        if let Some(burn) = self.burn.as_ref() {
            output_native_tokens.merge(NativeTokensBuilder::from(burn.native_tokens.clone()))?;
        }

        let native_tokens_diff = get_native_tokens_diff(&input_native_tokens, &output_native_tokens)?;
        let native_tokens_remainder = native_tokens_diff.is_some();

        let mut remainder_builder =
            BasicOutputBuilder::new_with_minimum_storage_deposit(self.protocol_parameters.rent_structure().clone())?
                .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(Address::from(
                    Ed25519Address::from([0; 32]),
                ))));

        if let Some(native_tokens) = native_tokens_diff {
            remainder_builder = remainder_builder.with_native_tokens(native_tokens);
        }

        Ok((
            remainder_builder
                .finish_output(self.protocol_parameters.token_supply())?
                .amount(),
            native_tokens_remainder,
        ))
    }

    pub(crate) fn remainder_and_storage_deposit_return_outputs(&self) -> Result<(Option<RemainderData>, Vec<Output>)> {
        let (inputs_sum, outputs_sum, inputs_sdr, outputs_sdr) =
            amount_sums(&self.selected_inputs, &self.outputs, self.timestamp);
        let mut storage_deposit_returns = Vec::new();

        for (address, amount) in inputs_sdr {
            let output_sdr_amount = *outputs_sdr.get(&address).unwrap_or(&0);

            if amount > output_sdr_amount {
                let diff = amount - output_sdr_amount;
                let srd_output = BasicOutputBuilder::new_with_amount(diff)?
                    .with_unlock_conditions([UnlockCondition::Address(AddressUnlockCondition::new(address))])
                    .finish_output(self.protocol_parameters.token_supply())?;

                // TODO verify_storage_deposit ?

                log::debug!("Created storage deposit return output of {diff} for {address:?}");

                storage_deposit_returns.push(srd_output);
            }
        }

        let mut input_native_tokens = get_native_tokens(self.selected_inputs.iter().map(|input| &input.output))?;
        let mut output_native_tokens = get_native_tokens(self.outputs.iter())?;
        let (minted_native_tokens, melted_native_tokens) =
            get_minted_and_melted_native_tokens(&self.selected_inputs, &self.outputs)?;

        input_native_tokens.merge(minted_native_tokens)?;
        output_native_tokens.merge(melted_native_tokens)?;

        if let Some(burn) = self.burn.as_ref() {
            output_native_tokens.merge(NativeTokensBuilder::from(burn.native_tokens.clone()))?;
        }

        let native_tokens_diff = get_native_tokens_diff(&input_native_tokens, &output_native_tokens)?;

        if inputs_sum == outputs_sum && native_tokens_diff.is_none() {
            log::debug!("No remainder required");
            return Ok((None, storage_deposit_returns));
        }

        let Some((remainder_address, chain)) = self.get_remainder_address() else {
            return Err(Error::MissingInputWithEd25519Address);
        };

        // TODO checked ops ?
        let diff = inputs_sum - outputs_sum;
        let mut remainder_builder = BasicOutputBuilder::new_with_amount(diff)?;

        remainder_builder = remainder_builder
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_address)));

        if let Some(native_tokens) = native_tokens_diff {
            log::debug!("Adding {native_tokens:?} to remainder output for {remainder_address:?}");
            remainder_builder = remainder_builder.with_native_tokens(native_tokens);
        }

        let remainder = remainder_builder.finish_output(self.protocol_parameters.token_supply())?;

        log::debug!("Created remainder output of {diff} for {remainder_address:?}");

        remainder.verify_storage_deposit(
            self.protocol_parameters.rent_structure().clone(),
            self.protocol_parameters.token_supply(),
        )?;

        Ok((
            Some(RemainderData {
                output: remainder,
                chain,
                address: remainder_address,
            }),
            storage_deposit_returns,
        ))
    }
}
