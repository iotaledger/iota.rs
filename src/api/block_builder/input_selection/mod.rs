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
        BasicOutputBuilder, ByteCost, ByteCostConfig, NativeTokens, Output, UnlockCondition, OUTPUT_COUNT_MAX,
    },
};
use packable::{bounded::TryIntoBoundedU16Error, PackableExt};

pub(crate) use self::{automatic::get_inputs, manual::get_custom_inputs};
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
    byte_cost_config: &ByteCostConfig,
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
            byte_cost_config,
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
            byte_cost_config,
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
            byte_cost_config,
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
        byte_cost_config,
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

/// Computes the minimum amount that an output needs to have, when native tokens are sent with [AddressUnlockCondition].
pub fn minimum_storage_deposit(
    config: &ByteCostConfig,
    address: &Address,
    native_tokens: &Option<NativeTokens>,
) -> Result<u64> {
    let address_condition = UnlockCondition::Address(AddressUnlockCondition::new(*address));
    // Safety: This can never fail because the amount will always be within the valid range. Also, the actual value is
    // not important, we are only interested in the storage requirements of the type.
    // todo: use `OutputAmount::MIN` when public, see https://github.com/iotaledger/bee/issues/1238
    let mut basic_output_builder = BasicOutputBuilder::new_with_amount(1_000_000_000)?;
    if let Some(native_tokens) = native_tokens {
        basic_output_builder = basic_output_builder.with_native_tokens(native_tokens.clone());
    }
    let basic_output = basic_output_builder
        .add_unlock_condition(address_condition)
        .finish_output()?;

    Ok(basic_output.byte_cost(config))
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
            if !expired {
                Some(sdr)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
