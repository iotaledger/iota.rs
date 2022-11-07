// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

mod automatic;
mod helpers;
mod manual;
mod native_token_helpers;
mod remainder;
mod sender_issuer;
pub mod types;
mod utxo_chains;
use std::collections::HashSet;

pub use helpers::minimum_storage_deposit_basic_output;
use iota_types::block::{
    address::Address,
    input::INPUT_COUNT_MAX,
    output::{Output, OutputId, RentStructure, OUTPUT_COUNT_MAX},
};
use packable::bounded::TryIntoBoundedU16Error;

use self::{
    helpers::get_accumulated_output_amounts,
    native_token_helpers::{get_minted_and_melted_native_tokens, get_remainder_native_tokens, missing_native_tokens},
    remainder::{get_additional_required_remainder_amount, get_remainder_output},
    sender_issuer::select_inputs_for_sender_and_issuer,
    types::SelectedTransactionData,
};
use crate::{
    api::input_selection::{
        helpers::{sdr_not_expired, sort_input_signing_data},
        remainder::get_storage_deposit_return_outputs,
        types::AccumulatedOutputAmounts,
        utxo_chains::{check_utxo_chain_inputs, select_utxo_chain_inputs},
    },
    secret::types::InputSigningData,
    Error, Result,
};

/// Select inputs from provided mandatory_inputs([InputSigningData]) and additional_inputs([InputSigningData]) for
/// provided [Output]s, validate amounts and create remainder output if necessary. Also checks for alias, foundry and
/// nft outputs that there previous output exist in the inputs, when required. Careful with setting `allow_burning` to
/// `true`, native tokens, nfts or alias outputs can get easily burned by accident. Without burning, alias, foundry and
/// nft outputs will be created on the output side, if not already present.
#[allow(clippy::too_many_arguments)]
pub fn try_select_inputs(
    mut mandatory_inputs: Vec<InputSigningData>,
    mut additional_inputs: Vec<InputSigningData>,
    mut outputs: Vec<Output>,
    remainder_address: Option<Address>,
    rent_structure: &RentStructure,
    allow_burning: bool,
    current_time: u32,
    token_supply: u64,
) -> Result<SelectedTransactionData> {
    log::debug!("[try_select_inputs]");

    // Can't select inputs if there are no inputs.
    if mandatory_inputs.is_empty() && additional_inputs.is_empty() {
        return Err(crate::Error::NoInputs);
    }

    dedup_inputs(&mut mandatory_inputs, &mut additional_inputs);

    // Always have the mandatory inputs already selected.
    let mut selected_inputs: Vec<InputSigningData> = mandatory_inputs.clone();
    // Keep track of which inputs we selected in a HashSet, so we don't need to iterate over the inputs every time.
    let mut selected_inputs_output_ids: HashSet<OutputId> =
        selected_inputs.iter().map(|input| *input.output_id()).collect();
    let all_inputs = mandatory_inputs.iter().chain(additional_inputs.iter());
    let input_outputs = all_inputs.clone().map(|i| &i.output);

    // select outputs for sender/issuer features. Alias and nft outputs added to the inputs will be added to the outputs
    // in select_utxo_chain_inputs().
    select_inputs_for_sender_and_issuer(
        all_inputs.clone(),
        &mut selected_inputs,
        &mut selected_inputs_output_ids,
        &mut outputs,
        current_time,
    )?;

    let mut required = get_accumulated_output_amounts(&input_outputs, outputs.iter())?;
    // Add the minted tokens to the inputs, because we don't need to provide other inputs for them
    let mut selected_input_native_tokens = required.minted_native_tokens.clone();

    // Add the mandatory inputs amounts.
    let mut selected_input_amount = selected_inputs.iter().map(|i| i.output.amount()).sum();

    // Add the mandatory inputs native tokens.
    for input in selected_inputs.iter() {
        if let Some(native_tokens) = input.output.native_tokens() {
            selected_input_native_tokens.add_native_tokens(native_tokens.clone())?;
        }
    }

    // Basic outputs.
    let mut basic_outputs = Vec::new();
    // Alias, Foundry and NFT outputs.
    let mut utxo_chain_inputs = Vec::new();

    for input_signing_data in all_inputs.clone() {
        match input_signing_data.output {
            Output::Basic(_) => basic_outputs.push(input_signing_data),
            Output::Alias(_) | Output::Foundry(_) | Output::Nft(_) => {
                utxo_chain_inputs.push(input_signing_data.clone())
            }
            Output::Treasury(_) => {}
        }
    }

    // 1. Get Alias, Foundry or NFT inputs (because amount and native tokens of these outputs will also be available for
    // the outputs).
    // Check the inputs in a loop, because if we add an an output which requires another Alias or NFT output to unlock
    // it, then we might have to add this also.
    // Inputs for which no outputs exists already, will be added automatically to the outputs, if burning isn't allowed.
    select_utxo_chain_inputs(
        &mut selected_inputs,
        &mut selected_inputs_output_ids,
        &mut selected_input_amount,
        &mut selected_input_native_tokens,
        &mut outputs,
        &mut required,
        &mut utxo_chain_inputs,
        allow_burning,
        current_time,
        rent_structure,
        token_supply,
    )?;

    // No need to check for sender and issuer again, since these outputs already exist and we don't set new features
    // for them.

    // Validate that we have selected the required inputs for alias, nft and foundry outputs.
    check_utxo_chain_inputs(&selected_inputs, &outputs)?;

    // Remove inputs we added in `select_inputs_for_sender_and_issuer()`
    let mut index = 0;
    while index < basic_outputs.len() {
        // Remove already added inputs
        if selected_inputs_output_ids.contains(basic_outputs[index].output_id()) {
            basic_outputs.remove(index);
            // Continue without increasing the index because we removed one element
            continue;
        }
        // Increase index so we check the next index
        index += 1;
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
            current_time,
            token_supply,
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
            current_time,
            token_supply,
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
        get_storage_deposit_return_outputs(all_inputs, outputs.iter(), current_time, token_supply)?;
    outputs.extend(additional_storage_deposit_return_outputs.into_iter());

    // Check utxo chain inputs again, because new inputs could have an alias or nft address in their unlock condition
    select_utxo_chain_inputs(
        &mut selected_inputs,
        &mut selected_inputs_output_ids,
        &mut selected_input_amount,
        &mut selected_input_native_tokens,
        &mut outputs,
        &mut required,
        &mut utxo_chain_inputs,
        allow_burning,
        current_time,
        rent_structure,
        token_supply,
    )?;

    // create remainder output if necessary
    // get_remainder also checks for amounts and returns an error if we don't have enough
    let remainder_data = get_remainder_output(
        selected_inputs.iter(),
        outputs.iter(),
        remainder_address,
        rent_structure,
        allow_burning,
        current_time,
        token_supply,
    )?;
    if let Some(remainder_data) = &remainder_data {
        outputs.push(remainder_data.output.clone());

        // check if we have too many outputs after adding the remainder output
        if outputs.len() as u16 > OUTPUT_COUNT_MAX {
            return Err(Error::BlockError(iota_types::block::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(outputs.len()),
            )));
        }
    }

    let sorted_inputs = sort_input_signing_data(selected_inputs)?;

    Ok(SelectedTransactionData {
        inputs: sorted_inputs,
        outputs,
        remainder: remainder_data,
    })
}

// Dedup inputs by output id, because other data could be different, even if it's the same output
fn dedup_inputs(mandatory_inputs: &mut Vec<InputSigningData>, additional_inputs: &mut Vec<InputSigningData>) {
    // Sorting inputs by OutputId so duplicates can be safely removed.
    mandatory_inputs.sort_by_key(|input| *input.output_metadata.output_id());
    mandatory_inputs.dedup_by_key(|input| *input.output_metadata.output_id());
    additional_inputs.sort_by_key(|input| *input.output_metadata.output_id());
    additional_inputs.dedup_by_key(|input| *input.output_metadata.output_id());

    // Remove additional inputs that are already mandatory.
    // TODO: could be done more efficiently with itertools unique?
    additional_inputs.retain(|input| {
        !mandatory_inputs
            .iter()
            .any(|mandatory_input| input.output_metadata.output_id() == mandatory_input.output_metadata.output_id())
    });
}
