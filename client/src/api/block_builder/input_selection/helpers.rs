// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Helper functions used in the input selection

use iota_types::block::{
    address::{Address, Ed25519Address},
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, NativeTokens, Output, Rent, RentStructure,
    },
};

use crate::Result;

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
