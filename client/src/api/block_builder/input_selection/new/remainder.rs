// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{requirement::base_token::base_token_sums, OutputInfo};
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
    // let input_native_tokens = gather_nts(selected_inputs);
    // let output_native_tokens = gather_nts(outputs);
    // let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_nts(selected_inputs, outputs)?;
    // let native_tokens_diffs = (input_native_tokens + minted) - (output + melted + burn);

    let (inputs_sum, outputs_sum) = base_token_sums(selected_inputs, outputs);

    // println!("remainder: input {inputs_sum} output {outputs_sum}");

    // println!("{selected_inputs:?}\n{outputs:?}");

    if inputs_sum > outputs_sum {
        let diff = inputs_sum - outputs_sum;

        let Some(remainder_address) = get_remainder_address(selected_inputs,remainder_address) else {
            return Err(Error::MissingInputWithEd25519Address);
        };

        // println!("{diff} {remainder_address:?}");

        let output = BasicOutputBuilder::new_with_amount(diff)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_address)))
            .finish_output(protocol_parameters.token_supply())?;

        // TODO should we always try to select enough inputs so the diff covers the deposit?
        output.verify_storage_deposit(
            protocol_parameters.rent_structure().clone(),
            protocol_parameters.token_supply(),
        )?;

        return Ok(Some(output));
    }

    // if !native_tokens_diffs.is_empty() || base_coin_diff != 0 {
    //     let mut remainder = OutputBuilder::new(base_coin_diff).with_unlock_condition(remainder_address);
    //     if !native_tokens_diffs.is_empty() {
    //         remainder = remainder.with_native_tokens(native_tokens_diffs);
    //     }
    //     Some(remainder.finish())
    // } else {
    //     None
    // }

    Ok(None)
}
