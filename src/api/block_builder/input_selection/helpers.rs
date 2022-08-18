// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Helper functions used in the input selection

use bee_block::{
    address::{Address, AliasAddress, Ed25519Address, NftAddress},
    output::{
        unlock_condition::{AddressUnlockCondition, StorageDepositReturnUnlockCondition},
        BasicOutputBuilder, NativeTokens, NativeTokensBuilder, Output, OutputAmount, OutputId, Rent, RentStructure,
        UnlockCondition,
    },
};

use crate::{
    api::input_selection::{get_minted_and_melted_native_tokens, types::AccumulatedOutputAmounts},
    secret::types::InputSigningData,
    Result,
};

// Calculate required accumulated amounts from the outputs, considers also minted and melted native tokens
pub(crate) fn get_accumulated_output_amounts<'a>(
    inputs: &(impl Iterator<Item = &'a Output> + Clone),
    outputs: impl Iterator<Item = &'a Output> + Clone,
) -> Result<AccumulatedOutputAmounts> {
    // Calculate the total tokens to spend
    let mut required_amount: u64 = 0;
    let mut required_native_tokens = NativeTokensBuilder::new();

    for output in outputs.clone() {
        required_amount += output.amount();

        if let Some(output_native_tokens) = output.native_tokens() {
            required_native_tokens.add_native_tokens(output_native_tokens.clone())?;
        }
    }

    // check if a foundry mints or melts native tokens
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(inputs, outputs)?;
    // add melted native tokens as outputs, because we need to have this amount in the inputs
    required_native_tokens.merge(melted_native_tokens)?;

    Ok(AccumulatedOutputAmounts {
        minted_native_tokens,
        amount: required_amount,
        native_tokens: required_native_tokens,
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

// Inputs need to be sorted before signing, because the reference unlock conditions can only reference a lower index
pub(crate) fn sort_input_signing_data(inputs: Vec<InputSigningData>) -> crate::Result<Vec<InputSigningData>> {
    // filter for ed25519 address first, safe to unwrap since we encoded it before
    let (mut sorted_inputs, alias_nft_address_inputs): (Vec<InputSigningData>, Vec<InputSigningData>) = inputs
        .into_iter()
        // PANIC: safe to unwrap as we encoded the address before
        .partition(|input| Address::try_from_bech32(&input.bech32_address).unwrap().1.kind() == Ed25519Address::KIND);

    for input in alias_nft_address_inputs {
        let input_address = Address::try_from_bech32(&input.bech32_address);
        match sorted_inputs.iter().position(|input_signing_data| match input_address {
            Ok((_, unlock_address)) => match unlock_address {
                Address::Alias(unlock_address) => {
                    if let Output::Alias(alias_output) = &input_signing_data.output {
                        *unlock_address.alias_id()
                            == alias_output
                                .alias_id()
                                .or_from_output_id(input_signing_data.output_id().expect("Invalid output id"))
                    } else {
                        false
                    }
                }
                Address::Nft(unlock_address) => {
                    if let Output::Nft(nft_output) = &input_signing_data.output {
                        *unlock_address.nft_id()
                            == nft_output
                                .nft_id()
                                .or_from_output_id(input_signing_data.output_id().expect("Invalid output id"))
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        }) {
            Some(position) => {
                // Insert after the output we need
                sorted_inputs.insert(position + 1, input);
            }
            None => {
                // insert before address
                let alias_or_nft_address = match &input.output {
                    Output::Alias(alias_output) => Some(Address::Alias(AliasAddress::new(
                        alias_output
                            .alias_id()
                            .or_from_output_id(input.output_id().expect("Invalid output id")),
                    ))),
                    Output::Nft(nft_output) => Some(Address::Nft(NftAddress::new(
                        nft_output
                            .nft_id()
                            .or_from_output_id(input.output_id().expect("Invalid output id")),
                    ))),
                    _ => None,
                };

                if let Some(alias_or_nft_address) = alias_or_nft_address {
                    // Check for existing outputs for this address, and insert before
                    match sorted_inputs.iter().position(|input_signing_data| {
                        Address::try_from_bech32(&input_signing_data.bech32_address)
                            .expect("Safe to unwrap, we encoded it before")
                            .1
                            == alias_or_nft_address
                    }) {
                        Some(position) => {
                            // Insert before the output with this address required for unlocking
                            sorted_inputs.insert(position, input);
                        }
                        // just push output
                        None => sorted_inputs.push(input),
                    }
                } else {
                    // just push basic or foundry output
                    sorted_inputs.push(input);
                }
            }
        }
    }

    Ok(sorted_inputs)
}

// Check if an address is required for unlockig an output in any unlock condition
// Also returns true if the output is an alias or foundry address and the address to search for matches this one
pub(crate) fn output_contains_address(
    output: &Output,
    output_id: OutputId,
    required_address: &Address,
    current_time: u32,
) -> bool {
    // Check alias and nft addresses
    match output {
        Output::Alias(alias_output) => {
            if *required_address
                == Address::Alias(AliasAddress::new(alias_output.alias_id().or_from_output_id(output_id)))
            {
                return true;
            }
        }
        Output::Nft(nft_output) => {
            if *required_address == Address::Nft(NftAddress::new(nft_output.nft_id().or_from_output_id(output_id))) {
                return true;
            }
        }
        _ => {}
    }

    // Check unlock conditions
    if let Some(unlock_conditions) = output.unlock_conditions() {
        if let Some(address_unlock_condition) = unlock_conditions.address() {
            if required_address == unlock_conditions.locked_address(address_unlock_condition.address(), current_time) {
                return true;
            }
        }
        if let Some(state_controller_unlock_condition) = unlock_conditions.state_controller_address() {
            if required_address == state_controller_unlock_condition.address() {
                return true;
            }
        }
        if let Some(governor_controller_unlock_condition) = unlock_conditions.governor_address() {
            if required_address == governor_controller_unlock_condition.address() {
                return true;
            }
        }
        if let Some(immutable_alias_address_unlock_condition) = unlock_conditions.immutable_alias_address() {
            if required_address == immutable_alias_address_unlock_condition.address() {
                return true;
            }
        }
    }

    false
}

pub(crate) fn is_basic_output_address_unlockable(output: &Output, address: &Address, current_time: u32) -> bool {
    match output {
        Output::Basic(_) => {
            if let Some(unlock_conditions) = output.unlock_conditions() {
                if unlock_conditions.is_time_locked(current_time) {
                    return false;
                }

                if let Some(expiration) = unlock_conditions.expiration() {
                    if let Some(expiration_address) = expiration.return_address_expired(current_time) {
                        return expiration_address == address;
                    }
                }

                // PANIC: safe to unwrap as basic outputs always have an address.
                unlock_conditions.address().unwrap().address() == address
            } else {
                // Should not happen anyway as there should always be at least the address unlock condition.
                false
            }
        }
        _ => false,
    }
}
