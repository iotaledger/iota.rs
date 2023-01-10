// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::input_selection::new::InputSelection, block::protocol::protocol_parameters};

use crate::input_selection::{
    build_basic_output, build_input_signing_data_most_basic_outputs, BECH32_ADDRESS, TOKEN_ID_1, TOKEN_ID_2,
};

#[test]
fn input_amount_equal_output_amount() {
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

    assert_eq!(selected.0, inputs);
    // assert_eq!(selected.1, outputs);
}

// expected selected: [
// basic{ amount: 1_000_000, native_tokens: [{‘a’: 100}] },
// basic{ amount: 1_000_000, native_tokens: [{‘a’: 100}, {‘b’: 100}] }]
// expected remainder: Some(basic{ amount: 1_000_000, native_tokens: [{‘a’: 50}, {‘b’: 100}] })
