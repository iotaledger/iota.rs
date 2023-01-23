// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::input_selection::InputSelection, block::protocol::protocol_parameters, Error};

use crate::input_selection::{
    addresses, build_inputs, build_outputs, unsorted_eq, Build::Basic, BECH32_ADDRESS_ED25519_0,
    BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_ED25519_2,
};

#[test]
fn one_output_expiration_not_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 200)),
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn expiration_equal_timestamp() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 200)),
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(200)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn one_output_expiration_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 50)),
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_expiration_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_0, 200)),
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_0, 50)),
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_expiration_expired_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_0, 200)),
        ),
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_two_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_0, 100)),
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 100)),
        ),
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_2]),
        protocol_parameters,
    )
    .timestamp(200)
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_two_expired_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 100)),
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 100)),
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        4_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_ED25519_2]),
        protocol_parameters,
    )
    .timestamp(200)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        None,
        Some((BECH32_ADDRESS_ED25519_0, 50)),
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        None,
        Some((BECH32_ADDRESS_ED25519_0, 50)),
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_and_timelock() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 1_000_000)),
        Some(50),
        Some((BECH32_ADDRESS_ED25519_0, 50)),
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_and_timelock_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        Some(50),
        Some((BECH32_ADDRESS_ED25519_0, 50)),
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}
