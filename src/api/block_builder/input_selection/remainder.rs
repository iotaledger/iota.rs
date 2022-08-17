// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use bee_block::{
    address::Address,
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition, UnlockConditions},
        BasicOutputBuilder, NativeTokensBuilder, Output, RentStructure,
    },
};

use crate::{
    api::{
        input_selection::{
            get_accumulated_output_amounts, get_minted_and_melted_native_tokens, get_remainder_native_tokens,
            helpers::{minimum_storage_deposit_basic_output, sdr_not_expired},
            AccumulatedOutputAmounts,
        },
        RemainderData,
    },
    crypto::keys::slip10::Chain,
    secret::types::InputSigningData,
    Error, Result,
};

// Get possible required storage deposit return outputs, if there is already an output for the storage deposit return,
// then don't return a new output for that.
pub(crate) fn get_storage_deposit_return_outputs<'a>(
    inputs: impl Iterator<Item = &'a InputSigningData> + Clone,
    outputs: impl Iterator<Item = &'a Output> + Clone,
    current_time: u32,
) -> Result<Vec<Output>> {
    log::debug!("[get_storage_deposit_return_outputs]");

    // Get inputs with storage deposit return unlock condition that are not expired
    let inputs_sdr = inputs.filter_map(|i| sdr_not_expired(&i.output, current_time));

    // There could be multiple sdr outputs required for the same address, so we keep track of the required amount here.
    let mut required_address_returns: HashMap<Address, u64> = HashMap::new();
    for sdr in inputs_sdr {
        *required_address_returns.entry(*sdr.return_address()).or_default() += sdr.amount();
    }

    let mut new_sdr_outputs = Vec::new();

    for (return_address, required_return_amount) in required_address_returns {
        // Check if we already have outputs for this storage deposit return
        let existing_sdr_amount: u64 = outputs
            .clone()
            .filter(|output| {
                // sdr output must be a basic output with only the AddressUnlockCondition
                if let Output::Basic(output) = output {
                    output
                        .simple_deposit_address()
                        .map_or(false, |address| address == &return_address)
                } else {
                    false
                }
            })
            .map(|o| o.amount())
            .sum();

        if required_return_amount > existing_sdr_amount {
            // create a new output with the missing amount
            let remaining_amount_to_add = required_return_amount - existing_sdr_amount;
            new_sdr_outputs.push(
                BasicOutputBuilder::new_with_amount(remaining_amount_to_add)?
                    .with_unlock_conditions([UnlockCondition::Address(AddressUnlockCondition::new(return_address))])
                    .finish_output()?,
            );
        }
    }

    Ok(new_sdr_outputs)
}

// Get a remainder with amount and native tokens if necessary, if no remainder_address is provided it will be selected
// from the inputs, also validates the amounts
pub(crate) fn get_remainder_output<'a>(
    inputs: impl Iterator<Item = &'a InputSigningData> + Clone,
    outputs: impl Iterator<Item = &'a Output> + Clone,
    remainder_address: Option<Address>,
    rent_structure: &RentStructure,
    allow_burning: bool,
) -> Result<Option<RemainderData>> {
    log::debug!("[get_remainder]");
    let input_outputs = inputs.clone().map(|i| &i.output);
    let input_data = get_accumulated_output_amounts(&std::iter::empty(), input_outputs.clone())?;
    let output_data = get_accumulated_output_amounts(&std::iter::empty(), outputs.clone())?;
    // Get minted native tokens
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(&input_outputs, outputs)?;

    // check amount first
    if input_data.amount < output_data.amount {
        return Err(Error::NotEnoughBalance {
            found: input_data.amount,
            required: output_data.amount,
        });
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
    let remainder_data = if remainder_amount > 0 {
        let (remainder_addr, address_chain) = match remainder_address {
            // For provided remainder addresses we can't get the Chain
            Some(a) => (a, None),
            None => get_remainder_address(inputs)?,
        };

        let mut remainder_output_builder = BasicOutputBuilder::new_with_amount(remainder_amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_addr)));
        if let Some(remainder_native_tokens) = native_token_remainder {
            remainder_output_builder = remainder_output_builder.with_native_tokens(remainder_native_tokens);
        }
        let remainder = remainder_output_builder.finish_output()?;
        // Check if output has enough amount to cover the storage deposit
        remainder.verify_storage_deposit(rent_structure)?;
        Some(RemainderData {
            output: remainder,
            chain: address_chain,
            address: remainder_addr,
        })
    } else {
        // if we have remaining native tokens, but no amount left, then we can't create this transaction unless we want
        // to burn them
        if native_token_remainder.is_some() && !allow_burning {
            return Err(Error::NoBalanceForNativeTokenRemainder);
        }
        None
    };

    Ok(remainder_data)
}

// Get an Ed25519 address from the inputs as remainder address
// We don't want to use nft or alias addresses as remainder address, because we might not be able to control them later
pub(crate) fn get_remainder_address<'a>(
    inputs: impl Iterator<Item = &'a InputSigningData>,
) -> Result<(Address, Option<Chain>)> {
    for input in inputs {
        // todo: check expiration with time, for now we just ignore outputs with an expiration unlock condition here
        if input
            .output
            .unlock_conditions()
            .and_then(UnlockConditions::expiration)
            .is_some()
        {
            continue;
        }
        if let Some(address_unlock_condition) = input.output.unlock_conditions().and_then(UnlockConditions::address) {
            if address_unlock_condition.address().is_ed25519() {
                return Ok((*address_unlock_condition.address(), input.chain.clone()));
            }
        }
        if let Some(governor_address_unlock_condition) = input
            .output
            .unlock_conditions()
            .and_then(UnlockConditions::governor_address)
        {
            if governor_address_unlock_condition.address().is_ed25519() {
                return Ok((*governor_address_unlock_condition.address(), input.chain.clone()));
            }
        }
    }

    Err(Error::MissingInputWithEd25519Address)
}

// Get additional required storage deposit amount for the remainder output
pub(crate) fn get_additional_required_remainder_amount(
    remainder_address: Option<Address>,
    selected_inputs: &[InputSigningData],
    selected_input_amount: u64,
    selected_input_native_tokens: &NativeTokensBuilder,
    required_accumulated_amounts: &AccumulatedOutputAmounts,
    rent_structure: &RentStructure,
) -> crate::Result<u64> {
    let additional_required_remainder_amount = {
        if selected_input_amount > required_accumulated_amounts.amount {
            let current_remainder_amount = selected_input_amount - required_accumulated_amounts.amount;
            let native_token_remainder = get_remainder_native_tokens(
                selected_input_native_tokens,
                &required_accumulated_amounts.native_tokens,
            )?;

            let required_deposit = minimum_storage_deposit_basic_output(
                rent_structure,
                &match remainder_address {
                    Some(a) => a,
                    None => get_remainder_address(selected_inputs.iter())?.0,
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
