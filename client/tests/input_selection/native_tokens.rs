// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::InputSelection,
    block::{address::Address, output::TokenId, protocol::protocol_parameters},
};
use primitive_types::U256;

use crate::input_selection::{
    build_basic_output, build_input_signing_data_most_basic_outputs, unsorted_eq, BECH32_ADDRESS, TOKEN_ID_1,
    TOKEN_ID_2,
};

#[test]
fn two_native_tokens_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 1_000_000, Some(vec![(TOKEN_ID_1, 100)])),
        (
            BECH32_ADDRESS,
            1_000_000,
            Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
        ),
    ]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        Some(vec![(TOKEN_ID_1, 150)]),
        None,
    )];

    let selected = InputSelection::build(inputs.clone(), outputs.clone(), protocol_parameters)
        .finish()
        .unwrap()
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.0, &inputs));
    assert_eq!(selected.1.len(), 2);
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_000_000);
            assert_eq!(output.as_basic().native_tokens().len(), 2);
            assert_eq!(
                output
                    .as_basic()
                    .native_tokens()
                    .get(&TokenId::from_str(TOKEN_ID_1).unwrap())
                    .unwrap()
                    .amount(),
                U256::from(50),
            );
            assert_eq!(
                output
                    .as_basic()
                    .native_tokens()
                    .get(&TokenId::from_str(TOKEN_ID_2).unwrap())
                    .unwrap()
                    .amount(),
                U256::from(100),
            );
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
fn two_native_tokens_both_needed_plus_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 1_000_000, Some(vec![(TOKEN_ID_1, 100)])),
        (
            BECH32_ADDRESS,
            1_000_000,
            Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
        ),
    ]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        Some(vec![(TOKEN_ID_1, 150), (TOKEN_ID_2, 100)]),
        None,
    )];

    let selected = InputSelection::build(inputs.clone(), outputs.clone(), protocol_parameters)
        .finish()
        .unwrap()
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.0, &inputs));
    assert_eq!(selected.1.len(), 2);
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_000_000);
            assert_eq!(output.as_basic().native_tokens().len(), 1);
            assert_eq!(
                output
                    .as_basic()
                    .native_tokens()
                    .get(&TokenId::from_str(TOKEN_ID_1).unwrap())
                    .unwrap()
                    .amount(),
                U256::from(50),
            );
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
fn three_inputs_two_needed_plus_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 1_000_000, Some(vec![(TOKEN_ID_1, 100)])),
        (BECH32_ADDRESS, 1_000_000, Some(vec![(TOKEN_ID_1, 100)])),
        (BECH32_ADDRESS, 1_000_000, Some(vec![(TOKEN_ID_1, 100)])),
    ]);
    let outputs = vec![build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        Some(vec![(TOKEN_ID_1, 120)]),
        None,
    )];

    let selected = InputSelection::build(inputs.clone(), outputs.clone(), protocol_parameters)
        .finish()
        .unwrap()
        .select()
        .unwrap();

    assert_eq!(selected.0.len(), 2);
    assert_eq!(selected.1.len(), 2);
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_000_000);
            assert_eq!(output.as_basic().native_tokens().len(), 1);
            assert_eq!(
                output
                    .as_basic()
                    .native_tokens()
                    .get(&TokenId::from_str(TOKEN_ID_1).unwrap())
                    .unwrap()
                    .amount(),
                U256::from(80),
            );
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
fn three_inputs_two_needed_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![
        (BECH32_ADDRESS, 1_000_000, Some(vec![(TOKEN_ID_1, 100)])),
        (BECH32_ADDRESS, 1_000_000, Some(vec![(TOKEN_ID_1, 100)])),
        (BECH32_ADDRESS, 1_000_000, Some(vec![(TOKEN_ID_1, 100)])),
    ]);
    let outputs = vec![build_basic_output(
        2_000_000,
        BECH32_ADDRESS,
        Some(vec![(TOKEN_ID_1, 200)]),
        None,
    )];

    let selected = InputSelection::build(inputs.clone(), outputs.clone(), protocol_parameters)
        .finish()
        .unwrap()
        .select()
        .unwrap();

    assert_eq!(selected.0.len(), 2);
    assert_eq!(selected.1, outputs);
}
