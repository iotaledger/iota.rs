// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_types::block::{
    address::Address,
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, Output, RentStructure,
    },
};

use crate::{
    api::{
        input_selection::{
            get_accumulated_output_amounts, get_minted_and_melted_native_tokens, get_remainder_native_tokens,
        },
        RemainderData,
    },
    crypto::keys::slip10::Chain,
    secret::types::InputSigningData,
    Error, Result,
};

// Get a remainder with amount and native tokens if necessary, if no remainder_address is provided it will be selected
// from the inputs, also validates the amounts
pub(crate) fn get_remainder_output<'a>(
    inputs: impl Iterator<Item = &'a InputSigningData> + Clone,
    outputs: impl Iterator<Item = &'a Output> + Clone,
    remainder_address: Option<Address>,
    rent_structure: &RentStructure,
    allow_burning: bool,
    current_time: u32,
    token_supply: u64,
) -> Result<Option<RemainderData>> {
    log::debug!("[get_remainder]");
    let input_outputs = inputs.clone().map(|i| &i.output);
    let input_data = get_accumulated_output_amounts(&std::iter::empty(), input_outputs.clone())?;
    let output_data = get_accumulated_output_amounts(&std::iter::empty(), outputs.clone())?;
    // Get minted native tokens
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(&input_outputs, outputs)?;

    // check amount first
    if input_data.amount < output_data.amount {
        return Err(Error::InsufficientAmount {
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
            None => get_remainder_address(inputs, current_time)?,
        };

        let mut remainder_output_builder = BasicOutputBuilder::new_with_amount(remainder_amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_addr)));
        if let Some(remainder_native_tokens) = native_token_remainder {
            remainder_output_builder = remainder_output_builder.with_native_tokens(remainder_native_tokens);
        }
        let remainder = remainder_output_builder.finish_output(token_supply)?;
        // Check if output has enough amount to cover the storage deposit
        remainder.verify_storage_deposit(rent_structure.clone(), token_supply)?;
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
    current_time: u32,
) -> Result<(Address, Option<Chain>)> {
    for input in inputs {
        if let Some(unlock_conditions) = input.output.unlock_conditions() {
            if let Some(address_expired) = unlock_conditions
                .expiration()
                .and_then(|e| e.return_address_expired(current_time))
            {
                if address_expired.is_ed25519() {
                    return Ok((*address_expired, input.chain.clone()));
                }
            }

            if let Some(address_unlock_condition) = unlock_conditions.address() {
                if address_unlock_condition.address().is_ed25519() {
                    return Ok((*address_unlock_condition.address(), input.chain.clone()));
                }
            }

            if let Some(governor_address_unlock_condition) = unlock_conditions.governor_address() {
                if governor_address_unlock_condition.address().is_ed25519() {
                    return Ok((*governor_address_unlock_condition.address(), input.chain.clone()));
                }
            }
        }
    }

    Err(Error::MissingInputWithEd25519Address)
}
