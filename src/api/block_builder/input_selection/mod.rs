// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

mod automatic;
mod manual;
mod native_token_helpers;
mod output_data;
mod remainder;
pub mod types;

use bee_block::{
    address::Address,
    input::INPUT_COUNT_MAX,
    output::{
        unlock_condition::{AddressUnlockCondition, StorageDepositReturnUnlockCondition},
        AliasOutputBuilder, BasicOutputBuilder, FoundryOutputBuilder, NativeTokens, NftOutputBuilder, Output,
        OutputAmount, Rent, RentStructure, UnlockCondition, OUTPUT_COUNT_MAX,
    },
};
use packable::{bounded::TryIntoBoundedU16Error, PackableExt};

use self::{
    native_token_helpers::{get_minted_and_melted_native_tokens, get_remainder_native_tokens, missing_native_tokens},
    output_data::get_accumulated_output_amounts,
    remainder::{get_additional_required_remainder_amount, get_remainder_output},
    types::SelectedTransactionData,
};
use crate::{
    api::input_selection::{remainder::get_storage_deposit_return_outputs, types::AccumulatedOutputAmounts},
    secret::types::InputSigningData,
    Error, Result,
};

/// Select inputs from provided inputs([InputSigningData]) for provided [Output]s, validate amounts and create remainder
/// output if necessary. Also checks for alias, foundry and nft outputs that there previous output exist in the inputs,
/// when required. Careful with setting `allow_burning` to `true`, native tokens can get easily burned by accident.
/// Provided alias, foundry and nft outputs will be used first as inputs and therefore destroyed if not existent in the
/// output
pub fn try_select_inputs(
    mut inputs: Vec<InputSigningData>,
    mut outputs: Vec<Output>,
    force_use_all_inputs: bool,
    remainder_address: Option<Address>,
    rent_structure: &RentStructure,
    allow_burning: bool,
    current_time: u32,
) -> Result<SelectedTransactionData> {
    inputs.sort_by_key(|a| (a.output_metadata.transaction_id, a.output_metadata.output_index));
    inputs.dedup_by_key(|a| (a.output_metadata.transaction_id, a.output_metadata.output_index));

    // Validate and only create a remainder if necessary
    if force_use_all_inputs {
        if inputs.len() as u16 > INPUT_COUNT_MAX {
            return Err(Error::ConsolidationRequired(inputs.len()));
        }

        let additional_storage_deposit_return_outputs =
            get_storage_deposit_return_outputs(inputs.iter(), outputs.iter(), current_time)?;
        outputs.extend(additional_storage_deposit_return_outputs.into_iter());

        let remainder_data = get_remainder_output(
            inputs.iter(),
            outputs.iter(),
            remainder_address,
            rent_structure,
            allow_burning,
        )?;

        if let Some(remainder_data) = &remainder_data {
            outputs.push(remainder_data.output.clone());
        }

        // check if we have too many outputs after adding possible remainder or storage deposit return outputs
        if outputs.len() as u16 > OUTPUT_COUNT_MAX {
            return Err(Error::BlockError(bee_block::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(outputs.len()),
            )));
        }

        return Ok(SelectedTransactionData {
            inputs,
            outputs,
            remainder: remainder_data,
        });
    }
    // else: only select inputs that are necessary for the provided outputs

    let input_outputs = inputs.iter().map(|i| &i.output);

    let mut required = get_accumulated_output_amounts(&input_outputs, outputs.iter())?;
    let mut selected_input_native_tokens = required.minted_native_tokens.clone();

    let mut selected_input_amount = 0;
    let mut selected_inputs: Vec<InputSigningData> = Vec::new();

    // Split outputs by type
    let mut basic_outputs = Vec::new();
    // alias, foundry, nft outputs
    let mut utxo_chain_outputs = Vec::new();
    for input_signing_data in &inputs {
        match input_signing_data.output {
            Output::Basic(_) => basic_outputs.push(input_signing_data),
            Output::Alias(_) | Output::Foundry(_) | Output::Nft(_) => utxo_chain_outputs.push(input_signing_data),
            Output::Treasury(_) => {}
        }
    }

    // 1. get alias, foundry or nft inputs (because amount and native tokens of these outputs will also be available for
    // the outputs)
    for input_signing_data in utxo_chain_outputs {
        // Only add outputs if also exisiting on the output side or if burning is allowed
        let minimum_required_storage_deposit = input_signing_data.output.rent_cost(rent_structure);
        let output_id = input_signing_data.output_id()?;

        match &input_signing_data.output {
            Output::Nft(nft_input) => {
                // or if an output contains an nft output with the same nft id
                let is_required = outputs.iter().any(|output| {
                    if let Output::Nft(nft_output) = output {
                        nft_input.nft_id().or_from_output_id(output_id) == *nft_output.nft_id()
                    } else {
                        false
                    }
                });
                if !is_required && !allow_burning {
                    // Don't add if it doesn't give us any amount or native tokens
                    if input_signing_data.output.amount() == minimum_required_storage_deposit
                        && nft_input.native_tokens().is_empty()
                    {
                        continue;
                    }
                    // else add output to outputs with minimum_required_storage_deposit
                    let new_output = NftOutputBuilder::from(nft_input)
                        .with_nft_id(nft_input.nft_id().or_from_output_id(output_id))
                        .with_amount(minimum_required_storage_deposit)?
                        .finish_output()?;
                    outputs.push(new_output);
                }
            }
            Output::Alias(alias_input) => {
                // Don't add if output has not the same AliasId, so we don't burn it
                if !outputs.iter().any(|output| {
                    if let Output::Alias(alias_output) = output {
                        alias_input.alias_id().or_from_output_id(output_id) == *alias_output.alias_id()
                    } else {
                        false
                    }
                }) && !allow_burning
                {
                    // Don't add if it doesn't give us any amount or native tokens
                    if input_signing_data.output.amount() == minimum_required_storage_deposit
                        && alias_input.native_tokens().is_empty()
                    {
                        continue;
                    }
                    // else add output to outputs with minimum_required_storage_deposit
                    let new_output = AliasOutputBuilder::from(alias_input)
                        .with_alias_id(alias_input.alias_id().or_from_output_id(output_id))
                        .with_state_index(alias_input.state_index() + 1)
                        .with_amount(minimum_required_storage_deposit)?
                        .finish_output()?;
                    outputs.push(new_output);
                }
            }
            Output::Foundry(foundry_input) => {
                // Don't add if output has not the same FoundryId, so we don't burn it
                if !outputs.iter().any(|output| {
                    if let Output::Foundry(foundry_output) = output {
                        foundry_input.id() == foundry_output.id()
                    } else {
                        false
                    }
                }) && !allow_burning
                {
                    // Don't add if it doesn't give us any amount or native tokens
                    if input_signing_data.output.amount() == minimum_required_storage_deposit
                        && foundry_input.native_tokens().is_empty()
                    {
                        continue;
                    }
                    // else add output to outputs with minimum_required_storage_deposit
                    let new_output = FoundryOutputBuilder::from(foundry_input)
                        .with_amount(minimum_required_storage_deposit)?
                        .finish_output()?;
                    outputs.push(new_output);
                }
            }
            _ => {}
        }

        let output = &input_signing_data.output;
        selected_input_amount += output.amount();

        if let Some(output_native_tokens) = output.native_tokens() {
            selected_input_native_tokens.add_native_tokens(output_native_tokens.clone())?;
        }

        if let Some(sdr) = sdr_not_expired(output, current_time) {
            // add sdr to required amount, because we have to send it back
            required.amount += sdr.amount();
        }

        selected_inputs.push(input_signing_data.clone());

        // Updated required value with possible new input
        let input_outputs = inputs.iter().map(|i| &i.output);
        required = get_accumulated_output_amounts(&input_outputs.clone(), outputs.iter())?;
    }

    // Validate that we have the required inputs for alias and nft outputs
    for output in &outputs {
        match output {
            Output::Alias(alias_output) => {
                // New created output requires no specific input
                let alias_id = alias_output.alias_id();
                if alias_id.is_null() {
                    continue;
                }

                if !selected_inputs.iter().any(|data| {
                    if let Output::Alias(input_alias_output) = &data.output {
                        input_alias_output
                            .alias_id()
                            .or_from_output_id(data.output_id().expect("Invalid output id"))
                            == *alias_id
                    } else {
                        false
                    }
                }) {
                    return Err(crate::Error::MissingInput(format!(
                        "Missing alias input for {alias_id}"
                    )));
                }
            }
            Output::Foundry(foundry_output) => {
                let required_alias = foundry_output.alias_address().alias_id();
                if !selected_inputs.iter().any(|data| {
                    if let Output::Alias(input_alias_output) = &data.output {
                        input_alias_output
                            .alias_id()
                            .or_from_output_id(data.output_id().expect("Invalid output id"))
                            == *required_alias
                    } else {
                        false
                    }
                }) {
                    return Err(crate::Error::MissingInput(format!(
                        "Missing alias input {required_alias} for foundry {}",
                        foundry_output.id()
                    )));
                }
            }
            Output::Nft(nft_output) => {
                // New created output requires no specific input
                let nft_id = nft_output.nft_id();
                if nft_id.is_null() {
                    continue;
                }

                if !selected_inputs.iter().any(|data| {
                    if let Output::Nft(input_nft_output) = &data.output {
                        input_nft_output
                            .nft_id()
                            .or_from_output_id(data.output_id().expect("Invalid output id"))
                            == *nft_id
                    } else {
                        false
                    }
                }) {
                    return Err(crate::Error::MissingInput(format!("Missing nft input for {nft_id}")));
                }
            }
            _ => {}
        }
    }

    // 2. get basic inputs for the required native tokens (because the amount of these outputs will also be available in
    // the outputs)
    if !required.native_tokens.is_empty() {
        let mut index = 0;
        while index < basic_outputs.len() {
            let mut added_to_inputs = false;
            let output = &basic_outputs[index].output;

            if let Some(output_native_tokens) = output.native_tokens() {
                // Only add output to the inputs if it has a native token we need for the outputs
                if output_native_tokens
                    .iter()
                    .any(|native_token| required.native_tokens.get(native_token.token_id()).is_some())
                {
                    // If there is a native token we need for the outputs we'll also add all others, because we'll
                    // consume this output
                    selected_input_native_tokens.add_native_tokens(output_native_tokens.clone())?;
                    selected_input_amount += output.amount();
                    selected_inputs.push(basic_outputs[index].clone());
                    added_to_inputs = true;
                    if let Some(sdr) = sdr_not_expired(output, current_time) {
                        // add sdr to required amount, because we have to send it back
                        required.amount += sdr.amount();
                    }
                }
            }

            // If added to the inputs, remove it so it can't be selected again
            if added_to_inputs {
                basic_outputs.remove(index);
                // Continue without increasing the index because we removed one element
                continue;
            }
            // Increase index so we check the next index
            index += 1;
        }
    }

    // check if we got all required native tokens
    if let Some(native_token) = missing_native_tokens(&selected_input_native_tokens, &required.native_tokens)? {
        return Err(Error::NotEnoughNativeTokens(native_token));
    }

    // 3. try to select basic outputs without native tokens
    let mut index = 0;
    while index < basic_outputs.len() {
        let mut added_to_inputs = false;

        let additional_required_remainder_amount = get_additional_required_remainder_amount(
            remainder_address,
            &selected_inputs,
            selected_input_amount,
            &selected_input_native_tokens,
            &required,
            rent_structure,
        )?;

        if selected_input_amount < required.amount || additional_required_remainder_amount > 0 {
            let output = &basic_outputs[index].output;

            if let Some(output_native_tokens) = output.native_tokens() {
                if output_native_tokens.is_empty() {
                    selected_input_amount += output.amount();
                    selected_inputs.push(basic_outputs[index].clone());
                    added_to_inputs = true;
                    if let Some(sdr) = sdr_not_expired(output, current_time) {
                        // add sdr to required amount, because we have to send it back
                        required.amount += sdr.amount();
                    }
                }
            }
        }

        // If added to the inputs, remove it so it can't be selected again
        if added_to_inputs {
            basic_outputs.remove(index);
            // Continue without increasing the index because we removed one element
            continue;
        }
        // Increase index so we check the next index
        index += 1;
    }

    // check if we have too many inputs
    let current_selected_input_len = selected_inputs.len() as u16;
    if current_selected_input_len > INPUT_COUNT_MAX {
        return Err(Error::ConsolidationRequired(current_selected_input_len.into()));
    }

    // Order input outputs descending, so that as few inputs as necessary are used
    basic_outputs.sort_by(|l, r| l.output.amount().cmp(&r.output.amount()));

    // 4. try to select basic outputs with native tokens we need for the outputs
    let mut index = 0;
    while index < basic_outputs.len() {
        let mut added_to_inputs = false;

        let additional_required_remainder_amount = get_additional_required_remainder_amount(
            remainder_address,
            &selected_inputs,
            selected_input_amount,
            &selected_input_native_tokens,
            &required,
            rent_structure,
        )?;

        if selected_input_amount < required.amount || additional_required_remainder_amount > 0 {
            let output = &basic_outputs[index].output;

            selected_input_amount += output.amount();
            if let Some(output_native_tokens) = output.native_tokens() {
                selected_input_native_tokens.add_native_tokens(output_native_tokens.clone())?;
            }
            selected_inputs.push(basic_outputs[index].clone());
            added_to_inputs = true;
            if let Some(sdr) = sdr_not_expired(output, current_time) {
                // add sdr to required amount, because we have to send it back
                required.amount += sdr.amount();
            }
        }

        // If added to the inputs, remove it so it can't be selected again
        if added_to_inputs {
            basic_outputs.remove(index);
            // Continue without increasing the index because we removed one element
            continue;
        }
        // Increase index so we check the next index
        index += 1;
    }

    // check if we have too many inputs
    let current_selected_input_len = selected_inputs.len() as u16;
    if current_selected_input_len > INPUT_COUNT_MAX {
        return Err(Error::ConsolidationRequired(current_selected_input_len.into()));
    }

    // Add possible required storage deposit return outputs
    let additional_storage_deposit_return_outputs =
        get_storage_deposit_return_outputs(inputs.iter(), outputs.iter(), current_time)?;
    outputs.extend(additional_storage_deposit_return_outputs.into_iter());

    // create remainder output if necessary
    // get_remainder also checks for amounts and returns an error if we don't have enough
    let remainder_data = get_remainder_output(
        selected_inputs.iter(),
        outputs.iter(),
        remainder_address,
        rent_structure,
        allow_burning,
    )?;
    if let Some(remainder_data) = &remainder_data {
        outputs.push(remainder_data.output.clone());

        // check if we have too many outputs after adding the remainder output
        if outputs.len() as u16 > OUTPUT_COUNT_MAX {
            return Err(Error::BlockError(bee_block::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(outputs.len()),
            )));
        }
    }

    // sort inputs so ed25519 address unlocks will be first, safe to unwrap since we encoded it before
    selected_inputs.sort_unstable_by_key(|a| Address::try_from_bech32(&a.bech32_address).unwrap().1.pack_to_vec());
    Ok(SelectedTransactionData {
        inputs: selected_inputs,
        outputs,
        remainder: remainder_data,
    })
}

