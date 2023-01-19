// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

mod automatic;
mod helpers;
mod manual;
mod native_token_helpers;
/// TODO No need to document, will be removed in the future.
pub mod new;
mod remainder;
mod sender_issuer;
pub mod types;
mod utxo_chains;

pub use helpers::minimum_storage_deposit_basic_output;
use iota_types::block::{
    address::Address,
    output::{Output, RentStructure, OUTPUT_COUNT_MAX},
};
use packable::bounded::TryIntoBoundedU16Error;

use self::{
    helpers::get_accumulated_output_amounts,
    native_token_helpers::{get_minted_and_melted_native_tokens, get_remainder_native_tokens},
    remainder::get_remainder_output,
    types::SelectedTransactionData,
};
use crate::{
    api::input_selection::{helpers::sort_input_signing_data, remainder::get_storage_deposit_return_outputs},
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
    dedup_inputs(&mut mandatory_inputs, &mut additional_inputs);

    // Always have the mandatory inputs already selected.
    let selected_inputs: Vec<InputSigningData> = mandatory_inputs.clone();
    let all_inputs = mandatory_inputs.iter().chain(additional_inputs.iter());
    let input_outputs = all_inputs.clone().map(|i| &i.output);

    let required = get_accumulated_output_amounts(&input_outputs, outputs.iter())?;
    // Add the minted tokens to the inputs, because we don't need to provide other inputs for them
    let mut selected_input_native_tokens = required.minted_native_tokens.clone();

    // Add the mandatory inputs native tokens.
    for input in selected_inputs.iter() {
        if let Some(native_tokens) = input.output.native_tokens() {
            selected_input_native_tokens.add_native_tokens(native_tokens.clone())?;
        }
    }

    // Basic outputs.
    let mut basic_outputs = Vec::<InputSigningData>::new();

    // Order input outputs descending, so that as few inputs as necessary are used
    basic_outputs.sort_by(|l, r| l.output.amount().cmp(&r.output.amount()));

    // Add possible required storage deposit return outputs
    let additional_storage_deposit_return_outputs =
        get_storage_deposit_return_outputs(all_inputs, outputs.iter(), current_time, token_supply)?;
    outputs.extend(additional_storage_deposit_return_outputs.into_iter());

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
