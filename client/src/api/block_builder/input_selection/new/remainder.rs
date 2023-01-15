// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    requirement::{
        base_token::base_token_sums,
        native_tokens::{get_minted_and_melted_native_tokens, get_native_tokens, get_native_tokens_diff},
    },
    Burn, OutputInfo,
};
use crate::{
    block::{
        address::Address,
        output::{
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            BasicOutputBuilder, NativeTokensBuilder, Output,
        },
        protocol::ProtocolParameters,
    },
    error::{Error, Result},
    secret::types::InputSigningData,
};

// Gets the remainder address from configuration of finds one from the inputs.
// TODO should this also look for non-ed25519 addresses?
// TODO should this also check available inputs ? not only selected.
fn get_remainder_address(selected_inputs: &[InputSigningData], remainder_address: Option<Address>) -> Option<Address> {
    if remainder_address.is_some() {
        return remainder_address;
    }

    // TODO need to check timelock/expiration

    for input in selected_inputs {
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

// TODO make self
pub(crate) fn remainder_output(
    selected_inputs: &[InputSigningData],
    outputs: &[OutputInfo],
    remainder_address: Option<Address>,
    protocol_parameters: &ProtocolParameters,
    burn: Option<&Burn>,
) -> Result<Option<Output>> {
    let (inputs_sum, outputs_sum) = base_token_sums(selected_inputs, outputs);

    let mut input_native_tokens = get_native_tokens(selected_inputs.iter().map(|input| &input.output))?;
    let mut output_native_tokens = get_native_tokens(outputs.iter().map(|output| &output.output))?;
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(selected_inputs, outputs)?;

    input_native_tokens.merge(minted_native_tokens)?;
    output_native_tokens.merge(melted_native_tokens)?;

    if let Some(burn) = burn.as_ref() {
        output_native_tokens.merge(NativeTokensBuilder::from(burn.native_tokens.clone()))?;
    }

    let native_tokens_diff = get_native_tokens_diff(&input_native_tokens, &output_native_tokens)?;

    if inputs_sum > outputs_sum || native_tokens_diff.is_some() {
        let Some(remainder_address) = get_remainder_address(selected_inputs,remainder_address) else {
            return Err(Error::MissingInputWithEd25519Address);
        };

        let mut remainder_builder = if inputs_sum > outputs_sum {
            // TODO could this also fail if not enough to cover native tokens ?
            // TODO checked ops
            BasicOutputBuilder::new_with_amount(inputs_sum - outputs_sum)?
        } else {
            BasicOutputBuilder::new_with_minimum_storage_deposit(protocol_parameters.rent_structure().clone())?
        };

        remainder_builder = remainder_builder
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_address)));

        if let Some(native_tokens) = native_tokens_diff {
            remainder_builder = remainder_builder.with_native_tokens(native_tokens);
        }

        let remainder = remainder_builder.finish_output(protocol_parameters.token_supply())?;

        // TODO should we always try to select enough inputs so the diff covers the deposit?
        remainder.verify_storage_deposit(
            protocol_parameters.rent_structure().clone(),
            protocol_parameters.token_supply(),
        )?;

        return Ok(Some(remainder));
    }

    Ok(None)
}
