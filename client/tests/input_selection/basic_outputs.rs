// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::input_selection::new::InputSelection, block::protocol::protocol_parameters, Error};

use crate::input_selection::{build_input_signing_data_most_basic_outputs, build_most_basic_output};

const BECH32_ADDRESS: &str = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";

#[test]
fn input_amount_eq_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_most_basic_output(BECH32_ADDRESS, 1_000_000)];

    let selected = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
}

#[test]
fn input_amount_lt_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_most_basic_output(BECH32_ADDRESS, 2_000_000)];

    assert!(matches!(
        InputSelection::build(outputs, inputs, protocol_parameters.clone())
            .finish()
            .select(),
        Err(Error::NotEnoughBalance {
            found: 1_000_000,
            required: 2_000_000,
        })
    ));
}

#[test]
fn input_amount_gt_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_most_basic_output(BECH32_ADDRESS, 1_000_000)];

    let selected = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
    // One output should be added for the remainder.
    assert_eq!(selected.1.len(), 2);
}

#[test]
fn two_inputs_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs =
        build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000), (BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_most_basic_output(BECH32_ADDRESS, 1_000_000)];

    let selected = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()
        .unwrap();

    // One input has enough amount.
    assert_eq!(selected.0.len(), 1);
    // One output should be added for the remainder.
    assert_eq!(selected.1.len(), 2);
}

#[test]
fn not_enough_storage_deposit_for_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_001)]);
    let outputs = vec![build_most_basic_output(BECH32_ADDRESS, 1_000_000)];

    assert!(matches!(
        InputSelection::build(outputs, inputs.clone(), protocol_parameters)
            .finish()
            .select(),
        Err(Error::BlockError(
            iota_types::block::Error::InsufficientStorageDepositAmount {
                amount: 1,
                required: 213000,
            }
        ))
    ))
}
