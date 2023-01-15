// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection},
    block::{address::Address, output::AliasId, protocol::protocol_parameters},
    Error,
};

use crate::input_selection::{
    build_inputs, build_outputs,
    Build::{Alias, Basic},
    ALIAS_ID_2, BECH32_ADDRESS,
};

#[test]
fn no_inputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = Vec::new();
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(selected, Err(Error::NoInputsProvided)));
}

#[test]
fn no_outputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = Vec::new();

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(selected, Err(Error::NoOutputsProvided)));
}

#[test]
fn no_outputs_but_burn() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(2_000_000, alias_id_2, BECH32_ADDRESS, None, None, None)]);
    let outputs = Vec::new();

    let selected = InputSelection::new(inputs.clone(), outputs, protocol_parameters)
        .burn(Burn::new().add_alias(alias_id_2))
        .select()
        .unwrap();

    assert_eq!(selected.inputs, inputs);
    assert_eq!(selected.outputs.len(), 1);
    assert!(selected.outputs[0].is_basic());
    assert_eq!(selected.outputs[0].amount(), 2_000_000);
    assert_eq!(selected.outputs[0].as_basic().native_tokens().len(), 0);
    assert_eq!(selected.outputs[0].as_basic().unlock_conditions().len(), 1);
    assert_eq!(selected.outputs[0].as_basic().features().len(), 0);
    assert_eq!(
        *selected.outputs[0].as_basic().address(),
        Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
    );
}
