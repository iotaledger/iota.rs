// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Helper functions used in the input selection

use iota_types::block::{
    address::{Address, AliasAddress, Ed25519Address, NftAddress},
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, NativeTokens, Output, Rent, RentStructure,
    },
};

use crate::{secret::types::InputSigningData, Result};

// Dedup inputs by output id, because other data could be different, even if it's the same output
// TODO remove ?
// pub(crate) fn dedup_inputs(
//     mandatory_inputs: &mut Vec<InputSigningData>,
//     additional_inputs: &mut Vec<InputSigningData>,
// ) {
//     // Sorting inputs by OutputId so duplicates can be safely removed.
//     mandatory_inputs.sort_by_key(|input| *input.output_metadata.output_id());
//     mandatory_inputs.dedup_by_key(|input| *input.output_metadata.output_id());
//     additional_inputs.sort_by_key(|input| *input.output_metadata.output_id());
//     additional_inputs.dedup_by_key(|input| *input.output_metadata.output_id());

//     // Remove additional inputs that are already mandatory.
//     // TODO: could be done more efficiently with itertools unique?
//     additional_inputs.retain(|input| {
//         !mandatory_inputs
//             .iter()
//             .any(|mandatory_input| input.output_metadata.output_id() == mandatory_input.output_metadata.output_id())
//     });
// }

// Inputs need to be sorted before signing, because the reference unlock conditions can only reference a lower index
pub(crate) fn _sort_input_signing_data(inputs: Vec<InputSigningData>) -> crate::Result<Vec<InputSigningData>> {
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
                        *unlock_address.alias_id() == alias_output.alias_id_non_null(input_signing_data.output_id())
                    } else {
                        false
                    }
                }
                Address::Nft(unlock_address) => {
                    if let Output::Nft(nft_output) = &input_signing_data.output {
                        *unlock_address.nft_id() == nft_output.nft_id_non_null(input_signing_data.output_id())
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
                        alias_output.alias_id_non_null(input.output_id()),
                    ))),
                    Output::Nft(nft_output) => Some(Address::Nft(NftAddress::new(
                        nft_output.nft_id_non_null(input.output_id()),
                    ))),
                    _ => None,
                };

                if let Some(alias_or_nft_address) = alias_or_nft_address {
                    // Check for existing outputs for this address, and insert before
                    match sorted_inputs.iter().position(|input_signing_data| {
                        Address::try_from_bech32(&input_signing_data.bech32_address)
                            .expect("safe to unwrap, we encoded it before")
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

/// Computes the minimum storage deposit amount that a basic output needs to have with an [AddressUnlockCondition] and
/// optional [NativeTokens].
pub fn minimum_storage_deposit_basic_output(
    config: &RentStructure,
    native_tokens: &Option<NativeTokens>,
    token_supply: u64,
) -> Result<u64> {
    // Null address because we only care about the size and ed25519, alias and nft addresses have the same size.
    let address_condition = UnlockCondition::Address(AddressUnlockCondition::new(Address::from(Ed25519Address::from(
        [0; Ed25519Address::LENGTH],
    ))));
    let mut basic_output_builder = BasicOutputBuilder::new_with_amount(Output::AMOUNT_MIN)?;
    if let Some(native_tokens) = native_tokens {
        basic_output_builder = basic_output_builder.with_native_tokens(native_tokens.clone());
    }
    let basic_output = basic_output_builder
        .add_unlock_condition(address_condition)
        .finish_output(token_supply)?;

    Ok(basic_output.rent_cost(config))
}
