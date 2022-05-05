// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{
    address::Address,
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, ByteCostConfig, NativeTokensBuilder, Output,
    },
};

use crate::{
    api::input_selection::{
        get_accumulated_output_amounts, get_minted_and_melted_native_tokens, get_remainder_native_tokens,
        minimum_storage_deposit, AccumulatedOutputAmounts,
    },
    secret::types::InputSigningData,
    Error, Result,
};

// Get a remainder with amount and native tokens if necessary, if no remainder_address is provided it will be selected
// from the inputs, also validates the amounts
pub(crate) async fn get_remainder_output<'a>(
    inputs: impl Iterator<Item = &'a Output> + Clone,
    outputs: impl Iterator<Item = &'a Output> + Clone,
    remainder_address: Option<Address>,
    byte_cost_config: &ByteCostConfig,
    allow_burning: bool,
) -> Result<Option<Output>> {
    log::debug!("[get_remainder]");
    let mut remainder_output = None;
    let input_data = get_accumulated_output_amounts(std::iter::empty(), inputs.clone()).await?;
    let output_data = get_accumulated_output_amounts(std::iter::empty(), outputs.clone()).await?;
    // Get minted native tokens
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(inputs.clone(), outputs)?;

    // check amount first
    if input_data.amount < output_data.amount {
        return Err(Error::NotEnoughBalance(input_data.amount, output_data.amount));
    }
    let remainder_amount = input_data.amount - output_data.amount;

    // add minted tokens
    let mut input_native_tokens = input_data.native_tokens;
    input_native_tokens.merge(minted_native_tokens)?;

    // add melted tokens
    let mut output_native_tokens = output_data.native_tokens;
    // add melted native tokens as outputs, because we need to have this amount in the inputs
    output_native_tokens.merge(melted_native_tokens)?;

    let native_token_remainder = get_remainder_native_tokens(&input_native_tokens, &output_native_tokens)?;
    // Output possible remaining tokens back to the original address
    if remainder_amount > 0 {
        let remainder_addr = match remainder_address {
            Some(a) => a,
            None => get_remainder_address(inputs)?,
        };

        let mut remainder_output_builder = BasicOutputBuilder::new_with_amount(remainder_amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_addr)));
        if let Some(remainder_native_tokens) = native_token_remainder {
            remainder_output_builder = remainder_output_builder.with_native_tokens(remainder_native_tokens);
        }
        let remainder = remainder_output_builder.finish_output()?;
        // Check if output has enough amount to cover the storage deposit
        remainder.verify_storage_deposit(byte_cost_config)?;
        remainder_output.replace(remainder);
    } else {
        // if we have remaining native tokens, but no amount left, then we can't create this transaction unless we want
        // to burn them
        if native_token_remainder.is_some() && !allow_burning {
            return Err(Error::NoBalanceForNativeTokenRemainder);
        }
    }

    Ok(remainder_output)
}

// Get an Ed25519 address from the inputs as remainder address
// We don't want to use nft or alias addresses as remainder address, because we might can't control them later
pub(crate) fn get_remainder_address<'a>(inputs: impl Iterator<Item = &'a Output>) -> Result<Address> {
    // get address from an input, by default we only allow ed25519 addresses as remainder, because then we're sure that
    // the sender can access it
    let mut address = None;
    'outer: for input in inputs {
        if let Some(unlock_conditions) = input.unlock_conditions() {
            for unlock_condition in unlock_conditions.iter() {
                if let UnlockCondition::Address(address_unlock_condition) = unlock_condition {
                    address.replace(address_unlock_condition.address());
                    break 'outer;
                }
            }
        }
    }
    match address {
        Some(addr) => Ok(*addr),
        None => Err(Error::MissingInputWithEd25519UnlockCondition),
    }
}

// Get additional required storage deposit amount for the remainder output
pub(crate) fn get_additional_required_remainder_amount(
    remainder_address: Option<Address>,
    selected_inputs: &[InputSigningData],
    selected_input_amount: u64,
    selected_input_native_tokens: &NativeTokensBuilder,
    required_accumulated_amounts: &AccumulatedOutputAmounts,
    byte_cost_config: &ByteCostConfig,
) -> crate::Result<u64> {
    let additional_required_remainder_amount = {
        if selected_input_amount > required_accumulated_amounts.amount {
            let current_remainder_amount = selected_input_amount - required_accumulated_amounts.amount;
            let native_token_remainder = get_remainder_native_tokens(
                selected_input_native_tokens,
                &required_accumulated_amounts.native_tokens,
            )?;

            let required_deposit = minimum_storage_deposit(
                byte_cost_config,
                &match remainder_address {
                    Some(a) => a,
                    None => get_remainder_address(selected_inputs.iter().map(|i| &i.output))?,
                },
                &native_token_remainder,
            )?;
            if required_deposit > current_remainder_amount {
                required_deposit - current_remainder_amount
            } else {
                0
            }
        } else {
            0
        }
    };
    Ok(additional_required_remainder_amount)
}
