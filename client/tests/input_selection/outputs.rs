// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::input_selection::new::InputSelection, block::protocol::protocol_parameters, Error};

use crate::input_selection::{build_inputs, build_outputs, Build::Basic, BECH32_ADDRESS};

#[test]
fn no_inputs_provided() {
    let protocol_parameters = protocol_parameters();

    let inputs = Vec::new();
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(selected, Err(Error::NoInputsProvided)));
}

#[test]
fn no_outputs_provided() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = Vec::new();

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(selected, Err(Error::NoOutputsProvided)));
}
