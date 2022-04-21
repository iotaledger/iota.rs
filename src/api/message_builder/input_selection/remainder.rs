// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{hash_map::Entry, HashMap};

use bee_message::{
    address::Address,
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, ByteCostConfig, NativeToken, Output, TokenId,
    },
};
use primitive_types::U256;

use crate::{
    api::input_selection::{
        get_accumulated_output_amounts, get_minted_and_melted_native_tokens, get_remainder_native_tokens,
        minimum_storage_deposit, AccumulatedOutputAmounts,
    },
    signing::types::InputSigningData,
    Error, Result,
};

// Get a remainder with amount and native tokens if necessary, if no remainder_address is provided it will be selected
// from the inputs, also validates the amounts
pub(crate) async fn get_remainder_output(
    inputs: &[Output],
    outputs: &[Output],
    remainder_address: Option<Address>,
    byte_cost_config: &ByteCostConfig,
    allow_burning: bool,
) -> Result<Option<Output>> {
    log::debug!("[get_remainder]");
    let mut remainder_output = None;
    let input_data = get_accumulated_output_amounts(&[], inputs).await?;
    let output_data = get_accumulated_output_amounts(&[], outputs).await?;
    // Get minted native tokens
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(inputs, outputs)?;

    // check amount first
    if input_data.amount < output_data.amount {
        return Err(Error::NotEnoughBalance(input_data.amount, output_data.amount));
    }
    let remainder_amount = input_data.amount - output_data.amount;

    // add minted tokens
    let mut input_native_tokens = input_data.native_tokens;
    for (token_id, minted_native_token_amount) in minted_native_tokens {
        match input_native_tokens.entry(token_id) {
            Entry::Vacant(e) => {
                e.insert(minted_native_token_amount);
            }
            Entry::Occupied(mut e) => {
                *e.get_mut() += minted_native_token_amount;
            }
        }
    }

    // add burned tokens
    let mut output_native_tokens: HashMap<TokenId, U256> = output_data.native_tokens;
    // add burned native tokens as outputs, because we need to have this amount in the inputs
    for (tokend_id, burned_amount) in melted_native_tokens {
        match output_native_tokens.entry(tokend_id) {
            Entry::Vacant(e) => {
                e.insert(burned_amount);
            }
            Entry::Occupied(mut e) => {
                *e.get_mut() += burned_amount;
            }
        }
    }

    let native_token_remainder = get_remainder_native_tokens(&input_native_tokens, &output_native_tokens);
    // Output possible remaining tokens back to the original address
    if remainder_amount > 0 {
        let remainder_addr = get_remainder_address(remainder_address, inputs)?;

        let mut remainder_output_builder = BasicOutputBuilder::new_with_amount(remainder_amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_addr)));
        if let Some(remainder_native_tokens) = native_token_remainder {
            for (token_id, amount) in remainder_native_tokens {
                remainder_output_builder =
                    remainder_output_builder.add_native_token(NativeToken::new(token_id, amount)?);
            }
        }
        let remainder = Output::Basic(remainder_output_builder.finish()?);
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

// Return provided remainder address or get an Ed25519 address from the inputs as remainder address
// We don't want to use nft or alias addresses as remainder address, because we might can't control them later
pub(crate) fn get_remainder_address(remainder_address: Option<Address>, inputs: &[Output]) -> Result<Address> {
    match remainder_address {
        Some(address) => Ok(address),
        // get address from an input, by default we only allow ed25519 addresses as remainder, because then we're
        // sure that the sender can access it
        None => {
            let mut address = None;
            for input in inputs {
                if let Some(unlock_conditions) = input.unlock_conditions() {
                    for unlock_condition in unlock_conditions.iter() {
                        if let UnlockCondition::Address(address_unlock_condition) = unlock_condition {
                            address.replace(address_unlock_condition.address());
                            break;
                        }
                    }
                }
            }
            match address {
                Some(addr) => Ok(*addr),
                None => Err(Error::MissingInputWithEd25519UnlockCondition),
            }
        }
    }
}

// Get additional required storage deposit amount for the remainder output
pub(crate) fn get_additional_required_remainder_amount(
    remainder_address: Option<Address>,
    selected_inputs: &[InputSigningData],
    selected_input_amount: u64,
    selected_input_native_tokens: &HashMap<TokenId, U256>,
    required_accumulated_amounts: &AccumulatedOutputAmounts,
    byte_cost_config: &ByteCostConfig,
) -> crate::Result<u64> {
    let additional_required_remainder_amount = {
        if selected_input_amount > required_accumulated_amounts.amount {
            let current_remainder_amount = selected_input_amount - required_accumulated_amounts.amount;
            let native_token_remainder = get_remainder_native_tokens(
                selected_input_native_tokens,
                &required_accumulated_amounts.native_tokens,
            );

            let required_deposit = minimum_storage_deposit(
                byte_cost_config,
                &get_remainder_address(
                    remainder_address,
                    &selected_inputs
                        .iter()
                        .map(|i| Ok(Output::try_from(&i.output_response.output)?))
                        .collect::<Result<Vec<Output>>>()?,
                )?,
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
