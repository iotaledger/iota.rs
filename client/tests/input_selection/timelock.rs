// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::input_selection::InputSelection, block::protocol::protocol_parameters, Error};

use crate::input_selection::{
    build_inputs, build_outputs, unsorted_eq, Build::Basic, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1,
};

#[test]
fn one_output_timelock_not_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        Some(200),
    )]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_1, None, None, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .timestamp(100)
        .select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn timelock_equal_timestamp() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        Some(200),
    )]);
    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS_ED25519_1, None, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .timestamp(200)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_timelock_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            Some(200),
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            Some(50),
        ),
    ]);
    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS_ED25519_1, None, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .timestamp(100)
        .select()
        .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_timelock_expired_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            Some(200),
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS_ED25519_1, None, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .timestamp(100)
        .select()
        .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn one_output_timelock_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        Some(50),
    )]);
    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS_ED25519_1, None, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .timestamp(100)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}
