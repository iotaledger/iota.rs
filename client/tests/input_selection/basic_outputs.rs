// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    api::input_selection::new::{InputSelection, Requirement},
    block::{address::Address, protocol::protocol_parameters},
    Error,
};

use crate::input_selection::{
    build_basic_output, build_input_signing_data_most_basic_outputs, BECH32_ADDRESS, BECH32_ADDRESS_ALIAS_SENDER,
    BECH32_ADDRESS_ED25519_SENDER, BECH32_ADDRESS_NFT_SENDER, BECH32_ADDRESS_REMAINDER,
};

#[test]
fn input_amount_equal_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_basic_output(1_000_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(outputs.clone(), inputs.clone(), protocol_parameters)
        .finish()
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
    assert_eq!(selected.1, outputs);
}

#[test]
fn input_amount_lower_than_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(outputs, inputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::NotEnoughBalance {
            found: 1_000_000,
            required: 2_000_000,
        })
    ));
}

#[test]
fn input_amount_greater_than_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_basic_output(500_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(outputs.clone(), inputs.clone(), protocol_parameters)
        .finish()
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
    // One output should be added for the remainder.
    assert_eq!(selected.1.len(), 2);
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_500_000);
            assert_eq!(output.as_basic().native_tokens().len(), 0);
            assert_eq!(output.as_basic().unlock_conditions().len(), 1);
            assert_eq!(output.as_basic().features().len(), 0);
            assert_eq!(
                *output.as_basic().address(),
                Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
            );
        }
    });
}

#[test]
fn input_amount_greater_than_output_amount_with_remainder_address() {
    let protocol_parameters = protocol_parameters();
    let remainder_address = Address::try_from_bech32(BECH32_ADDRESS_REMAINDER).unwrap().1;

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_basic_output(500_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(outputs.clone(), inputs.clone(), protocol_parameters)
        .remainder_address(remainder_address)
        .finish()
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
    // One output should be added for the remainder.
    assert_eq!(selected.1.len(), 2);
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_500_000);
            assert_eq!(output.as_basic().native_tokens().len(), 0);
            assert_eq!(output.as_basic().unlock_conditions().len(), 1);
            assert_eq!(output.as_basic().features().len(), 0);
            assert_eq!(*output.as_basic().address(), remainder_address);
        }
    });
}

#[test]
fn two_same_inputs_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs =
        build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000), (BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_basic_output(500_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(outputs.clone(), inputs, protocol_parameters)
        .finish()
        .select()
        .unwrap();

    // One input has enough amount.
    assert_eq!(selected.0.len(), 1);
    // One output should be added for the remainder.
    assert_eq!(selected.1.len(), 2);
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_500_000);
            assert_eq!(output.as_basic().native_tokens().len(), 0);
            assert_eq!(output.as_basic().unlock_conditions().len(), 1);
            assert_eq!(output.as_basic().features().len(), 0);
            assert_eq!(
                *output.as_basic().address(),
                Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
            );
        }
    });
}

#[test]
fn not_enough_storage_deposit_for_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_001)]);
    let outputs = vec![build_basic_output(1_000_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(outputs, inputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::BlockError(
            iota_types::block::Error::InsufficientStorageDepositAmount {
                amount: 1,
                required: 213000,
            }
        ))
    ))
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 5_000_000)]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        Some(BECH32_ADDRESS_ED25519_SENDER),
    )];

    let selected = InputSelection::build(outputs, inputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_ed25519() && sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_SENDER).unwrap().1
    ))
}

#[test]
fn missing_alias_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 5_000_000)]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
    )];

    let selected = InputSelection::build(outputs, inputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_alias() && sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_SENDER).unwrap().1
    ))
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 5_000_000)]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        Some(BECH32_ADDRESS_NFT_SENDER),
    )];

    let selected = InputSelection::build(outputs, inputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_nft() && sender == Address::try_from_bech32(BECH32_ADDRESS_NFT_SENDER).unwrap().1
    ))
}
