// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::requirement::base_token::base_token_sums;
use crate::{
    block::{
        address::Address,
        output::{
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            BasicOutputBuilder, Output,
        },
        protocol::ProtocolParameters,
    },
    error::Result,
    secret::types::InputSigningData,
};

pub(crate) fn remainder_output(
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
    remainder_address: Option<Address>,
    protocol_parameters: &ProtocolParameters,
) -> Result<Option<Output>> {
    // let input_native_tokens = gather_nts(selected_inputs);
    // let output_native_tokens = gather_nts(outputs);
    // let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_nts(selected_inputs, outputs)?;
    // let native_tokens_diffs = (input_native_tokens + minted) - (output + melted + burn);

    let (inputs_sum, outputs_sum) = base_token_sums(selected_inputs, outputs);

    if inputs_sum > outputs_sum {
        let diff = inputs_sum - outputs_sum;
        // Gets the remainder address from configuration of finds one from the inputs.
        let remainder_address = remainder_address.or_else(|| {
            selected_inputs.iter().find_map(|input| {
                if let Some(address) = input
                    .output
                    .unlock_conditions()
                    .and_then(|unlock_conditions| unlock_conditions.address())
                {
                    if address.address().is_ed25519() {
                        Some(*address.address())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        });

        let Some(remainder_address) = remainder_address else {
                // TODO return actual error
                panic!("");
            };

        println!("{diff} {remainder_address:?}");

        let output = BasicOutputBuilder::new_with_amount(diff)?
            .add_unlock_condition(UnlockCondition::from(AddressUnlockCondition::new(remainder_address)))
            .finish_output(protocol_parameters.token_supply())?;

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
