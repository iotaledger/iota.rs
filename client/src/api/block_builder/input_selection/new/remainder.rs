// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    requirement::{
        base_token::base_token_sums,
        native_tokens::{get_minted_and_melted_native_tokens, get_native_tokens, get_native_tokens_diff},
    },
    OutputInfo,
};
use crate::{
    block::{
        address::Address,
        output::{
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            BasicOutputBuilder, Output,
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

    // TODO need to check timelock/expiration?

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

pub(crate) fn remainder_output(
    selected_inputs: &[InputSigningData],
    outputs: &[OutputInfo],
    remainder_address: Option<Address>,
    protocol_parameters: &ProtocolParameters,
) -> Result<Option<Output>> {
    let (inputs_sum, outputs_sum) = base_token_sums(selected_inputs, outputs);

    let mut input_native_tokens = get_native_tokens(selected_inputs.iter().map(|input| &input.output))?;
    let mut output_native_tokens = get_native_tokens(outputs.iter().map(|output| &output.output))?;
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(selected_inputs, &outputs)?;

    println!("Input {input_native_tokens:?}");
    println!("Output {output_native_tokens:?}");
    println!("Minted {minted_native_tokens:?}");
    println!("Melted {melted_native_tokens:?}");

    input_native_tokens.merge(minted_native_tokens)?;
    output_native_tokens.merge(melted_native_tokens)?;
    // TODO also merge burn

    println!("Input merged {input_native_tokens:?}");
    println!("Output merged {output_native_tokens:?}");

    let native_tokens_diff = get_native_tokens_diff(&input_native_tokens, &output_native_tokens)?;

    println!("Diff {native_tokens_diff:?}");

    // println!("remainder: input {inputs_sum} output {outputs_sum}");

    // println!("{selected_inputs:?}\n{outputs:?}");

    if inputs_sum > outputs_sum || native_tokens_diff.is_some() {
        let Some(remainder_address) = get_remainder_address(selected_inputs,remainder_address) else {
            return Err(Error::MissingInputWithEd25519Address);
        };

        let mut remainder_builder = if inputs_sum > outputs_sum {
            // TODO could this also fail if not enough to cover native tokens ?
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
