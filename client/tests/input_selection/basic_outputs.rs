// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{InputSelection, Requirement},
    block::{
        address::Address,
        output::{AliasId, NftId},
        protocol::protocol_parameters,
    },
    Error,
};

use crate::input_selection::{
    build_basic_output, build_input_signing_data_alias_outputs, build_input_signing_data_most_basic_outputs,
    build_input_signing_data_nft_outputs, ALIAS_ID_1, BECH32_ADDRESS, BECH32_ADDRESS_ALIAS_SENDER,
    BECH32_ADDRESS_ED25519_SENDER, BECH32_ADDRESS_NFT_SENDER, BECH32_ADDRESS_REMAINDER, NFT_ID_1,
};

#[test]
fn input_amount_equal_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000, None)]);
    let outputs = vec![build_basic_output(1_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
    assert_eq!(selected.1, outputs);
}

#[test]
fn input_amount_lower_than_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000, None)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientBaseTokenAmount {
            found: 1_000_000,
            required: 2_000_000,
        })
    ));
}

#[test]
fn input_amount_greater_than_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000, None)]);
    let outputs = vec![build_basic_output(500_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
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

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000, None)]);
    let outputs = vec![build_basic_output(500_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .remainder_address(remainder_address)
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

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]);
    let outputs = vec![build_basic_output(500_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs, outputs.clone(), protocol_parameters)
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
fn two_inputs_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 1_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]);
    let outputs = vec![build_basic_output(1_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0, vec![inputs[0].clone()]);
    assert_eq!(selected.1, outputs);
}

#[test]
fn two_inputs_one_needed_reversed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS, 1_000_000, None),
    ]);
    let outputs = vec![build_basic_output(1_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0, vec![inputs[1].clone()]);
    assert_eq!(selected.1, outputs);
}

#[test]
fn two_inputs_both_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 1_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]);
    let outputs = vec![build_basic_output(3_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
    assert_eq!(selected.1, outputs);
}

#[test]
fn two_inputs_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 1_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]);
    let outputs = vec![build_basic_output(2_500_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
    // One output should be added for the remainder.
    assert_eq!(selected.1.len(), 2);
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 500_000);
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

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_001, None)]);
    let outputs = vec![build_basic_output(1_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

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
fn ed25519_sender() {
    let protocol_parameters = protocol_parameters();
    let sender = Address::try_from_bech32(BECH32_ADDRESS_ED25519_SENDER).unwrap().1;

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS_ED25519_SENDER, 1_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]);
    let outputs = vec![build_basic_output(
        2_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ED25519_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .select()
        .unwrap();

    // Sender + another for amount
    assert_eq!(selected.0.len(), 2);
    assert!(
        selected
            .0
            .iter()
            .any(|input| *input.output.as_basic().address() == sender)
    );
    // Provided output + remainder
    assert_eq!(selected.1.len(), 2);
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 5_000_000, None)]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ED25519_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_ed25519() && sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_SENDER).unwrap().1
    ))
}

#[test]
fn alias_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let mut inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]);
    inputs.extend(build_input_signing_data_alias_outputs(vec![(
        alias_id_1,
        BECH32_ADDRESS,
        1_000_000,
        None,
    )]));
    inputs.extend(build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]));
    let outputs = vec![build_basic_output(
        2_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
    )];

    let selected = InputSelection::new(inputs.clone(), outputs, protocol_parameters)
        .select()
        .unwrap();

    // Sender + another for amount
    assert_eq!(selected.0.len(), 2);
    assert!(
        selected
            .0
            .iter()
            .any(|input| input.output.is_alias() && *input.output.as_alias().alias_id() == alias_id_1)
    );
    // Provided output + alias
    assert_eq!(selected.1.len(), 2);
}

#[test]
fn missing_alias_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 5_000_000, None)]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_alias() && sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_SENDER).unwrap().1
    ))
}

#[test]
fn nft_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let mut inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]);
    inputs.extend(build_input_signing_data_nft_outputs(vec![(
        nft_id_1,
        BECH32_ADDRESS,
        1_000_000,
        None,
    )]));
    inputs.extend(build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 2_000_000, None),
        (BECH32_ADDRESS, 2_000_000, None),
    ]));
    let outputs = vec![build_basic_output(
        2_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_NFT_SENDER),
    )];

    let selected = InputSelection::new(inputs.clone(), outputs, protocol_parameters)
        .select()
        .unwrap();

    // Sender + another for amount
    assert_eq!(selected.0.len(), 2);
    assert!(
        selected
            .0
            .iter()
            .any(|input| input.output.is_nft() && *input.output.as_nft().nft_id() == nft_id_1)
    );
    // Provided output + nft
    assert_eq!(selected.1.len(), 2);
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 5_000_000, None)]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_NFT_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_nft() && sender == Address::try_from_bech32(BECH32_ADDRESS_NFT_SENDER).unwrap().1
    ))
}