/// Computes the minimum storage deposit amount that a basic output needs to have with an [AddressUnlockCondition] and
/// optional [NativeTokens].
pub fn minimum_storage_deposit_basic_output(
    config: &RentStructure,
    address: &Address,
    native_tokens: &Option<NativeTokens>,
) -> Result<u64> {
    let address_condition = UnlockCondition::Address(AddressUnlockCondition::new(*address));
    let mut basic_output_builder = BasicOutputBuilder::new_with_amount(OutputAmount::MIN)?;
    if let Some(native_tokens) = native_tokens {
        basic_output_builder = basic_output_builder.with_native_tokens(native_tokens.clone());
    }
    let basic_output = basic_output_builder
        .add_unlock_condition(address_condition)
        .finish_output()?;

    Ok(basic_output.rent_cost(config))
}

/// Get the `StorageDepositReturnUnlockCondition`, if not expired
pub(crate) fn sdr_not_expired(output: &Output, current_time: u32) -> Option<&StorageDepositReturnUnlockCondition> {
    if let Some(unlock_conditions) = output.unlock_conditions() {
        if let Some(sdr) = unlock_conditions.storage_deposit_return() {
            let expired = if let Some(expiration) = unlock_conditions.expiration() {
                current_time >= expiration.timestamp()
            } else {
                false
            };

            // We only have to send the storage deposit return back if the output is not expired
            if !expired { Some(sdr) } else { None }
        } else {
            None
        }
    } else {
        None
    }
}
